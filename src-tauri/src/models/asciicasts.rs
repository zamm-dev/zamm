use crate::commands::errors::ZammResult;
use crate::models::llm_calls::EntityId;
use crate::models::os::OS;
use crate::schema::asciicasts;
use anyhow::anyhow;
use asciicast::{Entry, Header};
use chrono::naive::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct AsciiCastData {
    pub header: Header,
    pub entries: Vec<Entry>,
}

impl Default for AsciiCastData {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiCastData {
    pub fn new() -> Self {
        Self {
            header: Header {
                version: 2,
                width: 80,
                height: 24,
                timestamp: None,
                duration: None,
                idle_time_limit: None,
                command: None,
                title: None,
                env: None,
            },
            entries: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn load(file: &str) -> ZammResult<Self> {
        let contents = std::fs::read_to_string(file)?;
        AsciiCastData::parse(&contents)
    }

    #[cfg(test)]
    pub fn save(&self, file: &str) -> ZammResult<()> {
        let contents = format!("{}", self);
        std::fs::write(file, contents)?;
        Ok(())
    }

    pub fn parse(contents: &str) -> ZammResult<Self> {
        let mut lines = contents.lines();
        let header_str = lines.next().ok_or(anyhow!("Empty cast"))?;
        let header: Header = serde_json::from_str(header_str)?;
        let entries = lines
            .map(serde_json::from_str)
            .collect::<Result<Vec<Entry>, _>>()?;
        Ok(Self { header, entries })
    }
}

impl serde::Serialize for AsciiCastData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

struct AsciiCastDataVisitor;

impl<'de> serde::de::Visitor<'de> for AsciiCastDataVisitor {
    type Value = AsciiCastData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an asciicast string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        AsciiCastData::parse(value).map_err(serde::de::Error::custom)
    }
}

impl<'de> serde::Deserialize<'de> for AsciiCastData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(AsciiCastDataVisitor)
    }
}

impl Display for AsciiCastData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = serde_json::to_string(&self.header).map_err(|_| fmt::Error)?;
        let entries = self
            .entries
            .iter()
            .map(|entry| serde_json::to_string(entry).map_err(|_| fmt::Error))
            .collect::<Result<Vec<String>, _>>()?;
        write!(f, "{}\n{}", header, entries.join("\n"))
    }
}

#[derive(Queryable, Selectable, Debug, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = asciicasts)]
pub struct AsciiCast {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub command: String,
    pub os: Option<OS>,
    pub cast: AsciiCastData,
}

impl AsciiCast {
    pub fn as_insertable(&self) -> NewAsciiCast {
        NewAsciiCast {
            id: &self.id,
            timestamp: &self.timestamp,
            command: &self.command,
            os: self.os,
            cast: &self.cast,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = asciicasts)]
pub struct NewAsciiCast<'a> {
    pub id: &'a EntityId,
    pub timestamp: &'a NaiveDateTime,
    pub command: &'a str,
    pub os: Option<OS>,
    pub cast: &'a AsciiCastData,
}

impl ToSql<Text, Sqlite> for AsciiCastData
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = format!("{}", self);
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for AsciiCastData
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let json_str = String::from_sql(bytes)?;
        AsciiCastData::parse(&json_str).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::database::setup_database;
    use asciicast::{Entry, EventType, Header};
    use chrono::{DateTime, SubsecRound};
    use uuid::Uuid;

    const TEST_CAST_STR: &str = r#""{\"version\":2,\"width\":80,\"height\":24,\"timestamp\":1731159783}\n[0.0,\"i\",\"echo hello\"]\n[1.0,\"o\",\"hello\"]""#;

    fn get_test_ascii_cast() -> AsciiCastData {
        AsciiCastData {
            header: Header {
                version: 2,
                width: 80,
                height: 24,
                // `asciicast` crate has problems parsing non-existent time
                timestamp: DateTime::from_timestamp(1731159783, 0),
                duration: None,
                idle_time_limit: None,
                command: None,
                title: None,
                env: None,
            },
            entries: vec![
                Entry {
                    time: 0.0,
                    event_type: EventType::Input,
                    event_data: "echo hello".to_string(),
                },
                Entry {
                    time: 1.0,
                    event_type: EventType::Output,
                    event_data: "hello".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_serialize_ascii_cast_data() {
        let serialized = serde_json::to_string(&get_test_ascii_cast()).unwrap();
        assert_eq!(serialized, TEST_CAST_STR);
    }

    #[test]
    fn test_deserialize_ascii_cast_data() {
        let deserialized: AsciiCastData = serde_json::from_str(TEST_CAST_STR).unwrap();
        assert_eq!(deserialized, get_test_ascii_cast());
    }

    #[test]
    fn test_ascii_cast_round_trip() {
        let mut conn = setup_database(None);
        let timestamp = chrono::Utc::now().trunc_subsecs(0);

        let header = Header {
            version: 2,
            width: 80,
            height: 24,
            timestamp: Some(timestamp),
            duration: Some(1.0),
            idle_time_limit: None,
            command: Some("echo hello".to_string()),
            title: None,
            env: None,
        };
        let entries = vec![
            Entry {
                time: 0.0,
                event_type: EventType::Input,
                event_data: "echo hello".to_string(),
            },
            Entry {
                time: 1.0,
                event_type: EventType::Output,
                event_data: "hello".to_string(),
            },
        ];
        let cast = AsciiCastData { header, entries };

        let row = AsciiCast {
            id: EntityId {
                uuid: Uuid::new_v4(),
            },
            timestamp: timestamp.naive_utc(),
            command: "echo hello".to_string(),
            os: Some(OS::Mac),
            cast: cast.clone(),
        };

        diesel::insert_into(asciicasts::table)
            .values(&row.as_insertable())
            .execute(&mut conn)
            .unwrap();

        let result = asciicasts::table.first::<AsciiCast>(&mut conn).unwrap();

        assert_eq!(result.id, row.id);
        assert_eq!(result.timestamp, row.timestamp);
        assert_eq!(result.command, row.command);
        assert_eq!(result.os, row.os);
        assert_eq!(result.cast, row.cast);
    }
}

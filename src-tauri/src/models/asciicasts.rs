use crate::models::llm_calls::EntityId;
use crate::models::os::OS;
use crate::schema::asciicasts;
use anyhow::anyhow;
use asciicast::{Entry, Header};
use chrono::naive::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AsciiCastData {
    pub header: Header,
    pub entries: Vec<Entry>,
}

#[derive(Queryable, Selectable, Debug, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = asciicasts)]
pub struct AsciiCast {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub command: String,
    pub os: Option<OS>,
    pub cast: String,
}

impl AsciiCast {
    #[allow(dead_code)]
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
    pub cast: &'a str,
}

impl ToSql<Text, Sqlite> for AsciiCastData
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let header_str = serde_json::to_string(&self.header)?;
        let entries_str = self
            .entries
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<String>, _>>()?;
        let json_str = format!("{}\n{}", header_str, entries_str.join("\n"));
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
        let mut lines = json_str.lines();
        let header_str = lines.next().ok_or(anyhow!("Empty cast"))?;
        let header: Header = serde_json::from_str(header_str)?;
        let entries = lines
            .map(serde_json::from_str)
            .collect::<Result<Vec<Entry>, _>>()?;
        Ok(AsciiCastData { header, entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::database::setup_database;
    use asciicast::{Entry, EventType, Header};
    use uuid::Uuid;

    #[test]
    fn test_ascii_cast_round_trip() {
        let mut conn = setup_database(None);

        let header = Header {
            version: 2,
            width: 80,
            height: 24,
            timestamp: None,
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
            timestamp: chrono::Utc::now().naive_utc(),
            command: "echo hello".to_string(),
            os: Some(OS::Mac),
            cast: serde_json::to_string(&cast).unwrap(),
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

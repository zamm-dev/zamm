use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

#[derive(
    AsExpression,
    FromSqlRow,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    specta::Type,
    Serialize,
    Deserialize,
)]
#[diesel(sql_type = Text)]
#[serde(transparent)]
pub struct EntityId {
    pub uuid: Uuid,
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityId {
    pub fn new() -> Self {
        EntityId {
            uuid: Uuid::new_v4(),
        }
    }
}

impl Deref for EntityId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl From<Uuid> for EntityId {
    fn from(uuid: Uuid) -> Self {
        EntityId { uuid }
    }
}

impl TryFrom<&str> for EntityId {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(value)?;
        Ok(EntityId { uuid })
    }
}

impl ToSql<Text, Sqlite> for EntityId
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let uuid_str = self.uuid.to_string();
        out.set_value(uuid_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for EntityId
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let uuid_str = String::from_sql(bytes)?;
        let parsed_uuid = Uuid::parse_str(&uuid_str)?;
        Ok(EntityId { uuid: parsed_uuid })
    }
}

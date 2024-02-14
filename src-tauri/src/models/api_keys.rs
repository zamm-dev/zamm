use crate::schema::api_keys;
use crate::setup::api_keys::Service;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::str::FromStr;

#[derive(Queryable, Selectable, Debug)]
pub struct ApiKey {
    pub service: Service,
    pub api_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey<'a> {
    pub service: Service,
    pub api_key: &'a str,
}

impl ToSql<Text, Sqlite> for Service
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let service_str = self.to_string();
        out.set_value(service_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Service
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let service_str = String::from_sql(bytes)?;
        let parsed_service = Service::from_str(&service_str)?;
        Ok(parsed_service)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::db::MIGRATIONS;

    use diesel_migrations::MigrationHarness;

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    #[test]
    fn test_uuid_serialization_and_deserialization() {
        let mut conn = setup_database();
        let dummy_api_key = "0p3n41-4p1-k3y";

        let openai_api_key = NewApiKey {
            service: Service::OpenAI,
            api_key: dummy_api_key,
        };

        // Insert
        diesel::insert_into(api_keys::table)
            .values(&openai_api_key)
            .execute(&mut conn)
            .unwrap();

        // Query
        let results: Vec<ApiKey> = api_keys::table.load(&mut conn).unwrap();
        assert_eq!(results.len(), 1);

        let retrieved_api_key = &results[0];
        assert_eq!(retrieved_api_key.service, Service::OpenAI);
        assert_eq!(retrieved_api_key.api_key.as_str(), dummy_api_key);
    }
}

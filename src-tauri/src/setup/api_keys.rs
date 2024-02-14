use crate::models::ApiKey;
use crate::schema::api_keys;
use diesel;
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::env;
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Type,
    EnumString,
    Display,
    AsExpression,
    FromSqlRow,
)]
#[diesel(sql_type = Text)]
#[strum(serialize_all = "snake_case")]
pub enum Service {
    OpenAI,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct ApiKeys {
    pub openai: Option<String>,
}

impl ApiKeys {
    pub fn update(&mut self, service: &Service, key: String) {
        match service {
            Service::OpenAI => self.openai = Some(key),
        }
    }

    pub fn remove(&mut self, service: &Service) {
        match service {
            Service::OpenAI => self.openai = None,
        }
    }
}

pub fn setup_api_keys(possible_db: &mut Option<SqliteConnection>) -> ApiKeys {
    let mut api_keys = ApiKeys { openai: None };

    if let Some(conn) = possible_db.as_mut() {
        let load_result: Result<Vec<ApiKey>, diesel::result::Error> =
            api_keys::table.load(conn);
        if let Ok(api_keys_rows) = load_result {
            for api_key in api_keys_rows {
                api_keys.update(&api_key.service, api_key.api_key);
            }
        }
    }

    // database keys will get overridden by environment keys
    if let Ok(openai_api_key) = env::var("OPENAI_API_KEY") {
        api_keys.openai = Some(openai_api_key);
    }

    api_keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NewApiKey;
    use crate::setup::db::MIGRATIONS;
    use diesel_migrations::MigrationHarness;
    use temp_env;

    const DUMMY_API_KEY: &str = "0p3n41-4p1-k3y";

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    #[test]
    fn test_get_empty_api_keys_no_db() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let api_keys = setup_api_keys(&mut None);
            assert!(api_keys.openai.is_none());
        });
    }

    #[test]
    fn test_get_present_api_keys_no_db() {
        temp_env::with_var("OPENAI_API_KEY", Some(DUMMY_API_KEY), || {
            let api_keys = setup_api_keys(&mut None);
            assert_eq!(api_keys.openai, Some(DUMMY_API_KEY.to_string()));
        });
    }

    #[test]
    fn test_get_api_keys_from_db() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let mut conn = setup_database();
            diesel::insert_into(api_keys::table)
                .values(&NewApiKey {
                    service: Service::OpenAI,
                    api_key: DUMMY_API_KEY,
                })
                .execute(&mut conn)
                .unwrap();

            let api_keys = setup_api_keys(&mut Some(conn));
            assert_eq!(api_keys.openai, Some(DUMMY_API_KEY.to_string()));
        });
    }

    #[test]
    fn test_env_key_overrides_db_key() {
        let custom_api_key = "c0st0m-4p1-k3y";

        temp_env::with_var("OPENAI_API_KEY", Some(custom_api_key.to_string()), || {
            let mut conn = setup_database();
            diesel::insert_into(api_keys::table)
                .values(&NewApiKey {
                    service: Service::OpenAI,
                    api_key: DUMMY_API_KEY,
                })
                .execute(&mut conn)
                .unwrap();

            let api_keys = setup_api_keys(&mut Some(conn));
            assert_eq!(api_keys.openai, Some(custom_api_key.to_string()));
        });
    }

    #[test]
    fn test_empty_db_doesnt_crash() {
        temp_env::with_var("OPENAI_API_KEY", None::<String>, || {
            let conn = setup_database();

            let api_keys = setup_api_keys(&mut Some(conn));
            assert_eq!(api_keys.openai, None);
        });
    }
}

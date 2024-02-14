use crate::commands::errors::ZammResult;
use crate::schema::api_keys;
use crate::setup::api_keys::Service;
use crate::{ZammApiKeys, ZammDatabase};
use diesel::{ExpressionMethods, RunQueryDsl};
use specta::specta;
use tauri::State;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

async fn set_api_key_helper(
    zamm_api_keys: &ZammApiKeys,
    zamm_db: &ZammDatabase,
    filename: Option<&str>,
    service: &Service,
    api_key: String,
) -> ZammResult<()> {
    let api_keys = &mut zamm_api_keys.0.lock().await;
    let db = &mut zamm_db.0.lock().await;

    // write new API key to disk before we can no longer borrow it
    let init_update_result = || -> ZammResult<()> {
        if api_key.is_empty() {
            return Ok(());
        }

        if let Some(untrimmed_filename) = filename {
            let f = untrimmed_filename.trim();
            if !f.is_empty() {
                let ends_in_newline = {
                    if Path::new(f).exists() {
                        let mut file = OpenOptions::new().read(true).open(f)?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;
                        contents.ends_with('\n')
                    } else {
                        true // no need to start the file with a newline later
                    }
                };

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(f)?;
                if !ends_in_newline {
                    writeln!(file)?;
                }
                writeln!(file, "export OPENAI_API_KEY=\"{}\"", api_key)?;
            }
        }
        Ok(())
    }();

    // write new API key to database before we can no longer borrow it
    let db_update_result = || -> ZammResult<()> {
        if let Some(conn) = db.as_mut() {
            if api_key.is_empty() {
                // delete from db
                diesel::delete(api_keys::table)
                    .filter(api_keys::service.eq(service))
                    .execute(conn)?;
            } else {
                diesel::replace_into(api_keys::table)
                    .values(crate::models::NewApiKey {
                        service: service.clone(),
                        api_key: &api_key,
                    })
                    .execute(conn)?;
            }
        }
        Ok(())
    }();

    // assign ownership of new API key string to in-memory API keys
    if api_key.is_empty() {
        api_keys.remove(service);
    } else {
        api_keys.update(service, api_key);
    }

    // if any errors exist, return one of them
    init_update_result?;
    db_update_result
}

#[tauri::command(async)]
#[specta]
pub async fn set_api_key(
    api_keys: State<'_, ZammApiKeys>,
    database: State<'_, ZammDatabase>,
    filename: Option<&str>,
    service: Service,
    api_key: String,
) -> ZammResult<()> {
    set_api_key_helper(&api_keys, &database, filename, &service, api_key).await
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::NewApiKey;
    use crate::sample_call::SampleCall;
    use crate::schema;
    use crate::setup::api_keys::ApiKeys;
    use crate::setup::db::MIGRATIONS;
    use crate::test_helpers::get_temp_test_dir;
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::{Path, PathBuf};
    use tokio::sync::Mutex;

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    pub fn setup_zamm_db() -> ZammDatabase {
        ZammDatabase(Mutex::new(Some(setup_database())))
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SetApiKeyRequest {
        filename: Option<String>,
        service: Service,
        api_key: String,
    }

    fn parse_request(request_str: &str) -> SetApiKeyRequest {
        serde_json::from_str(request_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    async fn get_openai_api_key_from_db(db: &ZammDatabase) -> Option<String> {
        use schema::api_keys::dsl::*;
        let mut conn_mutex = db.0.lock().await;
        let conn = conn_mutex.as_mut().unwrap();
        api_keys
            .filter(service.eq(Service::OpenAI))
            .select(api_key)
            .first::<String>(conn)
            .ok()
    }

    pub async fn check_set_api_key_sample(
        db: &ZammDatabase,
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
        test_dir_name: &str,
    ) {
        let sample = read_sample(sample_file);
        assert_eq!(sample.request.len(), 2);
        assert_eq!(sample.request[0], "set_api_key");

        let request = parse_request(&sample.request[1]);
        let valid_request_path_specified = request
            .filename
            .as_ref()
            .map(|f| !f.is_empty() && !f.ends_with('/'))
            .unwrap_or(false);
        let request_path = request.filename.as_ref().map(|f| PathBuf::from(&f));
        let test_init_file = if valid_request_path_specified {
            let p = request_path.as_ref().unwrap();
            let sample_file_directory = p.parent().unwrap().to_str().unwrap();
            let test_name = format!("{}/{}", test_dir_name, sample_file_directory);
            let temp_init_dir = get_temp_test_dir(&test_name);
            let init_file = temp_init_dir.join(p.file_name().unwrap());
            println!(
                "Test will be performed on shell init file at {}",
                init_file.display()
            );

            let starting_init_file = Path::new("api/sample-init-files").join(p);
            if PathBuf::from(&starting_init_file).exists() {
                fs::copy(&starting_init_file, &init_file).unwrap();
            }

            Some(init_file.to_str().unwrap().to_owned())
        } else {
            request.filename
        };

        let actual_result = set_api_key_helper(
            existing_zamm_api_keys,
            db,
            test_init_file.as_deref(),
            &request.service,
            request.api_key.clone(),
        )
        .await;

        // check that the API call returns the expected success or failure signal
        if sample.response.success == Some(false) {
            assert!(actual_result.is_err(), "API call should have thrown error");
        } else {
            assert!(
                actual_result.is_ok(),
                "API call failed: {:?}",
                actual_result
            );
        }

        // check that the API call returns the expected JSON
        let actual_json = match actual_result {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap(),
            Err(e) => serde_json::to_string_pretty(&e).unwrap(),
        };
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);

        // check that the API call actually modified the in-memory API keys,
        // regardless of success or failure
        let existing_api_keys = existing_zamm_api_keys.0.lock().await;
        if request.api_key.is_empty() {
            assert_eq!(existing_api_keys.openai, None);
            assert_eq!(get_openai_api_key_from_db(db).await, None);
        } else {
            assert_eq!(existing_api_keys.openai, Some(request.api_key.clone()));
            assert_eq!(
                get_openai_api_key_from_db(db).await,
                Some(request.api_key.clone())
            );
        }

        // check that the API call successfully wrote the API keys to disk, if asked to
        if valid_request_path_specified {
            let p = request_path.unwrap();
            let expected_init_file = Path::new("api/sample-init-files")
                .join(p)
                .with_file_name("expected.bashrc");

            let resulting_contents =
                fs::read_to_string(test_init_file.unwrap().as_str())
                    .expect("Test shell init file doesn't exist");
            let expected_contents = fs::read_to_string(&expected_init_file)
                .unwrap_or_else(|_| {
                    panic!(
                        "No gold init file found at {}",
                        expected_init_file.display()
                    )
                });
            assert_eq!(resulting_contents.trim(), expected_contents.trim());
        }
    }

    async fn check_set_api_key_sample_unit(
        db: &ZammDatabase,
        sample_file: &str,
        existing_zamm_api_keys: &ZammApiKeys,
    ) {
        check_set_api_key_sample(
            db,
            sample_file,
            existing_zamm_api_keys,
            "set_api_key",
        )
        .await;
    }

    #[tokio::test]
    async fn test_write_new_init_file() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            "api/sample-calls/set_api_key-no-file.yaml",
            &api_keys,
        )
        .await;
    }

    #[tokio::test]
    async fn test_overwrite_existing_init_file_with_newline() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            "api/sample-calls/set_api_key-existing-with-newline.yaml",
            &api_keys,
        )
        .await;
    }

    #[tokio::test]
    async fn test_overwrite_existing_init_file_no_newline() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
            &api_keys,
        )
        .await;
    }

    #[tokio::test]
    async fn test_no_disk_write() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            "api/sample-calls/set_api_key-no-disk-write.yaml",
            &api_keys,
        )
        .await;
    }

    #[tokio::test]
    async fn test_unset() {
        let dummy_key = "0p3n41-4p1-k3y";
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: Some(dummy_key.to_string()),
        }));
        let mut conn = setup_database();
        diesel::insert_into(api_keys::table)
            .values(&NewApiKey {
                service: Service::OpenAI,
                api_key: dummy_key,
            })
            .execute(&mut conn)
            .unwrap();

        check_set_api_key_sample_unit(
            &ZammDatabase(Mutex::new(Some(conn))),
            "api/sample-calls/set_api_key-unset.yaml",
            &api_keys,
        )
        .await;
        assert!(api_keys.0.lock().await.openai.is_none());
    }

    #[tokio::test]
    async fn test_empty_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            "api/sample-calls/set_api_key-empty-filename.yaml",
            &api_keys,
        )
        .await;
    }

    #[tokio::test]
    async fn test_invalid_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            &setup_zamm_db(),
            "api/sample-calls/set_api_key-invalid-filename.yaml",
            &api_keys,
        )
        .await;
    }
}

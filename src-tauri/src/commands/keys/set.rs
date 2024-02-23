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
    use crate::test_helpers::api_testing::{
        check_zamm_result, serialize_zamm_result, standard_test_subdir,
    };
    use crate::test_helpers::{
        setup_database, setup_zamm_db, SampleCallTestCase, SideEffectsHelpers,
        ZammResultReturn,
    };
    use diesel::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::env;
    use stdext::function_name;
    use tokio::sync::Mutex;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SetApiKeyRequest {
        filename: Option<String>,
        service: Service,
        api_key: String,
    }

    struct SetApiKeyTestCase<'a> {
        test_fn_name: &'static str,
        api_keys: &'a ZammApiKeys,
        db: &'a ZammDatabase,
        json_replacements: HashMap<String, String>,
    }

    impl<'a> SampleCallTestCase<SetApiKeyRequest, ZammResult<()>>
        for SetApiKeyTestCase<'a>
    {
        const EXPECTED_API_CALL: &'static str = "set_api_key";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<SetApiKeyRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<()> {
            let request = args.as_ref().unwrap();
            let temp_test_dir = side_effects.disk.as_ref().unwrap();
            let current_dir = env::current_dir().unwrap();
            env::set_current_dir(temp_test_dir).unwrap();

            let result = set_api_key_helper(
                self.api_keys,
                self.db,
                request.filename.as_deref(),
                &request.service,
                request.api_key.clone(),
            )
            .await;
            env::set_current_dir(current_dir).unwrap();
            result
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<()>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&SetApiKeyRequest>,
            result: &ZammResult<()>,
        ) {
            check_zamm_result(sample, result);

            // check that the API call actually modified the in-memory API keys,
            // regardless of success or failure. check the database as well
            let existing_api_keys = &self.api_keys.0.lock().await;
            let actual_args = args.unwrap();
            if actual_args.api_key.is_empty() {
                assert_eq!(existing_api_keys.openai, None);
                assert_eq!(get_openai_api_key_from_db(self.db).await, None);
            } else {
                let arg_api_key = Some(actual_args.api_key.clone());
                assert_eq!(existing_api_keys.openai, arg_api_key);
                assert_eq!(get_openai_api_key_from_db(self.db).await, arg_api_key);
            }
        }
    }

    impl<'a> ZammResultReturn<SetApiKeyRequest, ()> for SetApiKeyTestCase<'a> {
        fn serialize_result(
            &self,
            _sample: &SampleCall,
            result: &ZammResult<()>,
        ) -> String {
            let actual_json = serialize_zamm_result(result);
            self.json_replacements
                .iter()
                .fold(actual_json, |acc, (k, v)| acc.replace(k, v))
        }
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

    pub async fn check_set_api_key_sample<'a>(
        test_fn_name: &'static str,
        db: &'a ZammDatabase,
        existing_zamm_api_keys: &'a ZammApiKeys,
        sample_file: &str,
        json_replacements: HashMap<String, String>,
    ) {
        let mut test_case = SetApiKeyTestCase {
            test_fn_name,
            api_keys: existing_zamm_api_keys,
            db,
            json_replacements,
        };
        test_case.check_sample_call(sample_file).await;
    }

    async fn check_set_api_key_sample_unit(
        test_fn_name: &'static str,
        db: &ZammDatabase,
        existing_zamm_api_keys: &ZammApiKeys,
        sample_file: &str,
    ) {
        check_set_api_key_sample(
            test_fn_name,
            db,
            existing_zamm_api_keys,
            sample_file,
            HashMap::new(),
        )
        .await;
    }

    #[tokio::test]
    async fn test_write_new_init_file() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-no-file.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_overwrite_existing_init_file_with_newline() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-existing-with-newline.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_overwrite_existing_init_file_no_newline() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_nested_folder() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-nested-folder.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_no_disk_write() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-no-disk-write.yaml",
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
            function_name!(),
            &ZammDatabase(Mutex::new(Some(conn))),
            &api_keys,
            "api/sample-calls/set_api_key-unset.yaml",
        )
        .await;
        assert!(api_keys.0.lock().await.openai.is_none());
    }

    #[tokio::test]
    async fn test_empty_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample_unit(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-empty-filename.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_invalid_filename() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));
        check_set_api_key_sample(
            function_name!(),
            &setup_zamm_db(),
            &api_keys,
            "api/sample-calls/set_api_key-invalid-filename.yaml",
            HashMap::from([(
                // error on Windows
                "\"The system cannot find the path specified. (os error 3)\""
                    .to_string(),
                // should be replaced by equivalent error on Linux
                "\"Is a directory (os error 21)\"".to_string(),
            )]),
        )
        .await;
    }
}

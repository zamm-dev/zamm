use crate::commands::errors::ZammResult;
use crate::setup::api_keys::ApiKeys;
use crate::ZammApiKeys;
use specta::specta;
use std::clone::Clone;
use tauri::State;

async fn get_api_keys_helper(zamm_api_keys: &ZammApiKeys) -> ApiKeys {
    zamm_api_keys.0.lock().await.clone()
}

#[tauri::command(async)]
#[specta]
pub async fn get_api_keys(api_keys: State<'_, ZammApiKeys>) -> ZammResult<ApiKeys> {
    Ok(get_api_keys_helper(&api_keys).await)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::test_helpers::SampleCallTestCase;
    use tokio::sync::Mutex;

    struct GetApiKeysTestCase<'a> {
        api_keys: &'a ZammApiKeys,
    }

    impl<'a> SampleCallTestCase<(), ZammResult<ApiKeys>> for GetApiKeysTestCase<'a> {
        const EXPECTED_API_CALL: &'static str = "get_api_keys";
        const CALL_HAS_ARGS: bool = false;

        async fn make_request(&mut self, _args: &Option<()>) -> ZammResult<ApiKeys> {
            Ok(get_api_keys_helper(&self.api_keys).await)
        }
    }

    pub async fn check_get_api_keys_sample<'a>(
        file_prefix: &str,
        rust_input: &'a ZammApiKeys,
    ) {
        let mut test_case = GetApiKeysTestCase {
            api_keys: rust_input,
        };
        let call = test_case.check_sample_call(file_prefix).await;
        let actual_json = serde_json::to_string_pretty(&call.result.unwrap()).unwrap();
        let expected_json = call.sample.response.message.trim();
        assert_eq!(actual_json, expected_json);
    }

    #[tokio::test]
    async fn test_get_empty_keys() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-empty.yaml",
            &api_keys,
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_openai_key() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys {
            openai: Some("0p3n41-4p1-k3y".to_string()),
        }));

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-openai.yaml",
            &api_keys,
        )
        .await;
    }
}

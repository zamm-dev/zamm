mod get;
mod set;

pub use get::get_api_keys;
pub use set::set_api_key;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::api_keys::ApiKeys;
    use crate::ZammApiKeys;
    use get::tests::check_get_api_keys_sample;
    use set::tests::check_set_api_key_sample;
    use std::collections::HashMap;
    use stdext::function_name;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_get_after_set() {
        let api_keys = ZammApiKeys(Mutex::new(ApiKeys::default()));

        check_set_api_key_sample(
            function_name!(),
            &api_keys,
            "api/sample-calls/set_api_key-existing-no-newline.yaml",
            HashMap::new(),
        )
        .await;

        check_get_api_keys_sample(
            "./api/sample-calls/get_api_keys-openai.yaml",
            &api_keys,
        )
        .await;
    }
}

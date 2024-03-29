use specta::specta;
use std::fs;

use std::path::PathBuf;

use crate::commands::errors::ZammResult;
use crate::commands::preferences::models::{get_preferences_file, Preferences};

fn get_preferences_happy_path(
    maybe_preferences_dir: &Option<PathBuf>,
) -> ZammResult<Preferences> {
    let preferences_path = get_preferences_file(maybe_preferences_dir.as_ref())?;
    let display_filename = preferences_path.display();
    if preferences_path.exists() {
        println!("Reading preferences from {display_filename}");
        let contents = fs::read_to_string(preferences_path)?;
        let preferences: Preferences = toml::from_str(&contents)?;
        Ok(preferences)
    } else {
        println!("No preferences found at {display_filename}");
        Ok(Preferences::default())
    }
}

fn get_preferences_helper(preferences_path: &Option<PathBuf>) -> Preferences {
    match get_preferences_happy_path(preferences_path) {
        Ok(preferences) => preferences,
        Err(e) => {
            eprintln!("Error getting preferences: {e}");
            Preferences::default()
        }
    }
}

#[tauri::command(async)]
#[specta]
pub fn get_preferences(app_handle: tauri::AppHandle) -> Preferences {
    let app_dir = app_handle.path_resolver().app_config_dir();
    get_preferences_helper(&app_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::api_testing::standard_test_subdir;
    use crate::test_helpers::{DirectReturn, SampleCallTestCase, SideEffectsHelpers};
    use stdext::function_name;

    struct GetPreferencesTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<(), Preferences> for GetPreferencesTestCase {
        const EXPECTED_API_CALL: &'static str = "get_preferences";
        const CALL_HAS_ARGS: bool = false;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            _: &Option<()>,
            side_effects: &SideEffectsHelpers,
        ) -> Preferences {
            get_preferences_helper(&side_effects.disk)
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &Preferences,
        ) -> String {
            DirectReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&()>,
            result: &Preferences,
        ) {
            DirectReturn::check_result(self, sample, args, result).await
        }
    }

    impl DirectReturn<(), Preferences> for GetPreferencesTestCase {}

    async fn check_get_preferences_sample<'a>(
        test_fn_name: &'static str,
        file_prefix: &str,
    ) {
        let mut test_case = GetPreferencesTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_get_preferences_without_file() {
        check_get_preferences_sample(
            function_name!(),
            "./api/sample-calls/get_preferences-no-file.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_preferences_with_sound_override() {
        check_get_preferences_sample(
            function_name!(),
            "./api/sample-calls/get_preferences-sound-override.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_preferences_with_volume_override() {
        check_get_preferences_sample(
            function_name!(),
            "./api/sample-calls/get_preferences-volume-override.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_preferences_with_extra_settings() {
        check_get_preferences_sample(
            function_name!(),
            "./api/sample-calls/get_preferences-extra-settings.yaml",
        )
        .await;
    }
}

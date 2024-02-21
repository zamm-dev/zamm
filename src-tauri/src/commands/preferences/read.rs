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
    use crate::test_helpers::SampleCallTestCase;

    struct GetPreferencesTestCase<'a> {
        preferences_dir: &'a str,
    }

    impl<'a> SampleCallTestCase<(), Preferences> for GetPreferencesTestCase<'a> {
        const EXPECTED_API_CALL: &'static str = "get_preferences";
        const CALL_HAS_ARGS: bool = false;

        async fn make_request(&mut self, _args: &Option<()>) -> Preferences {
            get_preferences_helper(&Some(self.preferences_dir.into()))
        }
    }

    async fn check_get_preferences_sample<'a>(
        file_prefix: &str,
        preferences_dir: &'a str,
    ) {
        let mut test_case = GetPreferencesTestCase { preferences_dir };
        let call = test_case.check_sample_call(file_prefix).await;

        let actual_json = serde_json::to_string_pretty(&call.result).unwrap();
        let expected_json = call.sample.response.message.trim();
        assert_eq!(actual_json, expected_json);
    }

    #[tokio::test]
    async fn test_get_preferences_without_file() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-no-file.yaml",
            "./non-existent/path",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_preferences_with_sound_override() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-sound-override.yaml",
            "./api/sample-settings/sound-override",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_preferences_with_volume_override() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-volume-override.yaml",
            "./api/sample-settings/volume-override",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_preferences_with_extra_settings() {
        check_get_preferences_sample(
            "./api/sample-calls/get_preferences-extra-settings.yaml",
            "./api/sample-settings/extra-settings",
        )
        .await;
    }
}

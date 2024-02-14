use anyhow::anyhow;
use specta::specta;
use std::fs;
use std::path::PathBuf;
use toml::map::Entry;
use toml::Table;
use toml::Value;

use crate::commands::errors::ZammResult;
use crate::commands::preferences::models::{get_preferences_file, Preferences};

fn deep_merge(base: &mut Value, other: &Value) {
    match (base, other) {
        (&mut Value::Table(ref mut base_map), Value::Table(other_map)) => {
            for (k, v) in other_map {
                match base_map.entry(k.clone()) {
                    Entry::Vacant(entry) => {
                        entry.insert(v.clone());
                    }
                    Entry::Occupied(mut entry) => {
                        deep_merge(entry.get_mut(), v);
                    }
                }
            }
        }
        (base, other) => {
            *base = other.clone();
        }
    }
}

fn set_preferences_helper(
    maybe_preferences_dir: &Option<PathBuf>,
    preferences: &Preferences,
) -> ZammResult<()> {
    let preferences_dir = maybe_preferences_dir
        .as_ref()
        .ok_or(anyhow!("No preferences dir found"))?;
    let preferences_path = get_preferences_file(Some(preferences_dir))?;
    let mut existing_yaml: Value = if preferences_path.exists() {
        let file_contents = fs::read_to_string(&preferences_path)?;
        toml::from_str::<Table>(&file_contents)?.into()
    } else {
        toml::Table::new().into()
    };

    let override_toml = Table::try_from(preferences)?;
    deep_merge(&mut existing_yaml, &override_toml.into());

    let merged_prefs_str = toml::to_string(&existing_yaml)?;
    fs::create_dir_all(preferences_dir)?;
    fs::write(preferences_path, merged_prefs_str)?;
    Ok(())
}

#[tauri::command(async)]
#[specta]
pub fn set_preferences(
    app_handle: tauri::AppHandle,
    preferences: Preferences,
) -> ZammResult<()> {
    let app_dir = app_handle.path_resolver().app_config_dir();
    match set_preferences_helper(&app_dir, &preferences) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error writing preferences: {e}");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::get_temp_test_dir;
    use serde::{Deserialize, Serialize};

    use std::fs;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SetPreferencesRequest {
        preferences: Preferences,
    }

    fn parse_request(request_str: &str) -> SetPreferencesRequest {
        serde_json::from_str(request_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_set_preferences_sample(
        file_prefix: &str,
        existing_preferences_file: Option<&str>,
        expected_preferences_file: &str,
    ) {
        let sample = read_sample(file_prefix);
        assert_eq!(sample.request.len(), 2);
        assert_eq!(sample.request[0], "set_preferences");

        let test_preferences_dir = get_temp_test_dir(
            PathBuf::from(file_prefix)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        let test_preferences_file: PathBuf =
            get_preferences_file(Some(&test_preferences_dir)).unwrap();
        println!(
            "Test will use preference file at {}",
            test_preferences_file.display()
        );

        if let Some(existing_preferences) = existing_preferences_file {
            let test_preferences_path = test_preferences_file.as_path();
            fs::copy(existing_preferences, test_preferences_path).unwrap_or_else(|e| {
                panic!(
                    "Can't copy existing preferences file from {} to {}: {}",
                    existing_preferences,
                    test_preferences_path.display(),
                    e
                )
            });
        }

        let actual_request = parse_request(&sample.request[1]);
        let actual_result = set_preferences_helper(
            &Some(test_preferences_dir),
            &actual_request.preferences,
        );
        assert!(actual_result.is_ok());
        let actual_json =
            serde_json::to_string_pretty(&actual_result.unwrap()).unwrap();
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);

        let resulting_contents = fs::read_to_string(test_preferences_file)
            .expect("Test preferences file doesn't exist");
        let expected_contents = fs::read_to_string(expected_preferences_file)
            .unwrap_or_else(|_| {
                panic!("No file found at {}", expected_preferences_file)
            });
        assert_eq!(resulting_contents.trim(), expected_contents.trim());
    }

    #[test]
    fn test_set_preferences_sound_off_without_file() {
        check_set_preferences_sample(
            "./api/sample-calls/set_preferences-sound-off.yaml",
            None,
            "./api/sample-settings/sound-override/preferences.toml",
        );
    }

    #[test]
    fn test_set_preferences_sound_on_with_extra_settings() {
        check_set_preferences_sample(
            "./api/sample-calls/set_preferences-sound-on.yaml",
            Some("./api/sample-settings/extra-settings/preferences.toml"),
            "./api/sample-settings/extra-settings/sound-on.toml",
        );
    }

    #[test]
    fn test_set_preferences_volume_partial() {
        check_set_preferences_sample(
            "./api/sample-calls/set_preferences-volume-partial.yaml",
            None,
            "./api/sample-settings/volume-override/preferences.toml",
        );
    }
}

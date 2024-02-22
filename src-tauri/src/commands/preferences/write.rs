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
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use serde::{Deserialize, Serialize};
    use stdext::function_name;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SetPreferencesRequest {
        preferences: Preferences,
    }

    struct SetPreferencesTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<SetPreferencesRequest, ZammResult<()>>
        for SetPreferencesTestCase
    {
        const EXPECTED_API_CALL: &'static str = "set_preferences";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            let test_logical_path =
                self.test_fn_name.split("::").collect::<Vec<&str>>();
            let test_name = test_logical_path[test_logical_path.len() - 2];
            format!("{}/{}", Self::EXPECTED_API_CALL, test_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<SetPreferencesRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<()> {
            set_preferences_helper(
                &side_effects.disk,
                &args.as_ref().unwrap().preferences,
            )
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
            args: Option<&SetPreferencesRequest>,
            result: &ZammResult<()>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<SetPreferencesRequest, ()> for SetPreferencesTestCase {}

    async fn check_set_preferences_sample(
        test_fn_name: &'static str,
        file_prefix: &str,
    ) {
        let mut test_case = SetPreferencesTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_set_preferences_sound_off_without_file() {
        check_set_preferences_sample(
            function_name!(),
            "./api/sample-calls/set_preferences-sound-off.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_set_preferences_sound_on_with_extra_settings() {
        check_set_preferences_sample(
            function_name!(),
            "./api/sample-calls/set_preferences-sound-on.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_set_preferences_volume_partial() {
        check_set_preferences_sample(
            function_name!(),
            "./api/sample-calls/set_preferences-volume-partial.yaml",
        )
        .await;
    }
}

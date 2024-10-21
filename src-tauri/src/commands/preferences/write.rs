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

pub fn set_preferences_helper(
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
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SetPreferencesRequest {
        preferences: Preferences,
    }

    async fn make_request_helper(
        args: &SetPreferencesRequest,
        side_effects: &mut SideEffectsHelpers,
    ) -> ZammResult<()> {
        set_preferences_helper(&side_effects.disk, &args.preferences)
    }

    impl_result_test_case!(
        SetPreferencesTestCase,
        set_preferences,
        true,
        SetPreferencesRequest,
        ()
    );

    check_sample!(
        SetPreferencesTestCase,
        test_sound_off_without_file,
        "./api/sample-calls/set_preferences-sound-off.yaml"
    );

    check_sample!(
        SetPreferencesTestCase,
        test_sound_on_with_extra_settings,
        "./api/sample-calls/set_preferences-sound-on.yaml"
    );

    check_sample!(
        SetPreferencesTestCase,
        test_volume_partial,
        "./api/sample-calls/set_preferences-volume-partial.yaml"
    );

    check_sample!(
        SetPreferencesTestCase,
        test_transparency_on,
        "./api/sample-calls/set_preferences-transparency-on.yaml"
    );

    check_sample!(
        SetPreferencesTestCase,
        test_transparency_off,
        "./api/sample-calls/set_preferences-transparency-off.yaml"
    );

    check_sample!(
        SetPreferencesTestCase,
        test_high_dpi_adjust_on,
        "./api/sample-calls/set_preferences-high-dpi-adjust-on.yaml"
    );

    check_sample!(
        SetPreferencesTestCase,
        test_high_dpi_adjust_off,
        "./api/sample-calls/set_preferences-high-dpi-adjust-on.yaml"
    );
}

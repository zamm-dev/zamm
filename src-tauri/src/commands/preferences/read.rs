use specta::specta;
use std::fs;

use std::path::PathBuf;

use crate::commands::errors::ZammResult;
use crate::commands::preferences::models::{get_preferences_file, Preferences};

pub fn get_preferences_file_contents(
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

fn get_preferences_happy_path(
    maybe_preferences_dir: &Option<PathBuf>,
) -> ZammResult<Preferences> {
    #[allow(unused_mut)]
    let mut found_preferences = get_preferences_file_contents(maybe_preferences_dir)?;
    #[cfg(target_os = "windows")]
    if found_preferences.transparency_on.is_none() {
        found_preferences.transparency_on = Some(true);
    }
    #[cfg(target_os = "macos")]
    if found_preferences.high_dpi_adjust.is_none() {
        found_preferences.high_dpi_adjust = Some(true);
    }
    Ok(found_preferences)
}

pub fn get_preferences_helper(preferences_path: &Option<PathBuf>) -> Preferences {
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
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_direct_test_case};

    async fn make_request_helper(
        _: &(),
        side_effects: &SideEffectsHelpers,
    ) -> Preferences {
        get_preferences_helper(&side_effects.disk)
    }

    impl_direct_test_case!(
        GetPreferencesTestCase,
        get_preferences,
        false,
        (),
        Preferences
    );

    #[cfg(target_os = "linux")]
    check_sample!(
        GetPreferencesTestCase,
        test_without_file,
        "./api/sample-calls/get_preferences-no-file.yaml"
    );

    #[cfg(target_os = "windows")]
    check_sample!(
        GetPreferencesTestCase,
        test_without_file,
        "./api/sample-calls/get_preferences-no-file-windows.yaml"
    );

    #[cfg(target_os = "macos")]
    check_sample!(
        GetPreferencesTestCase,
        test_without_file,
        "./api/sample-calls/get_preferences-no-file-mac.yaml"
    );

    #[cfg(target_os = "linux")]
    check_sample!(
        GetPreferencesTestCase,
        test_sound_override,
        "./api/sample-calls/get_preferences-sound-override.yaml"
    );

    #[cfg(target_os = "linux")]
    check_sample!(
        GetPreferencesTestCase,
        test_volume_override,
        "./api/sample-calls/get_preferences-volume-override.yaml"
    );

    #[cfg(not(target_os = "macos"))]
    check_sample!(
        GetPreferencesTestCase,
        test_transparency_off,
        "./api/sample-calls/get_preferences-transparency-off.yaml"
    );

    #[cfg(not(target_os = "macos"))]
    check_sample!(
        GetPreferencesTestCase,
        test_transparency_on,
        "./api/sample-calls/get_preferences-transparency-on.yaml"
    );

    #[cfg(target_os = "linux")]
    check_sample!(
        GetPreferencesTestCase,
        test_extra_settings,
        "./api/sample-calls/get_preferences-extra-settings.yaml"
    );

    #[cfg(not(target_os = "windows"))]
    check_sample!(
        GetPreferencesTestCase,
        test_high_dpi_adjust,
        "./api/sample-calls/get_preferences-high-dpi-adjust-on.yaml"
    );
}

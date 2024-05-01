use crate::commands::errors::ZammResult;
use anyhow::anyhow;
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;

pub static PREFERENCES_FILENAME: &str = "preferences.toml";

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct Preferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}

pub fn get_preferences_file(
    maybe_preferences_dir: Option<&PathBuf>,
) -> ZammResult<PathBuf> {
    let preferences_dir =
        maybe_preferences_dir.ok_or(anyhow!("No preferences dir found"))?;
    let relative_preferences_path = preferences_dir.join(PREFERENCES_FILENAME);
    let absolute_preferences_path = relative_preferences_path.absolutize()?;
    Ok(absolute_preferences_path.into_owned())
}

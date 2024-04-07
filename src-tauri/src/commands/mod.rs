pub mod errors;
mod keys;
mod llms;
mod preferences;
mod sounds;
mod system;

pub use errors::Error;
pub use keys::{get_api_keys, set_api_key};
pub use llms::{chat, get_api_call, get_api_calls};
pub use preferences::{get_preferences, set_preferences};
pub use sounds::play_sound;
pub use system::get_system_info;

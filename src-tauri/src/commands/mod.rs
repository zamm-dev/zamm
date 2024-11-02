pub mod database;
pub mod errors;
mod keys;
mod llms;
pub mod preferences;
mod sounds;
mod system;
pub mod terminal;

// size of one page of results in database list view
const PAGE_SIZE: i64 = 50;

pub use database::{export_db, import_db};
pub use errors::Error;
pub use keys::{get_api_keys, set_api_key};
pub use llms::{chat, get_api_call, get_api_calls};
pub use preferences::{get_preferences, set_preferences};
pub use sounds::play_sound;
pub use system::get_system_info;
pub use terminal::{get_terminal_sessions, run_command, send_command_input};

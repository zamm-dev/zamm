// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod commands;
mod models;
#[cfg(test)]
mod sample_call;
mod schema;
mod setup;
#[cfg(test)]
mod test_helpers;
mod views;

use clap::Parser;
use diesel::sqlite::SqliteConnection;
use setup::api_keys::{setup_api_keys, ApiKeys};
#[cfg(debug_assertions)]
use specta::collect_types;
use std::env;
#[cfg(debug_assertions)]
use tauri_specta::ts;
use tokio::sync::Mutex;

use cli::{Cli, Commands};
use commands::{
    chat, get_api_call, get_api_calls, get_api_keys, get_preferences, get_system_info,
    play_sound, set_api_key, set_preferences,
};

pub struct ZammDatabase(Mutex<Option<SqliteConnection>>);
pub struct ZammApiKeys(Mutex<ApiKeys>);

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        #[cfg(debug_assertions)]
        Some(Commands::ExportBindings {}) => {
            ts::export(
                collect_types![
                    get_api_keys,
                    set_api_key,
                    play_sound,
                    get_preferences,
                    set_preferences,
                    get_system_info,
                    chat,
                    get_api_call,
                    get_api_calls,
                ],
                "../src-svelte/src/lib/bindings.ts",
            )
            .expect("Failed to export Specta bindings");
            println!("Specta bindings should be exported to ../src-svelte/src/lib/bindings.ts");
        }
        Some(Commands::Gui {}) | None => {
            let mut possible_db = setup::get_db();
            let api_keys = setup_api_keys(&mut possible_db);

            tauri::Builder::default()
                .manage(ZammDatabase(Mutex::new(possible_db)))
                .manage(ZammApiKeys(Mutex::new(api_keys)))
                .invoke_handler(tauri::generate_handler![
                    get_api_keys,
                    set_api_key,
                    play_sound,
                    get_preferences,
                    set_preferences,
                    get_system_info,
                    chat,
                    get_api_call,
                    get_api_calls,
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
}

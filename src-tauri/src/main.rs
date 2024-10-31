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
mod upgrades;
mod views;

use std::collections::HashMap;

use clap::Parser;
use commands::terminal::Terminal;
use diesel::sqlite::SqliteConnection;
use futures::executor;
use models::llm_calls::EntityId;
use setup::api_keys::{setup_api_keys, ApiKeys};
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri::Manager;
#[cfg(debug_assertions)]
use tauri_specta::{collect_commands, Builder};
use tokio::sync::Mutex;

use cli::{Cli, Commands};
use commands::preferences::get_preferences_file_contents;
use commands::{
    chat, export_db, get_api_call, get_api_calls, get_api_keys, get_preferences,
    get_system_info, import_db, play_sound, run_command, send_command_input,
    set_api_key, set_preferences,
};
use upgrades::handle_app_upgrades;

pub struct ZammDatabase(Mutex<Option<SqliteConnection>>);
pub struct ZammApiKeys(Mutex<ApiKeys>);
pub struct ZammTerminalSessions(Mutex<HashMap<EntityId, Box<dyn Terminal>>>);

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        #[cfg(debug_assertions)]
        Some(Commands::ExportBindings {}) => {
            let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
                get_api_keys,
                set_api_key,
                play_sound,
                get_preferences,
                set_preferences,
                get_system_info,
                chat,
                get_api_call,
                get_api_calls,
                import_db,
                export_db,
                run_command,
                send_command_input,
            ]);
            builder
                .export(Typescript::default(), "../src-svelte/src/lib/bindings.ts")
                .expect("Failed to export Specta bindings");
            println!("Specta bindings should be exported to ../src-svelte/src/lib/bindings.ts");
        }
        Some(Commands::Gui {}) | None => {
            let mut possible_db = setup::get_db();
            let api_keys = setup_api_keys(&mut possible_db);
            let terminal_sessions = HashMap::new();

            tauri::Builder::default()
                .setup(|app| {
                    let config_dir = app.path().app_config_dir().ok();
                    let zamm_db = app.state::<ZammDatabase>();

                    executor::block_on(async {
                        handle_app_upgrades(&config_dir, &zamm_db)
                            .await
                            .unwrap_or_else(|e| {
                                eprintln!("Couldn't run custom data migrations: {e}");
                                eprintln!("Continuing with unchanged data");
                            });
                    });

                    let prefs = get_preferences_file_contents(&config_dir)?;
                    #[cfg(target_os = "macos")]
                    let high_dpi_adjust_on = prefs.high_dpi_adjust.unwrap_or(true);
                    #[cfg(not(target_os = "macos"))]
                    let high_dpi_adjust_on = prefs.high_dpi_adjust.unwrap_or(false);
                    if high_dpi_adjust_on {
                        app.get_webview_window("main")
                            .ok_or(anyhow::anyhow!("No main window"))?
                            .set_size(tauri::Size::Logical(tauri::LogicalSize {
                                width: 708.0,  // 850 * 0.8333...
                                height: 541.0, // 650 * 0.8333...
                            }))?;
                    } else {
                        app.get_webview_window("main")
                            .ok_or(anyhow::anyhow!("No main window"))?
                            .set_size(tauri::Size::Logical(tauri::LogicalSize {
                                width: 850.0,
                                height: 650.0,
                            }))?;
                    }

                    Ok(())
                })
                .manage(ZammDatabase(Mutex::new(possible_db)))
                .manage(ZammApiKeys(Mutex::new(api_keys)))
                .manage(ZammTerminalSessions(Mutex::new(terminal_sessions)))
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
                    import_db,
                    export_db,
                    run_command,
                    send_command_input,
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
}

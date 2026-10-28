//! Meridian Installer's app: the window (`src/`) over the engine
//! (`meridian-installer-core`), through the commands in [`commands`].
#![doc = include_str!("../README.md")]

mod commands;
mod host;
mod startup;
mod state;

use std::sync::Arc;

use host::commands::INSTALLER;
use tauri::plugin::Builder;

// Both lists are also read by `build.rs`; here they must not be empty.
const _: () = assert!(!host::commands::ALL.is_empty() && !host::events::ALL.is_empty());

/// Starts the app.
///
/// # Errors
///
/// Tauri's error when the window cannot be made or run.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    let state = Arc::new(state::AppState::new(startup::operations()));
    tauri::Builder::default()
        .plugin(tauri_plugin_substrate::init())
        .manage(state)
        .plugin(
            Builder::<tauri::Wry>::new(INSTALLER.plugin)
                .invoke_handler(tauri::generate_handler![
                    commands::snapshot,
                    commands::selection_of,
                    commands::plan,
                    commands::apply,
                    commands::uninstall,
                    commands::cancel,
                    commands::pause,
                    commands::launch,
                    commands::self_update,
                ])
                .build(),
        )
        .run(tauri::generate_context!())
}

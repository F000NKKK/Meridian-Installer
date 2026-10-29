//! Starts the app: the plugins, the commands, the window.

use std::sync::Arc;

use crate::host::commands::INSTALLER;
use tauri::plugin::Builder;

// Both lists are also read by `build.rs`; here they must not be empty.
const _: () =
    assert!(!crate::host::commands::ALL.is_empty() && !crate::host::events::ALL.is_empty());

/// Starts the app.
///
/// # Errors
///
/// Tauri's error when the window cannot be made or run.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    #[cfg(target_os = "linux")]
    crate::restart::wait_for_parent()?;
    let state = Arc::new(crate::app_state::AppState::new(crate::startup::operations()));
    tauri::Builder::default()
        .plugin(tauri_plugin_substrate::init())
        .manage(state)
        .plugin(
            Builder::<tauri::Wry>::new(INSTALLER.plugin)
                .invoke_handler(tauri::generate_handler![
                    crate::commands::snapshot,
                    crate::commands::selection_of,
                    crate::commands::plan,
                    crate::commands::apply,
                    crate::commands::uninstall,
                    crate::commands::cancel,
                    crate::commands::pause,
                    crate::commands::launch,
                    crate::commands::self_update,
                ])
                .build(),
        )
        .run(tauri::generate_context!())
}

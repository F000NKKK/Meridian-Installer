//! The commands the window calls (`installer.installer.<command>`): each one
//! asks the operations of the core and turns an error into a sentence.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value: `State` and deserialized parameters cannot be borrowed"
)]

use std::sync::Arc;

use meridian_installer_core::activity::{ApplyRequest, Emit, describe};
use meridian_installer_core::display::{EventView, PlanView, SelectionRequest, Snapshot};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::app_state::AppState;
use crate::host::events::PROGRESS;

type Reply<T> = Result<T, String>;

/// An operation's progress, as the window reads it.
#[derive(Debug, Clone, Serialize)]
struct Progress<'a> {
    product: &'a str,
    event: EventView,
}

/// Who hears an operation: the window, by the progress event.
fn emitter(app: AppHandle) -> Emit {
    Arc::new(move |product, event| {
        // A window that is gone is nobody listening.
        app.emit(PROGRESS, Progress { product, event }).ok();
    })
}

/// Everything the window shows; with `refresh`, every source is asked again.
#[tauri::command]
pub(crate) fn snapshot(state: State<'_, Arc<AppState>>, refresh: bool) -> Reply<Snapshot> {
    state
        .operations()?
        .snapshot(refresh)
        .map_err(|e| describe(&e))
}

/// The checkboxes that show what is installed of `product`.
#[tauri::command]
pub(crate) fn selection_of(
    state: State<'_, Arc<AppState>>,
    product: String,
) -> Reply<SelectionRequest> {
    state
        .operations()?
        .selection_of(&product)
        .map_err(|e| describe(&e))
}

/// What `request` would do.
#[tauri::command]
pub(crate) fn plan(state: State<'_, Arc<AppState>>, request: ApplyRequest) -> Reply<PlanView> {
    state.operations()?.plan(&request).map_err(|e| describe(&e))
}

/// Installs or modifies a product; the progress comes as events.
#[tauri::command]
pub(crate) fn apply(
    state: State<'_, Arc<AppState>>,
    app: AppHandle,
    request: ApplyRequest,
) -> Reply<()> {
    state
        .operations()?
        .start_apply(&request, emitter(app))
        .map(drop)
        .map_err(|e| describe(&e))
}

/// Uninstalls a product; the progress comes as events.
#[tauri::command]
pub(crate) fn uninstall(
    state: State<'_, Arc<AppState>>,
    app: AppHandle,
    product: String,
) -> Reply<()> {
    state
        .operations()?
        .start_uninstall(&product, emitter(app))
        .map(drop)
        .map_err(|e| describe(&e))
}

/// Updates the Installer itself; the progress comes as events.
#[tauri::command]
pub(crate) fn self_update(state: State<'_, Arc<AppState>>, app: AppHandle) -> Reply<()> {
    state
        .operations()?
        .start_self_update(emitter(app))
        .map(drop)
        .map_err(|e| describe(&e))
}

/// Cancels the operation running.
#[tauri::command]
pub(crate) fn cancel(state: State<'_, Arc<AppState>>) -> Reply<()> {
    state.operations()?.cancel();
    Ok(())
}

/// Pauses or resumes the operation running.
#[tauri::command]
pub(crate) fn pause(state: State<'_, Arc<AppState>>, paused: bool) -> Reply<()> {
    state.operations()?.pause(paused);
    Ok(())
}

/// Starts an installed product (or the Installer, after its update).
#[tauri::command]
pub(crate) fn launch(state: State<'_, Arc<AppState>>, product: String) -> Reply<()> {
    let operations = state.operations()?;
    let program = operations
        .program_of(&product)
        .map_err(|e| describe(&e))?
        .ok_or_else(|| format!("{product} is not installed"))?;
    substrate_process::start(&program).map_err(|e| describe(&e))
}

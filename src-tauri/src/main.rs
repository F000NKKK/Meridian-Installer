//! The Meridian Installer program: starts the window.
// Prevents an additional console window on Windows in release, DO NOT REMOVE!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> tauri::Result<()> {
    meridian_installer_lib::run()
}

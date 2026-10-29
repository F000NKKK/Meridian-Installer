//! Meridian Installer's app: the window (`src/`) over the engine
//! (`meridian-installer-core`), through the commands in [`commands`].
#![doc = include_str!("../README.md")]

mod app;
mod app_state;
mod commands;
mod first_run;
mod host;
#[cfg(target_os = "linux")]
mod restart;
mod startup;

pub use app::run;

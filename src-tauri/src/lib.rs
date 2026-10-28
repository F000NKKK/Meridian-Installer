//! Meridian Installer's app: the window (`src/`) over the engine
//! (`meridian-installer-core`), through the commands in [`commands`].
#![doc = include_str!("../README.md")]

mod app;
mod app_state;
mod commands;
mod first_run;
mod host;
mod startup;

pub use app::run;

//! The headless Meridian Installer: the commands of a terminal or CI over a
//! [`Session`](meridian_installer_core::engine::Session).
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod cli_error;
pub mod command_line;
mod commands;
mod context;
mod entry;
mod format_bytes;
mod progress_sink;
mod render;
mod run;
mod start;
mod started;
mod target;

pub use cli_error::CliError;
pub use context::Context;
pub use entry::main_exit;
pub use progress_sink::ProgressSink;
pub use run::run;
pub use start::start;
pub use started::Started;

//! The command line, typed.

mod apply_args;
mod cli;
mod command;
mod downloads_arg;
mod install_args;
mod mode_arg;
mod remove_args;
mod target_args;
mod target_name;
#[cfg(test)]
mod tests;
mod update_args;

pub use apply_args::ApplyArgs;
pub use cli::Cli;
pub use command::Command;
pub use downloads_arg::DownloadsArg;
pub use install_args::InstallArgs;
pub use mode_arg::ModeArg;
pub use remove_args::RemoveArgs;
pub use target_args::TargetArgs;
pub use target_name::TargetName;
pub use update_args::UpdateArgs;

//! The command line as a whole.

use std::path::PathBuf;

use clap::Parser;

use super::Command;

/// Meridian Installer without a window.
#[derive(Debug, Parser)]
#[command(name = "meridian-installer", version, about)]
pub struct Cli {
    /// The installer's home folder (default: `MERIDIAN_HOME`, else `~/.meridian`).
    #[arg(long, global = true, value_name = "DIR")]
    pub home: Option<PathBuf>,
    /// Print JSON instead of text.
    #[arg(long, global = true)]
    pub json: bool,
    /// Do not look for a newer catalog in the latest Installer release.
    #[arg(long, global = true)]
    pub no_refresh: bool,
    /// What to do.
    #[command(subcommand)]
    pub command: Command,
}

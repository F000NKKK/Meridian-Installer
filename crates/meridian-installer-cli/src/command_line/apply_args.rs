//! How a plan is carried out.

use clap::Args;

use super::{DownloadsArg, ModeArg};

/// How a plan is carried out.
#[derive(Debug, Args)]
pub struct ApplyArgs {
    /// When downloads happen.
    #[arg(long, value_enum, default_value_t)]
    pub mode: ModeArg,
    /// What happens to downloads once installed.
    #[arg(long, value_enum, default_value_t)]
    pub downloads: DownloadsArg,
}

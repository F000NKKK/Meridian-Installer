//! `install`.

use clap::Args;

use super::{ApplyArgs, TargetArgs};

/// `install`.
#[derive(Debug, Args)]
pub struct InstallArgs {
    /// What to install.
    #[command(flatten)]
    pub targets: TargetArgs,
    /// How to carry it out.
    #[command(flatten)]
    pub apply: ApplyArgs,
}

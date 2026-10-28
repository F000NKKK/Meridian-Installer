//! `update`.

use clap::Args;

use super::{ApplyArgs, TargetName};

/// `update`.
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Only these items (default: everything the installer installed).
    #[arg(value_name = "TARGET")]
    pub targets: Vec<TargetName>,
    /// How to carry it out.
    #[command(flatten)]
    pub apply: ApplyArgs,
}

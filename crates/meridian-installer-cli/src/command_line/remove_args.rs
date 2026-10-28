//! `remove`.

use clap::Args;

use super::TargetName;

/// `remove`.
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Components or products to remove.
    #[arg(required = true, value_name = "TARGET")]
    pub targets: Vec<TargetName>,
    /// Remove even what another installed component requires.
    #[arg(long)]
    pub force: bool,
}

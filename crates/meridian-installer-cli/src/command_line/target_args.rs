//! What `plan` and `install` are given.

use clap::Args;
use substrate_installer_domain::ids::ComponentId;

use super::TargetName;

/// What to install.
#[derive(Debug, Args)]
pub struct TargetArgs {
    /// Workloads, components or products.
    #[arg(required = true, value_name = "TARGET")]
    pub targets: Vec<TargetName>,
    /// A recommended component of a workload to leave out.
    #[arg(long, value_name = "COMPONENT")]
    pub without: Vec<ComponentId>,
}

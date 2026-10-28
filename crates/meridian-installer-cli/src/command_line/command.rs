//! The subcommands.

use clap::Subcommand;

use super::{InstallArgs, RemoveArgs, TargetArgs, UpdateArgs};

/// What to do.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// List the products, workloads and components, with what is installed and newest.
    List,
    /// Show what installing the targets would do.
    Plan(TargetArgs),
    /// Install workloads, components or products.
    Install(InstallArgs),
    /// Update what is installed (everything, or only the named items).
    Update(UpdateArgs),
    /// Remove components or products the installer installed.
    Remove(RemoveArgs),
    /// Show what is installed and whether it is up to date.
    Status,
}

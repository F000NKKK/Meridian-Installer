//! Why a command failed.

use substrate_installer_app::installing::ApplyError;
use substrate_installer_app::ports::records::StoreError;
use substrate_installer_domain::ids::IdError;
use substrate_installer_domain::versions::VersionError;

use meridian_installer_core::catalog::LoadError;
use meridian_installer_core::host::DetectError;
use meridian_installer_core::wiring::MachineError;

/// Why a command failed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CliError {
    /// A name is not an id.
    #[error(transparent)]
    Id(#[from] IdError),
    /// A name is in no part of the catalog.
    #[error("`{0}` is not a workload, component or product of the catalog")]
    NotInCatalog(String),
    /// Something installed needs what would be removed.
    #[error("{id} is required by {by}; remove that first or pass --force")]
    StillRequired {
        /// What would be removed.
        id: String,
        /// What requires it.
        by: String,
    },
    /// The plan does not fit what releases need.
    #[error("{0} requirement(s) of the releases are not met; nothing was installed")]
    Conflicts(usize),
    /// The records could not be read.
    #[error(transparent)]
    Store(#[from] StoreError),
    /// The plan stopped.
    #[error(transparent)]
    Apply(#[from] ApplyError),
    /// The machine could not be made.
    #[error(transparent)]
    Machine(#[from] MachineError),
    /// The host could not be told.
    #[error(transparent)]
    Host(#[from] DetectError),
    /// The catalog could not be had.
    #[error(transparent)]
    Catalog(#[from] LoadError),
    /// The installer's own version is not a version.
    #[error(transparent)]
    Version(#[from] VersionError),
    /// Output could not be written.
    #[error("output could not be written")]
    Output(#[from] std::io::Error),
    /// JSON could not be written.
    #[error("output could not be written as JSON")]
    Json(#[from] serde_json::Error),
}

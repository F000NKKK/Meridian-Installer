//! Why the running Installer could not be retained.

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::records::StoreError;

/// Why first-run installation or registration failed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SelfInstallError {
    /// This operating system has no supported product adapter.
    #[error("this system cannot install Meridian Installer")]
    UnsupportedSystem,
    /// The bundle or its desktop launcher could not be placed.
    #[error("Meridian Installer could not install its running bundle")]
    Placement(#[from] ProductError),
    /// The record of the installed bundle could not be read or saved.
    #[error("Meridian Installer's installed record could not be saved")]
    Records(#[from] StoreError),
}

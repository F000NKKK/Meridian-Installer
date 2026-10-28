//! The records of what is installed.

use std::path::PathBuf;

/// The store of the installed records: one Substrate Data binary file.
pub type InstalledRecords = substrate_installer_data::BinaryInstalledStore;

/// The records kept in the file `path`.
#[must_use]
pub fn installed_records(path: impl Into<PathBuf>) -> InstalledRecords {
    InstalledRecords::new(path)
}

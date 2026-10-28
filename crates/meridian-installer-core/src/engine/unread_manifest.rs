//! A manifest that could not be had.

use std::fmt;

use substrate_installer_domain::ids::ItemId;

use super::ManifestError;

/// A release whose compatibility manifest could not be had; the release is
/// taken to need nothing, and the caller says so.
#[derive(Debug)]
pub struct UnreadManifest {
    /// The release's item.
    pub id: ItemId,
    /// Why.
    pub error: ManifestError,
}

impl fmt::Display for UnreadManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.error)
    }
}

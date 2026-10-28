//! Reading a catalog and a compatibility manifest.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::compat::Needs;

/// Why a catalog file is not a catalog.
pub type CatalogFormatError = substrate_installer_data::CatalogFileError;

/// Why a compatibility manifest is not one.
pub type NeedsFormatError = substrate_installer_data::NeedsError;

/// A catalog, only when its `revision` is above `than`.
///
/// # Errors
///
/// [`CatalogFormatError`] when `text` is not a catalog.
pub fn newer_revision(text: &str, than: u32) -> Result<Option<Catalog>, CatalogFormatError> {
    substrate_installer_data::newer_revision(text, than)
}

/// What a release's manifest says it needs.
///
/// # Errors
///
/// [`NeedsFormatError`] when `text` is not a manifest.
pub fn parse_needs(text: &str) -> Result<Needs, NeedsFormatError> {
    substrate_installer_data::parse_needs(text)
}

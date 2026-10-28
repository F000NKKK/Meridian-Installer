//! The names of the installer's files.

use substrate_installer_domain::channel::Catalog;

use super::CatalogFormatError;

/// The catalog file, in the repository and as a release asset.
pub const CATALOG_FILE_NAME: &str = "catalog.sdata";

/// The file that records what is installed.
pub const INSTALLED_FILE_NAME: &str = "installed.sdata";

/// A catalog read from its text.
///
/// # Errors
///
/// [`CatalogFormatError`] naming the line and field of the first problem.
pub fn parse_catalog_text(text: &str) -> Result<Catalog, CatalogFormatError> {
    substrate_installer_data::parse_catalog(text)
}

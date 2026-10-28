//! Why a catalog cannot be had.

use crate::formats::CatalogFormatError;

/// Why a catalog cannot be had.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LoadError {
    /// The catalog built into the Installer is not a catalog.
    #[error("the built-in catalog is invalid")]
    BuiltIn(#[source] CatalogFormatError),
}

//! What went wrong with a catalog that is not the end of the world.

use std::path::PathBuf;

use crate::formats::CatalogFormatError;

/// Why the catalog a release brought is not used: the built-in one is, and
/// the window or terminal says so.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Notice {
    /// The saved catalog could not be read.
    #[error("the saved catalog {path} could not be read")]
    Unreadable {
        /// The file.
        path: PathBuf,
        /// Why.
        #[source]
        source: std::io::Error,
    },
    /// The saved catalog is not a catalog.
    #[error("the saved catalog {path} is invalid")]
    Invalid {
        /// The file.
        path: PathBuf,
        /// Why.
        #[source]
        source: CatalogFormatError,
    },
}

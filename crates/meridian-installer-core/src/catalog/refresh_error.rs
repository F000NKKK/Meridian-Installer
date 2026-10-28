//! Why a catalog could not be refreshed.

use crate::formats::CatalogFormatError;
use substrate_installer_app::ports::downloads::DownloadError;
use substrate_installer_app::ports::sources::ListingError;

/// Why the catalog could not be refreshed from the latest Installer release.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RefreshError {
    /// The Installer's releases could not be listed.
    #[error("the Installer's releases could not be listed")]
    Listing(#[from] ListingError),
    /// The catalog could not be downloaded.
    #[error("the catalog could not be downloaded")]
    Download(#[from] DownloadError),
    /// The downloaded file could not be read or put in place.
    #[error("the downloaded catalog could not be saved")]
    Files(#[source] std::io::Error),
    /// The downloaded file is not a catalog.
    #[error("the downloaded catalog is invalid")]
    Invalid(#[from] CatalogFormatError),
}

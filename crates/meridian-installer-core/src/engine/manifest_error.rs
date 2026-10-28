//! Why a release's manifest could not be had.

use substrate_installer_app::ports::downloads::DownloadError;

use crate::formats::NeedsFormatError;

/// Why a release's compatibility manifest could not be had.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ManifestError {
    /// The download failed.
    #[error("the manifest could not be downloaded")]
    Download(#[from] DownloadError),
    /// The downloaded file could not be read.
    #[error("the manifest could not be read")]
    Read(#[source] std::io::Error),
    /// The file is not a manifest.
    #[error("the manifest is invalid")]
    Invalid(#[from] NeedsFormatError),
}

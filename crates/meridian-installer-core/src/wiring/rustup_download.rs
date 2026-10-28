//! Fetching `rustup-init` through the downloader.

use std::path::Path;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::Cause;
use substrate_installer_app::ports::downloads::{DownloadRequest, Downloader, TransferControl};
use substrate_installer_domain::releases::HttpUrl;
use substrate_installer_process::RustupInitFetcher;

/// Fetches `rustup-init` with the engine's downloader.
#[derive(Debug)]
pub(super) struct RustupDownload<D> {
    downloader: D,
}

impl<D> RustupDownload<D> {
    pub(super) fn new(downloader: D) -> Self {
        Self { downloader }
    }
}

impl<D: Downloader> RustupInitFetcher for RustupDownload<D> {
    fn fetch(&self, url: &HttpUrl, destination: &Path, cancel: &CancelToken) -> Result<(), Cause> {
        let progress = |_| {};
        let control = TransferControl {
            cancel,
            progress: &progress,
        };
        let request = DownloadRequest {
            url,
            destination,
            sha256: None,
        };
        self.downloader
            .download(&request, &control)
            .map_err(|error| Box::new(error) as Cause)
    }
}

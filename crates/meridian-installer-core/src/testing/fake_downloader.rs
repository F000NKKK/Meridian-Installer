//! A downloader that serves files it was given.

use std::collections::BTreeMap;
use std::io::Write;

use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, Progress, TransferControl,
};

/// Serves the bytes it was given for each address; any other address fails.
#[derive(Debug, Default, Clone)]
pub struct FakeDownloader {
    files: BTreeMap<String, Vec<u8>>,
}

impl FakeDownloader {
    /// A downloader with nothing to serve.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The same, serving `bytes` at `url`.
    #[must_use]
    pub fn with(mut self, url: &str, bytes: Vec<u8>) -> Self {
        self.files.insert(url.to_owned(), bytes);
        self
    }

    fn bytes_of(&self, request: &DownloadRequest<'_>) -> Result<&[u8], DownloadError> {
        self.files
            .get(request.url.as_str())
            .map(Vec::as_slice)
            .ok_or_else(|| DownloadError::Failed {
                url: request.url.clone(),
                cause: "not served".into(),
            })
    }
}

impl Downloader for FakeDownloader {
    fn download(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
    ) -> Result<(), DownloadError> {
        control.cancel.check()?;
        let bytes = self.bytes_of(request)?;
        if let Some(folder) = request.destination.parent() {
            std::fs::create_dir_all(folder).map_err(DownloadError::ConsumerStopped)?;
        }
        std::fs::write(request.destination, bytes).map_err(DownloadError::ConsumerStopped)?;
        (control.progress)(Progress {
            done: bytes.len() as u64,
            total: Some(bytes.len() as u64),
        });
        Ok(())
    }

    fn stream(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
        tee: &mut dyn Write,
    ) -> Result<(), DownloadError> {
        control.cancel.check()?;
        let bytes = self.bytes_of(request)?;
        tee.write_all(bytes)
            .map_err(DownloadError::ConsumerStopped)?;
        (control.progress)(Progress {
            done: bytes.len() as u64,
            total: Some(bytes.len() as u64),
        });
        Ok(())
    }
}

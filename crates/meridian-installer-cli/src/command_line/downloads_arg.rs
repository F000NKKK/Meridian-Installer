//! `--downloads`.

use clap::ValueEnum;
use substrate_installer_app::installing::DownloadCache;

/// What happens to downloads once installed.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum DownloadsArg {
    /// Delete them.
    #[default]
    Discard,
    /// Keep them in the cache.
    Keep,
}

impl From<DownloadsArg> for DownloadCache {
    fn from(downloads: DownloadsArg) -> Self {
        match downloads {
            DownloadsArg::Discard => Self::Discard,
            DownloadsArg::Keep => Self::Keep,
        }
    }
}

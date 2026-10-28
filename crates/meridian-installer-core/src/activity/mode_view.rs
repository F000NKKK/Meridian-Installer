//! When downloads happen, as the window names it.

use serde::{Deserialize, Serialize};
use substrate_installer_domain::planning::Mode;

/// When downloads happen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModeView {
    /// Each package installs as soon as it is downloaded.
    #[default]
    InstallWhileDownloading,
    /// Everything downloads first, then installs.
    DownloadAllThenInstall,
}

impl From<ModeView> for Mode {
    fn from(mode: ModeView) -> Self {
        match mode {
            ModeView::InstallWhileDownloading => Self::InstallWhileDownloading,
            ModeView::DownloadAllThenInstall => Self::DownloadAllThenInstall,
        }
    }
}

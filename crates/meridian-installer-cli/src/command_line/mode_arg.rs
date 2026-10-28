//! `--mode`.

use clap::ValueEnum;
use substrate_installer_domain::planning::Mode;

/// When downloads happen.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ModeArg {
    /// Each package installs as soon as it is downloaded.
    #[default]
    WhileDownloading,
    /// Everything downloads first, then installs.
    DownloadFirst,
}

impl From<ModeArg> for Mode {
    fn from(mode: ModeArg) -> Self {
        match mode {
            ModeArg::WhileDownloading => Self::InstallWhileDownloading,
            ModeArg::DownloadFirst => Self::DownloadAllThenInstall,
        }
    }
}

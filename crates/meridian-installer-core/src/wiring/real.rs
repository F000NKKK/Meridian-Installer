//! The machine of the real adapters.

use std::sync::Arc;

use substrate_installer_archive::ArchiveUnpacker;
use substrate_installer_fs::{DiskFiles, HostProducts, LauncherDirs};
use substrate_installer_http::{HttpClient, HttpDownloader, HttpReleaseSource};
use substrate_installer_process::{
    CargoBin, ProcessSystemDetector, ProcessToolRunner, RustupBootstrap, host_platform,
};

use super::rustup_download::RustupDownload;
use super::{Machine, MachineConfig, MachineError};
use crate::formats::installed_records;

impl Machine {
    /// The machine of the real adapters: GitHub and crates.io over HTTP,
    /// the real file system under the home folder, rustup and cargo, and the
    /// platform's way of installing programs.
    ///
    /// # Errors
    ///
    /// [`MachineError`] when the system is not one the installer supports or
    /// the folders of cargo and the launchers are not known.
    pub fn real(config: &MachineConfig) -> Result<Self, MachineError> {
        let client = HttpClient::new(config.client_config());
        let downloader = HttpDownloader::new(client.clone());
        let unpacker: Arc<ArchiveUnpacker> = Arc::new(ArchiveUnpacker);
        let layout = config.home.layout();
        let products = HostProducts::for_host(
            layout.clone(),
            unpacker.clone(),
            LauncherDirs::under_user_home(&config.user_home),
        )
        .ok_or(MachineError::UnsupportedSystem)?;
        let bin = CargoBin::from_environment().ok_or(MachineError::NoUserHome)?;
        let platform = host_platform().ok_or(MachineError::UnsupportedSystem)?;
        let bootstrap = RustupBootstrap::new(
            Box::new(RustupDownload::new(downloader.clone())),
            config.home.root().join("staging"),
        );
        Ok(Self {
            releases: Box::new(HttpReleaseSource::new(client)),
            downloader: Box::new(downloader),
            unpacker,
            layout: Box::new(layout),
            files: Box::new(DiskFiles),
            store: Box::new(installed_records(config.home.installed_file())),
            tools: Box::new(
                ProcessToolRunner::new(bin.clone(), platform).with_bootstrap(bootstrap),
            ),
            products: Box::new(products),
            detector: Box::new(ProcessSystemDetector::new(bin)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::folder::MeridianHome;

    #[test]
    fn the_real_machine_is_made_from_a_config() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config = MachineConfig {
            home: MeridianHome::new(dir.path().join("home")),
            user_home: dir.path().to_path_buf(),
            token: None,
        };
        let machine = Machine::real(&config).expect("a supported system");
        let sink = |_| {};
        let _adapters = machine.adapters(&sink);
    }
}

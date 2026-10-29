//! First-run installation through Substrate's platform adapter.

use std::path::PathBuf;
use std::sync::Arc;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::records::InstalledStore;
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_archive::ArchiveUnpacker;
use substrate_installer_domain::channel::Installer;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Installed;
use substrate_installer_domain::versions::Version;
use substrate_installer_fs::{HostProducts, LauncherDirs, RunningBundle, RunningProduct};

use super::{MachineConfig, SelfInstallError};
use crate::formats::installed_records;

/// Retains the running Installer and records its installed path. A newer
/// installed version is preserved, so opening an old download never downgrades it.
/// No network access or administrator rights are needed.
///
/// # Errors
///
/// [`SelfInstallError`] if the platform, bundle, launcher or record is unavailable.
pub fn install_running(
    config: &MachineConfig,
    installer: &Installer,
    version: &Version,
    bundle: RunningBundle<'_>,
) -> Result<PathBuf, SelfInstallError> {
    let store = installed_records(config.home.installed_file());
    let mut records = store.read()?;
    let id = ItemId::Product(installer.id.clone());
    if let Some(newer) = records.newest(&id)
        && newer.version.compare(version).is_gt()
        && newer.path.exists()
    {
        return Ok(newer.path.clone());
    }
    let products = HostProducts::for_host(
        config.home.layout(),
        Arc::new(ArchiveUnpacker),
        LauncherDirs::under_user_home(&config.user_home),
    )
    .ok_or(SelfInstallError::UnsupportedSystem)?;
    let cancel = CancelToken::new();
    let control = ToolControl {
        cancel: &cancel,
        output: &|_| {},
    };
    let request = RunningProduct {
        installer,
        version,
        bundle,
    };
    let path = products.place_running(&request, &control)?;
    records.put(Installed {
        id,
        version: version.clone(),
        path: path.clone(),
    });
    store.write(&records)?;
    Ok(path)
}

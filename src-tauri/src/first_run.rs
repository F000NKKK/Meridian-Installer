//! Retaining release bundles on first launch, without touching development builds.

use std::path::PathBuf;

use meridian_installer_core::activity::describe;
use meridian_installer_core::wiring::{MachineConfig, install_running};
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::versions::Version;
use substrate_installer_fs::RunningBundle;

pub(crate) fn install(
    config: &MachineConfig,
    catalog: &Catalog,
    version: &Version,
) -> Result<(), String> {
    if cfg!(debug_assertions) {
        return Ok(());
    }
    let Some(installer) = catalog.installer() else {
        return Ok(());
    };
    let executable = std::env::current_exe().map_err(|error| describe(&error))?;
    let source = bundle_path(&executable);
    let bundle = match () {
        () if cfg!(target_os = "linux") => source.as_deref().map(RunningBundle::AppImage),
        () if cfg!(target_os = "macos") => source.as_deref().map(RunningBundle::Application),
        () if cfg!(target_os = "windows") => Some(RunningBundle::InstalledProgram(&executable)),
        () => None,
    };
    if let Some(bundle) = bundle {
        install_running(config, installer, version, bundle).map_err(|error| describe(&error))?;
    }
    Ok(())
}

fn bundle_path(executable: &std::path::Path) -> Option<PathBuf> {
    if cfg!(target_os = "linux") {
        return std::env::var_os("APPIMAGE")
            .filter(|p| !p.is_empty())
            .map(PathBuf::from);
    }
    executable
        .ancestors()
        .find(|p| p.extension().is_some_and(|e| e == "app"))
        .map(std::path::Path::to_path_buf)
}

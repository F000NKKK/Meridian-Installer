//! First-run installation records its retained copy and never downgrades it.
#![cfg(target_os = "linux")]

use meridian_installer_core::catalog::built_in;
use meridian_installer_core::folder::MeridianHome;
use meridian_installer_core::formats::installed_records;
use meridian_installer_core::wiring::{MachineConfig, install_running};
use substrate_installer_app::ports::records::InstalledStore;
use substrate_installer_domain::versions::Version;
use substrate_installer_fs::RunningBundle;

#[test]
fn first_launch_retains_the_bundle_and_saves_its_installed_path() {
    let scratch = tempfile::tempdir().unwrap();
    let config = MachineConfig {
        home: MeridianHome::new(scratch.path().join("home")),
        user_home: scratch.path().join("user"),
        token: None,
    };
    let catalog = built_in().unwrap();
    let installer = catalog.installer().unwrap();
    let version = Version::new("0.1.0").unwrap();
    let bundle = scratch.path().join("Meridian_Installer.AppImage");
    std::fs::write(&bundle, b"complete bundle").unwrap();

    let retained = install_running(
        &config,
        installer,
        &version,
        RunningBundle::AppImage(&bundle),
    )
    .unwrap();

    let records = installed_records(config.home.installed_file())
        .read()
        .unwrap();
    assert_eq!(records.records().len(), 1);
    assert_eq!(records.records()[0].path, retained);
    assert_eq!(std::fs::read(&retained).unwrap(), b"complete bundle");
    let repeated = install_running(
        &config,
        installer,
        &version,
        RunningBundle::AppImage(&retained),
    )
    .unwrap();
    assert_eq!(repeated, retained);
    assert_eq!(
        installed_records(config.home.installed_file())
            .read()
            .unwrap()
            .records()
            .len(),
        1
    );
}

#[test]
fn opening_an_old_download_does_not_downgrade_the_installed_bundle() {
    let scratch = tempfile::tempdir().unwrap();
    let config = MachineConfig {
        home: MeridianHome::new(scratch.path().join("home")),
        user_home: scratch.path().join("user"),
        token: None,
    };
    let catalog = built_in().unwrap();
    let installer = catalog.installer().unwrap();
    let bundle = scratch.path().join("Installer.AppImage");
    std::fs::write(&bundle, b"newer").unwrap();
    let newer = install_running(
        &config,
        installer,
        &Version::new("0.2.0").unwrap(),
        RunningBundle::AppImage(&bundle),
    )
    .unwrap();
    std::fs::write(&bundle, b"older").unwrap();

    let retained = install_running(
        &config,
        installer,
        &Version::new("0.1.0").unwrap(),
        RunningBundle::AppImage(&bundle),
    )
    .unwrap();

    assert_eq!(retained, newer);
    assert_eq!(std::fs::read(retained).unwrap(), b"newer");
}

//! Making the operations of this process.

use std::sync::Arc;

use meridian_installer_core::activity::{Operations, describe};
use meridian_installer_core::catalog::Loaded;
use meridian_installer_core::engine::Session;
use meridian_installer_core::host::detect;
use meridian_installer_core::wiring::{Machine, MachineConfig};
use substrate_installer_domain::versions::Version;

/// The operations of this process: the real machine, the built-in catalog or
/// the newer one saved, on this host. The error is a sentence for the window.
pub(crate) fn operations() -> Result<Operations, String> {
    let config = MachineConfig::from_environment(None).map_err(|e| describe(&e))?;
    let machine = Machine::real(&config).map_err(|e| describe(&e))?;
    let loaded = Loaded::from_home(&config.home).map_err(|e| describe(&e))?;
    let mut notices: Vec<String> = loaded.notice.iter().map(|n| describe(n)).collect();
    let host = detect().map_err(|e| describe(&e))?;
    let version = Version::new(env!("CARGO_PKG_VERSION")).map_err(|e| describe(&e))?;
    if let Err(error) = crate::first_run::install(&config, &loaded.catalog, &version) {
        notices.push(error);
    }
    let session = Session::new(Arc::new(machine), config.home, loaded.catalog, host)
        .with_running_version(version);
    Ok(Operations::new(session, notices))
}

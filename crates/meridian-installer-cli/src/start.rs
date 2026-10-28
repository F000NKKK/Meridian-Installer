//! Making the session of this process.

use std::sync::Arc;

use meridian_installer_core::catalog::Loaded;
use meridian_installer_core::engine::Session;
use meridian_installer_core::host::detect;
use meridian_installer_core::wiring::{Machine, MachineConfig};
use substrate_installer_app::control::CancelToken;
use substrate_installer_domain::versions::Version;

use crate::CliError;
use crate::Started;
use crate::command_line::Cli;

/// The session of this process: the real machine, the built-in catalog or the
/// newer one saved, refreshed from the latest Installer release unless
/// `--no-refresh`.
///
/// # Errors
///
/// [`CliError`] when the machine, the host or the built-in catalog cannot be had.
pub fn start(cli: &Cli) -> Result<Started, CliError> {
    let config = MachineConfig::from_environment(cli.home.clone())?;
    let machine = Machine::real(&config)?;
    let loaded = Loaded::from_home(&config.home)?;
    let mut notices: Vec<String> = loaded.notice.iter().map(ToString::to_string).collect();
    let mut session = Session::new(Arc::new(machine), config.home, loaded.catalog, detect()?)
        .with_running_version(Version::new(env!("CARGO_PKG_VERSION"))?);
    if !cli.no_refresh
        && let Err(error) = session.refresh_catalog(&CancelToken::new())
    {
        notices.push(format!("the catalog was not refreshed: {error}"));
    }
    Ok(Started { session, notices })
}

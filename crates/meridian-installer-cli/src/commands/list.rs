//! `list`.

use meridian_installer_core::display::Snapshot;

use crate::render::{write_json, write_listing};
use crate::{CliError, Context};

/// Lists the catalog with what is installed and newest.
pub(crate) fn list(context: &mut Context<'_>) -> Result<(), CliError> {
    let snapshot = snapshot_of(context)?;
    if context.json {
        write_json(context.out, &snapshot)?;
    } else {
        write_listing(context.out, &snapshot)?;
    }
    Ok(())
}

/// The snapshot of the session now.
pub(super) fn snapshot_of(context: &Context<'_>) -> Result<Snapshot, CliError> {
    let session = context.session;
    let available = session.available();
    let installed = session.installed()?;
    Ok(Snapshot::of(
        session,
        &installed,
        &available,
        context.notices.to_vec(),
    ))
}

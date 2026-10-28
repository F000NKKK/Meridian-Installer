//! `status`.

use super::list::snapshot_of;
use crate::render::{write_json, write_status};
use crate::{CliError, Context};

/// Shows what is installed and whether it is up to date.
pub(crate) fn status(context: &mut Context<'_>) -> Result<(), CliError> {
    let snapshot = snapshot_of(context)?;
    if context.json {
        write_json(context.out, &snapshot)?;
    } else {
        write_status(context.out, &snapshot)?;
    }
    Ok(())
}

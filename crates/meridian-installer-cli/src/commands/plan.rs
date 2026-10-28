//! `plan`.

use super::planned::{check_and_show, plan_targets};
use crate::command_line::TargetArgs;
use crate::{CliError, Context};

/// Shows what installing the targets would do, and installs nothing.
pub(crate) fn plan(context: &mut Context<'_>, args: &TargetArgs) -> Result<(), CliError> {
    let available = context.session.available();
    let installed = context.session.installed()?;
    let planned = plan_targets(context, args, &installed, &available)?;
    check_and_show(context, &planned, &installed, &available)
}

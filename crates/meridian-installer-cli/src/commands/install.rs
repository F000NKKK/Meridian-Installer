//! `install`.

use super::planned::{apply, check_and_show, plan_targets};
use crate::command_line::InstallArgs;
use crate::{CliError, Context};

/// Installs the targets, after showing the plan; nothing is installed when
/// the plan does not fit what the releases need.
pub(crate) fn install(context: &mut Context<'_>, args: &InstallArgs) -> Result<(), CliError> {
    let available = context.session.available();
    let installed = context.session.installed()?;
    let planned = plan_targets(context, &args.targets, &installed, &available)?;
    check_and_show(context, &planned, &installed, &available)?;
    if planned.plan.is_empty() {
        return Ok(());
    }
    apply(context, &planned, &args.apply)
}

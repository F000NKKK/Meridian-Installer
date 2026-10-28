//! `update`.

use super::planned::{Planned, apply, check_and_show};
use crate::command_line::UpdateArgs;
use crate::target::Target;
use crate::{CliError, Context};

/// Updates what the installer installed: the named items, or all of them.
/// The Installer itself goes first.
pub(crate) fn update(context: &mut Context<'_>, args: &UpdateArgs) -> Result<(), CliError> {
    let session = context.session;
    let only = args
        .targets
        .iter()
        .map(|name| {
            Target::resolve(name, session.catalog())?
                .item()
                .ok_or_else(|| CliError::NotInCatalog(name.as_str().to_owned()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let available = session.available();
    let installed = session.installed()?;
    let planned = Planned {
        plan: session.plan_update(&only, &installed, &available),
        selection: None,
    };
    check_and_show(context, &planned, &installed, &available)?;
    if planned.plan.is_empty() {
        return Ok(());
    }
    apply(context, &planned, &args.apply)
}

//! `remove`.

use substrate_installer_app::installing::ApplyOptions;
use substrate_installer_app::listing::Available;
use substrate_installer_app::plans::for_removal;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::planning::Plan;

use super::planned::{Planned, check_and_show};
use crate::command_line::RemoveArgs;
use crate::target::Target;
use crate::{CliError, Context};

/// Removes components and products the installer installed, after showing
/// the plan. Refuses what another installed component requires, unless
/// `--force`.
pub(crate) fn remove(context: &mut Context<'_>, args: &RemoveArgs) -> Result<(), CliError> {
    let session = context.session;
    let ids = args
        .targets
        .iter()
        .map(|name| {
            Target::resolve(name, session.catalog())?
                .item()
                .ok_or_else(|| CliError::NotInCatalog(name.as_str().to_owned()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let installed = session.installed()?;
    if !args.force {
        refuse_required(context, &ids, &installed)?;
    }
    let mut plan = Plan::default();
    for id in &ids {
        plan.changes.extend(for_removal(id, &installed).changes);
    }
    let planned = Planned {
        plan,
        selection: None,
    };
    check_and_show(context, &planned, &installed, &Available::new())?;
    if planned.plan.is_empty() {
        return Ok(());
    }
    session.apply(
        &planned.plan,
        ApplyOptions::default(),
        context.cancel,
        context.events,
    )?;
    Ok(())
}

/// Fails when an installed component outside `ids` requires one of them.
fn refuse_required(
    context: &Context<'_>,
    ids: &[ItemId],
    installed: &[Found],
) -> Result<(), CliError> {
    let catalog = context.session.catalog();
    for found in installed.iter().filter(|found| found.is_managed()) {
        let ItemId::Component(own) = &found.id else {
            continue;
        };
        let Some(component) = catalog.component(own) else {
            continue;
        };
        let leaving = ids.contains(&found.id);
        let blocker = component
            .requires
            .iter()
            .find(|required| ids.contains(&ItemId::Component((*required).clone())));
        if let (false, Some(required)) = (leaving, blocker) {
            return Err(CliError::StillRequired {
                id: required.to_string(),
                by: own.to_string(),
            });
        }
    }
    Ok(())
}

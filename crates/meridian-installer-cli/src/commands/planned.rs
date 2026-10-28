//! Planning what the command line names, and showing the plan.

use meridian_installer_core::display::PlanView;
use substrate_installer_app::installing::ApplyOptions;
use substrate_installer_app::listing::Available;
use substrate_installer_app::plans::{Situation, for_program};
use substrate_installer_domain::changes::Change;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::planning::Plan;

use crate::command_line::{ApplyArgs, TargetArgs};
use crate::render::{write_json, write_plan};
use crate::target::Target;
use crate::{CliError, Context};

/// A plan, and the selection it was made from.
#[derive(Debug)]
pub(super) struct Planned {
    pub(super) plan: Plan,
    pub(super) selection: Option<Selection>,
}

/// The plan that installs the targets: the components of the workloads and
/// the components named, with what they require, and the products at their
/// newest. Nothing installed is removed.
pub(super) fn plan_targets(
    context: &Context<'_>,
    args: &TargetArgs,
    installed: &[Found],
    available: &Available,
) -> Result<Planned, CliError> {
    let session = context.session;
    let mut selection = Selection::default();
    let mut products = Vec::new();
    for name in &args.targets {
        match Target::resolve(name, session.catalog())? {
            Target::Workload(id) => selection.workloads.insert(id),
            Target::Component(id) => selection.checked.insert(id),
            Target::Product(id) => {
                products.push(ItemId::Product(id));
                continue;
            }
        };
    }
    selection.unchecked.extend(args.without.iter().cloned());
    let mut plan = session.plan_selection(&selection, installed, available);
    plan.changes
        .retain(|change| !matches!(change, Change::Remove { .. }));
    for id in &products {
        let own = for_program(
            id,
            &Situation {
                installed,
                available,
            },
        );
        plan.changes.extend(own.changes);
        plan.releases.extend(own.releases);
    }
    Ok(Planned {
        plan,
        selection: Some(selection),
    })
}

/// Checks `planned` against what its releases need and shows it. Fails when a
/// need is not met.
pub(super) fn check_and_show(
    context: &mut Context<'_>,
    planned: &Planned,
    installed: &[Found],
    available: &Available,
) -> Result<(), CliError> {
    let session = context.session;
    let compatibility = session.compatibility(&planned.plan, installed, available, context.cancel);
    let view = PlanView::of(
        session,
        &planned.plan,
        planned.selection.as_ref(),
        &compatibility,
    );
    if context.json {
        write_json(context.out, &view)?;
    } else {
        write_plan(context.out, &view)?;
    }
    match view.conflicts.len() {
        0 => Ok(()),
        count => Err(CliError::Conflicts(count)),
    }
}

/// Carries out `planned` the way `args` say.
pub(super) fn apply(
    context: &Context<'_>,
    planned: &Planned,
    args: &ApplyArgs,
) -> Result<(), CliError> {
    let options = ApplyOptions {
        mode: args.mode.into(),
        downloads: args.downloads.into(),
    };
    context
        .session
        .apply(&planned.plan, options, context.cancel, context.events)?;
    Ok(())
}

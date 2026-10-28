//! Writing answers as text.

use std::io::Write;

use meridian_installer_core::display::{ChangeView, PlanView, Snapshot};
use serde::Serialize;

use crate::CliError;
use crate::format_bytes::format_bytes;

/// What is shown where a version is not known.
const NONE: &str = "-";

/// Writes `value` as JSON, and a newline.
pub(crate) fn write_json(out: &mut dyn Write, value: &impl Serialize) -> Result<(), CliError> {
    serde_json::to_writer_pretty(&mut *out, value)?;
    writeln!(out)?;
    Ok(())
}

/// Writes a plan: what it does, how much it downloads, what does not fit.
pub(crate) fn write_plan(out: &mut dyn Write, plan: &PlanView) -> Result<(), CliError> {
    if plan.changes.is_empty() {
        writeln!(out, "Nothing to do.")?;
    } else {
        let size = format_bytes(plan.download_bytes);
        writeln!(out, "{} change(s), {size} to download:", plan.changes.len())?;
    }
    for change in &plan.changes {
        match change {
            ChangeView::Install { id, version } => writeln!(out, "  install {id} {version}")?,
            ChangeView::Update { id, from, to } => writeln!(out, "  update  {id} {from} -> {to}")?,
            ChangeView::Remove { id, version } => writeln!(out, "  remove  {id} {version}")?,
        }
    }
    for conflict in &plan.conflicts {
        let found = conflict.found.as_deref().unwrap_or("not installed");
        writeln!(
            out,
            "  conflict: {} {} needs {} {} ({found})",
            conflict.component, conflict.version, conflict.needs, conflict.requirement
        )?;
    }
    for note in &plan.unread {
        writeln!(out, "  note: {note}")?;
    }
    Ok(())
}

/// Writes the catalog: products with their workloads, then the components by group.
pub(crate) fn write_listing(out: &mut dyn Write, snapshot: &Snapshot) -> Result<(), CliError> {
    write_notices(out, snapshot)?;
    for product in &snapshot.products {
        writeln!(
            out,
            "{} ({}) installed {} latest {}",
            product.name,
            product.id,
            product.installed.as_deref().unwrap_or(NONE),
            product.latest.as_deref().unwrap_or(NONE)
        )?;
        for id in &product.workloads {
            if let Some(workload) = snapshot.workloads.iter().find(|own| &own.id == id) {
                writeln!(
                    out,
                    "  workload {} ({}): {}",
                    workload.name, workload.id, workload.description
                )?;
            }
        }
    }
    for group in &snapshot.groups {
        writeln!(out, "{}", group.name)?;
        for component in snapshot
            .components
            .iter()
            .filter(|own| own.group == group.id)
        {
            writeln!(
                out,
                "  {:<20} {:<28} installed {:<8} latest {}",
                component.id,
                component.name,
                component.installed.as_deref().unwrap_or(NONE),
                component.latest.as_deref().unwrap_or(NONE)
            )?;
        }
    }
    Ok(())
}

/// Writes what is installed, and whether each is up to date.
pub(crate) fn write_status(out: &mut dyn Write, snapshot: &Snapshot) -> Result<(), CliError> {
    write_notices(out, snapshot)?;
    if let Some(next) = &snapshot.installer_update {
        writeln!(
            out,
            "Meridian Installer {}: update available ({next})",
            snapshot.version
        )?;
    }
    let lines = snapshot
        .products
        .iter()
        .map(|p| (&p.id, &p.installed, &p.latest))
        .chain(
            snapshot
                .components
                .iter()
                .map(|c| (&c.id, &c.installed, &c.latest)),
        );
    let mut any = false;
    for (id, installed, latest) in lines {
        let Some(own) = installed else {
            continue;
        };
        any = true;
        let state = match latest {
            Some(newest) if newest != own => format!("update available ({newest})"),
            _ => "up to date".to_owned(),
        };
        writeln!(out, "{id} {own}: {state}")?;
    }
    if !any {
        writeln!(out, "Nothing is installed.")?;
    }
    Ok(())
}

fn write_notices(out: &mut dyn Write, snapshot: &Snapshot) -> Result<(), CliError> {
    for problem in &snapshot.problems {
        writeln!(out, "warning: {problem}")?;
    }
    Ok(())
}

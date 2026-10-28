//! A plan, for the Modify window and the terminal.

use std::collections::BTreeMap;

use serde::Serialize;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::planning::Plan;

use super::{ChangeView, ConflictView, WhyView};
use crate::engine::{Compatibility, Session};

/// What a plan does, why, how much it downloads and what does not fit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanView {
    /// The changes, in order.
    pub changes: Vec<ChangeView>,
    /// Why each component of the selection is in it.
    pub reasons: BTreeMap<String, WhyView>,
    /// Bytes to download.
    pub download_bytes: u64,
    /// Needs the machine will not meet.
    pub conflicts: Vec<ConflictView>,
    /// Releases whose manifest could not be read.
    pub unread: Vec<String>,
}

impl PlanView {
    /// The view of `plan` made for `selection` (none for a plan made without
    /// one), with what `compatibility` found.
    #[must_use]
    pub fn of(
        session: &Session,
        plan: &Plan,
        selection: Option<&Selection>,
        compatibility: &Compatibility,
    ) -> Self {
        let reasons = selection
            .map(|own| own.reasons(session.catalog()))
            .unwrap_or_default();
        Self {
            changes: plan.changes.iter().map(ChangeView::from).collect(),
            reasons: reasons
                .iter()
                .map(|(id, why)| (id.to_string(), WhyView::from(why)))
                .collect(),
            download_bytes: plan.download_size(session.catalog(), session.host()),
            conflicts: compatibility
                .conflicts
                .iter()
                .map(ConflictView::from)
                .collect(),
            unread: compatibility
                .unread
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }
}

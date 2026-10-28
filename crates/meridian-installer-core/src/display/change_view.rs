//! One change of a plan.

use serde::Serialize;
use substrate_installer_domain::changes::Change;

/// One thing a plan does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ChangeView {
    /// Install `id` at `version`.
    Install {
        /// What.
        id: String,
        /// The version.
        version: String,
    },
    /// Replace `from` with `to`.
    Update {
        /// What.
        id: String,
        /// The installed version.
        from: String,
        /// The new version.
        to: String,
    },
    /// Remove `id` at `version`.
    Remove {
        /// What.
        id: String,
        /// The version.
        version: String,
    },
}

impl From<&Change> for ChangeView {
    fn from(change: &Change) -> Self {
        match change {
            Change::Install { id, version } => Self::Install {
                id: id.to_string(),
                version: version.to_string(),
            },
            Change::Update { id, from, to } => Self::Update {
                id: id.to_string(),
                from: from.to_string(),
                to: to.to_string(),
            },
            Change::Remove { id, version } => Self::Remove {
                id: id.to_string(),
                version: version.to_string(),
            },
        }
    }
}

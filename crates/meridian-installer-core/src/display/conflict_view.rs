//! A need that is not met.

use serde::Serialize;
use substrate_installer_domain::compat::Conflict;

/// A need of a release that the machine will not meet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConflictView {
    /// Who needs it.
    pub component: String,
    /// Its version that will be installed.
    pub version: String,
    /// What it needs.
    pub needs: String,
    /// The versions it works with.
    pub requirement: String,
    /// The version there will be.
    pub found: Option<String>,
}

impl From<&Conflict> for ConflictView {
    fn from(conflict: &Conflict) -> Self {
        Self {
            component: conflict.component.to_string(),
            version: conflict.version.to_string(),
            needs: conflict.needs.to_string(),
            requirement: conflict.requirement.to_string(),
            found: conflict.found.as_ref().map(ToString::to_string),
        }
    }
}

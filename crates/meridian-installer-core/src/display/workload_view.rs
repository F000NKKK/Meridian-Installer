//! A workload.

use serde::Serialize;
use substrate_installer_domain::channel::Workload;
use substrate_installer_domain::ids::ComponentId;

/// A workload: a named set of components for one kind of work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkloadView {
    /// Its id.
    pub id: String,
    /// Its name.
    pub name: String,
    /// What it is for.
    pub description: String,
    /// Its icon.
    pub icon: Option<String>,
    /// Always installed with it.
    pub required: Vec<String>,
    /// Checked by default.
    pub recommended: Vec<String>,
    /// Unchecked by default.
    pub optional: Vec<String>,
}

impl From<&Workload> for WorkloadView {
    fn from(workload: &Workload) -> Self {
        let ids = |list: &[ComponentId]| list.iter().map(ToString::to_string).collect();
        Self {
            id: workload.id.to_string(),
            name: workload.name.to_string(),
            description: workload.description.to_string(),
            icon: workload.icon.as_ref().map(ToString::to_string),
            required: ids(&workload.required),
            recommended: ids(&workload.recommended),
            optional: ids(&workload.optional),
        }
    }
}

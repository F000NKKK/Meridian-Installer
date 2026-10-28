//! Why a component is in a selection.

use serde::Serialize;
use substrate_installer_domain::choice::Why;

/// Why a component is in a selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WhyView {
    /// A selected workload requires it.
    Required {
        /// The workload.
        workload: String,
    },
    /// A selected workload recommends it.
    Recommended {
        /// The workload.
        workload: String,
    },
    /// Checked by itself.
    Chosen,
    /// Another component requires it.
    RequiredBy {
        /// The component.
        component: String,
    },
}

impl From<&Why> for WhyView {
    fn from(why: &Why) -> Self {
        match why {
            Why::Required { workload } => Self::Required {
                workload: workload.to_string(),
            },
            Why::Recommended { workload } => Self::Recommended {
                workload: workload.to_string(),
            },
            Why::Chosen => Self::Chosen,
            Why::RequiredBy { component } => Self::RequiredBy {
                component: component.to_string(),
            },
        }
    }
}

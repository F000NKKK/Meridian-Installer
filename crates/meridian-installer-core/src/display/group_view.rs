//! A group of components.

use serde::Serialize;
use substrate_installer_domain::channel::Group;

/// A group the individual components are listed under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GroupView {
    /// Its id.
    pub id: String,
    /// Its name.
    pub name: String,
}

impl From<&Group> for GroupView {
    fn from(group: &Group) -> Self {
        Self {
            id: group.id.to_string(),
            name: group.name.to_string(),
        }
    }
}

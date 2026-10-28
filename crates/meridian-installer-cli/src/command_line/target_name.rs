//! A name given to a command.

use std::str::FromStr;

use substrate_installer_domain::ids::{ComponentId, IdError};

/// A name in the catalog's namespace: a workload, a component or a product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetName(String);

impl TargetName {
    /// The name as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for TargetName {
    type Err = IdError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // Every kind of id has the grammar of a component's.
        ComponentId::new(text).map(|id| Self(id.to_string()))
    }
}

//! What a name in a command line names.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::ids::{ComponentId, ItemId, ProductId, WorkloadId};

use crate::CliError;
use crate::command_line::TargetName;

/// What a name in the catalog's namespace names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Target {
    /// A workload: its components are installed.
    Workload(WorkloadId),
    /// A component.
    Component(ComponentId),
    /// A product, or the Installer itself.
    Product(ProductId),
}

impl Target {
    /// What `name` names in `catalog`.
    pub(crate) fn resolve(name: &TargetName, catalog: &Catalog) -> Result<Self, CliError> {
        let text = name.as_str();
        let not_found = || CliError::NotInCatalog(text.to_owned());
        let workload = WorkloadId::new(text)?;
        if catalog.workload(&workload).is_some() {
            return Ok(Self::Workload(workload));
        }
        let product = ProductId::new(text)?;
        if catalog.item(&ItemId::Product(product.clone())).is_some() {
            return Ok(Self::Product(product));
        }
        let component = ComponentId::new(text)?;
        catalog
            .component(&component)
            .map(|_| Self::Component(component))
            .ok_or_else(not_found)
    }

    /// The item it names, when it names one that can be installed alone.
    pub(crate) fn item(&self) -> Option<ItemId> {
        match self {
            Self::Workload(_) => None,
            Self::Component(id) => Some(ItemId::Component(id.clone())),
            Self::Product(id) => Some(ItemId::Product(id.clone())),
        }
    }
}

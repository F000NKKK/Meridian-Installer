//! Everything the window shows.

use serde::Serialize;
use substrate_installer_app::listing::Available;
use substrate_installer_app::plans::{Situation, for_program};
use substrate_installer_domain::inventory::Found;

use super::{ComponentView, GroupView, ProductView, WorkloadView};
use crate::engine::Session;

/// Everything the window shows: the catalog with what is installed and what
/// is newest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    /// The running Installer's version.
    pub version: String,
    /// The home folder.
    pub home: String,
    /// The products.
    pub products: Vec<ProductView>,
    /// The workloads.
    pub workloads: Vec<WorkloadView>,
    /// The components.
    pub components: Vec<ComponentView>,
    /// The groups.
    pub groups: Vec<GroupView>,
    /// The version of the Installer that a self-update would install.
    pub installer_update: Option<String>,
    /// What could not be asked, and other things to tell.
    pub problems: Vec<String>,
}

impl Snapshot {
    /// The snapshot of `session` with what is `installed` and `available`;
    /// `notices` are told along with the sources that failed.
    #[must_use]
    pub fn of(
        session: &Session,
        installed: &[Found],
        available: &Available,
        notices: Vec<String>,
    ) -> Self {
        let catalog = session.catalog();
        let host = session.host();
        let mut problems = notices;
        problems.extend(available.problems().iter().map(ToString::to_string));
        Self {
            version: session
                .running_version()
                .map_or_else(String::new, ToString::to_string),
            home: session.home().root().display().to_string(),
            products: catalog
                .products()
                .iter()
                .map(|product| ProductView::of(product, installed, available, host))
                .collect(),
            workloads: catalog.workloads().iter().map(WorkloadView::from).collect(),
            components: catalog
                .components()
                .iter()
                .map(|component| ComponentView::of(component, installed, available, host))
                .collect(),
            groups: catalog.groups().iter().map(GroupView::from).collect(),
            installer_update: installer_update(session, installed, available),
            problems,
        }
    }
}

/// The newest Installer's version, when it is newer than the running one.
fn installer_update(
    session: &Session,
    installed: &[Found],
    available: &Available,
) -> Option<String> {
    let id =
        substrate_installer_domain::ids::ItemId::Product(session.catalog().installer()?.id.clone());
    let plan = for_program(
        &id,
        &Situation {
            installed,
            available,
        },
    );
    let change = plan.changes.first()?;
    match change {
        substrate_installer_domain::changes::Change::Update { to, .. } => Some(to.to_string()),
        _ => None,
    }
}

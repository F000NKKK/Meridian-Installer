//! A component's line.

use serde::Serialize;
use substrate_installer_app::listing::Available;
use substrate_installer_domain::channel::{Component, Item};
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::platforms::Host;

/// A component's line in the Modify window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentView {
    /// Its id.
    pub id: String,
    /// Its name.
    pub name: String,
    /// What it is.
    pub description: String,
    /// The group it is listed under.
    pub group: String,
    /// The installed version.
    pub installed: Option<String>,
    /// Whether the installer put it there (it may update and remove it).
    pub managed: bool,
    /// The newest version the host can install.
    pub latest: Option<String>,
    /// Bytes to download for the newest.
    pub size: Option<u64>,
}

impl ComponentView {
    /// The line of `component`, from what is `installed` and `available` on `host`.
    #[must_use]
    pub fn of(
        component: &Component,
        installed: &[Found],
        available: &Available,
        host: &Host,
    ) -> Self {
        let own = installed
            .iter()
            .filter(|found| found.is_component(&component.id))
            .max_by(|a, b| a.version.compare(&b.version));
        let release = available.release(&ItemId::Component(component.id.clone()));
        Self {
            id: component.id.to_string(),
            name: component.name.to_string(),
            description: component.description.to_string(),
            group: component.group.to_string(),
            installed: own.map(|found| found.version.to_string()),
            managed: own.is_some_and(Found::is_managed),
            latest: release.map(|release| release.version.to_string()),
            size: release.and_then(|r| Item::Component(component).source().asset(r, host)?.size),
        }
    }
}

//! A product card.

use serde::Serialize;
use substrate_installer_app::listing::Available;
use substrate_installer_domain::channel::{Item, Product};
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::platforms::Host;

/// A product's card: what it is, what is installed and what is newest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductView {
    /// Its id.
    pub id: String,
    /// Its name.
    pub name: String,
    /// What it is.
    pub description: String,
    /// The installed version.
    pub installed: Option<String>,
    /// The newest version the host can install.
    pub latest: Option<String>,
    /// Whether the newest is newer than the installed one.
    pub update_available: bool,
    /// The newest release's notes.
    pub notes_url: Option<String>,
    /// The program that starts it.
    pub program: String,
    /// Bytes to download for the newest.
    pub size: Option<u64>,
    /// Its icon's address.
    pub icon: Option<String>,
    /// Its workloads.
    pub workloads: Vec<String>,
}

impl ProductView {
    /// The card of `product`, from what is `installed` and `available` on `host`.
    #[must_use]
    pub fn of(product: &Product, installed: &[Found], available: &Available, host: &Host) -> Self {
        let id = ItemId::Product(product.id.clone());
        let own = installed
            .iter()
            .filter(|found| found.id == id)
            .max_by(|a, b| a.version.compare(&b.version));
        let release = available.release(&id);
        let update_available = own
            .zip(release)
            .is_some_and(|(own, release)| release.version.compare(&own.version).is_gt());
        Self {
            id: id.to_string(),
            name: product.name.to_string(),
            description: product.description.to_string(),
            installed: own.map(|found| found.version.to_string()),
            latest: release.map(|release| release.version.to_string()),
            update_available,
            notes_url: release
                .and_then(|r| r.notes_url.as_ref())
                .map(ToString::to_string),
            program: product.program.to_string(),
            size: release.and_then(|r| Item::Product(product).source().asset(r, host)?.size),
            icon: product.icon.as_ref().map(ToString::to_string),
            workloads: product.workloads.iter().map(ToString::to_string).collect(),
        }
    }
}

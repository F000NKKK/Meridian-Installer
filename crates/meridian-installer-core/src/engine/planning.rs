//! Making plans.

use std::collections::BTreeMap;

use substrate_installer_app::listing::Available;
use substrate_installer_app::plans::{Situation, for_selection};
use substrate_installer_domain::changes::derive_item;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::planning::Plan;

use super::Session;

impl Session {
    /// The plan that makes `selection` installed (see the engine's
    /// `for_selection`).
    #[must_use]
    pub fn plan_selection(
        &self,
        selection: &Selection,
        installed: &[Found],
        available: &Available,
    ) -> Plan {
        for_selection(
            &self.catalog,
            selection,
            &Situation {
                installed,
                available,
            },
        )
    }

    /// The plan that updates what the installer put on the machine: the
    /// items in `only`, or all of them when `only` is empty. The Installer
    /// comes first. Nothing is installed and nothing is removed.
    #[must_use]
    pub fn plan_update(&self, only: &[ItemId], installed: &[Found], available: &Available) -> Plan {
        let mut newest_installed: BTreeMap<&ItemId, &Found> = BTreeMap::new();
        for found in installed.iter().filter(|found| found.is_managed()) {
            let newer = newest_installed
                .get(&found.id)
                .is_none_or(|own| found.version.compare(&own.version).is_gt());
            if newer && (only.is_empty() || only.contains(&found.id)) {
                newest_installed.insert(&found.id, found);
            }
        }
        let mut plan = Plan::default();
        for (id, _) in newest_installed {
            let Some(release) = available.release(id) else {
                continue;
            };
            let Some(change) = derive_item(id, &release.version, installed) else {
                continue;
            };
            plan.releases.insert(id.clone(), release.clone());
            plan.changes.push(change);
        }
        let self_id = self
            .catalog
            .installer()
            .map(|own| ItemId::Product(own.id.clone()));
        plan.changes
            .sort_by_key(|change| Some(change.id()) != self_id.as_ref());
        plan
    }
}

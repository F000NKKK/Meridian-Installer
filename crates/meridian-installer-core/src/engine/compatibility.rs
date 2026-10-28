//! Checking a plan against what its releases need.

use std::collections::BTreeMap;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::listing::Available;
use substrate_installer_app::plans::{Situation, conflicts};
use substrate_installer_app::ports::downloads::{DownloadRequest, TransferControl};
use substrate_installer_domain::compat::{Conflict, Needs};
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::planning::Plan;
use substrate_installer_domain::releases::Asset;

use super::{ManifestError, Session, UnreadManifest};
use crate::formats::parse_needs;

/// What checking a plan found.
#[derive(Debug, Default)]
pub struct Compatibility {
    /// The needs the machine, once the plan is carried out, will not meet.
    pub conflicts: Vec<Conflict>,
    /// The releases whose manifest could not be had.
    pub unread: Vec<UnreadManifest>,
}

impl Session {
    /// Reads the manifest of every release the plan installs and checks the
    /// plan against what they need. A release with no manifest needs
    /// nothing.
    #[must_use]
    pub fn compatibility(
        &self,
        plan: &Plan,
        installed: &[Found],
        available: &Available,
        cancel: &CancelToken,
    ) -> Compatibility {
        let mut needs = BTreeMap::new();
        let mut unread = Vec::new();
        for change in plan.changes.iter().filter(|change| change.installs()) {
            let Some(asset) = self.manifest_of(plan, change.id()) else {
                continue;
            };
            match self.read_needs(asset, cancel) {
                Ok(own) => {
                    needs.insert(change.id().clone(), own);
                }
                Err(error) => unread.push(UnreadManifest {
                    id: change.id().clone(),
                    error,
                }),
            }
        }
        Compatibility {
            conflicts: conflicts(
                plan,
                &Situation {
                    installed,
                    available,
                },
                &needs,
            ),
            unread,
        }
    }

    /// The manifest asset of the release the plan takes for `id`.
    fn manifest_of<'p>(&self, plan: &'p Plan, id: &ItemId) -> Option<&'p Asset> {
        let name = self.catalog.manifest()?;
        plan.releases.get(id)?.asset(name)
    }

    fn read_needs(&self, asset: &Asset, cancel: &CancelToken) -> Result<Needs, ManifestError> {
        let file = self.home().manifest_download();
        let progress = |_| {};
        let control = TransferControl {
            cancel,
            progress: &progress,
        };
        let request = DownloadRequest {
            url: &asset.url,
            destination: &file,
            sha256: asset.sha256.as_ref(),
        };
        self.machine.downloader.download(&request, &control)?;
        let text = std::fs::read_to_string(&file).map_err(ManifestError::Read);
        // The file is scratch: a failure to remove it leaves a file the next
        // download overwrites.
        std::fs::remove_file(&file).ok();
        Ok(parse_needs(&text?)?)
    }
}

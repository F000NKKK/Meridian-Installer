//! A catalog on a host, worked through one machine.

use std::sync::Arc;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::listing::{Available, newest};
use substrate_installer_app::plans::survey;
use substrate_installer_app::ports::records::StoreError;
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::{Found, Origin};
use substrate_installer_domain::platforms::Host;
use substrate_installer_domain::versions::Version;

use crate::catalog::{RefreshError, refresh};
use crate::folder::MeridianHome;
use crate::wiring::Machine;

/// The use cases of the engine over one [`Machine`], one catalog and one
/// host. Cheap to clone: a window clones it out of its state to run an
/// operation on a thread.
#[derive(Debug, Clone)]
pub struct Session {
    pub(super) machine: Arc<Machine>,
    pub(super) catalog: Arc<Catalog>,
    pub(super) host: Host,
    home: MeridianHome,
    running: Option<Version>,
}

impl Session {
    /// A session for `catalog` on `host`, working through `machine` and
    /// keeping its files in `home`.
    #[must_use]
    pub fn new(machine: Arc<Machine>, home: MeridianHome, catalog: Catalog, host: Host) -> Self {
        Self {
            machine,
            catalog: Arc::new(catalog),
            host,
            home,
            running: None,
        }
    }

    /// The same session, knowing the version of the Installer that runs it,
    /// so that the Installer counts as installed and can be updated.
    #[must_use]
    pub fn with_running_version(mut self, version: Version) -> Self {
        self.running = Some(version);
        self
    }

    /// The catalog.
    #[must_use]
    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    /// The host.
    #[must_use]
    pub fn host(&self) -> &Host {
        &self.host
    }

    /// The home folder.
    #[must_use]
    pub fn home(&self) -> &MeridianHome {
        &self.home
    }

    /// The version of the Installer that runs, when it is known.
    #[must_use]
    pub fn running_version(&self) -> Option<&Version> {
        self.running.as_ref()
    }

    /// The same session with `catalog` in place of its own.
    #[must_use]
    pub fn with_catalog(mut self, catalog: Catalog) -> Self {
        self.catalog = Arc::new(catalog);
        self
    }

    /// Asks the Installer's latest release for a newer catalog and takes it.
    /// True when there was one.
    ///
    /// # Errors
    ///
    /// [`RefreshError`] when the release or its catalog cannot be had.
    pub fn refresh_catalog(&mut self, cancel: &CancelToken) -> Result<bool, RefreshError> {
        let fresh = refresh(
            &self.catalog,
            &self.home,
            &*self.machine.releases,
            &*self.machine.downloader,
            cancel,
        )?;
        let found = fresh.is_some();
        if let Some(catalog) = fresh {
            self.catalog = Arc::new(catalog);
        }
        Ok(found)
    }

    /// What is published now: the newest release of everything in the
    /// catalog that the host can install.
    #[must_use]
    pub fn available(&self) -> Available {
        newest(&self.catalog, &*self.machine.releases, &self.host)
    }

    /// Everything installed: what the installer put there, what the system
    /// had before, and the running Installer.
    ///
    /// # Errors
    ///
    /// [`StoreError`] when the records cannot be read.
    pub fn installed(&self) -> Result<Vec<Found>, StoreError> {
        let mut found = survey(&*self.machine.store, &*self.machine.detector, &self.catalog)?;
        if let Some(own) = self.own_record(&found) {
            found.retain(|record| record.id != own.id);
            found.push(own);
        }
        Ok(found)
    }

    /// The running Installer as an installed thing, unless the records
    /// already have this version or a newer one of it.
    fn own_record(&self, found: &[Found]) -> Option<Found> {
        let id = ItemId::Product(self.catalog.installer()?.id.clone());
        let version = self.running.clone()?;
        let recorded = found.iter().find(|record| record.id == id);
        if recorded.is_some_and(|record| record.version.compare(&version).is_ge()) {
            return None;
        }
        Some(Found {
            id,
            version,
            path: std::env::current_exe().ok(),
            origin: Origin::Managed,
        })
    }
}

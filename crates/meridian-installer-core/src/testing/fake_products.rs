//! A product installer that installs nothing and remembers it.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use substrate_installer_app::ports::placement::{
    ProductError, ProductInstall, ProductInstaller, ProductRemoval,
};
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::versions::Version;

/// Remembers the products it was asked to install, as `id version`; clones
/// share the memory.
#[derive(Debug, Default, Clone)]
pub struct FakeProducts {
    installed: Arc<Mutex<Vec<String>>>,
}

impl FakeProducts {
    /// An installer that has installed nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// What was installed, as `id version`.
    #[must_use]
    pub fn installed(&self) -> Vec<String> {
        self.installed
            .lock()
            .map(|own| own.clone())
            .unwrap_or_default()
    }
}

impl ProductInstaller for FakeProducts {
    fn install(
        &self,
        request: &ProductInstall<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        control.cancel.check()?;
        let line = format!("{} {}", request.item.id(), request.version);
        if let Ok(mut own) = self.installed.lock() {
            own.push(line);
        }
        Ok(request.package.to_path_buf())
    }

    fn remove(
        &self,
        request: &ProductRemoval<'_>,
        control: &ToolControl<'_>,
    ) -> Result<(), ProductError> {
        control.cancel.check()?;
        let line = format!("{} {}", request.item.id(), request.version);
        if let Ok(mut own) = self.installed.lock() {
            own.retain(|installed| installed != &line);
        }
        Ok(())
    }

    fn forget_other_versions(&self, _item: Item<'_>, _keep: &Version) -> Result<(), ProductError> {
        Ok(())
    }
}

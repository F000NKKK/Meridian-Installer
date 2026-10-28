//! The ports of the engine, each with its adapter.

use std::fmt;
use std::sync::Arc;

use substrate_installer_app::EventSink;
use substrate_installer_app::installing::Adapters;
use substrate_installer_app::ports::detection::SystemDetector;
use substrate_installer_app::ports::downloads::Downloader;
use substrate_installer_app::ports::placement::{InstallFiles, InstallLayout, ProductInstaller};
use substrate_installer_app::ports::records::InstalledStore;
use substrate_installer_app::ports::sources::ReleaseSource;
use substrate_installer_app::ports::tools::ToolRunner;
use substrate_installer_app::ports::unpacking::Unpacker;

/// One adapter for each port of the engine. The real ones are made by
/// [`real`](Self::real); a test fills the fields with fakes. There is nothing
/// to keep consistent between the fields, so they are public.
pub struct Machine {
    /// Lists releases.
    pub releases: Box<dyn ReleaseSource>,
    /// Downloads.
    pub downloader: Box<dyn Downloader>,
    /// Unpacks archives (shared with the product installer).
    pub unpacker: Arc<dyn Unpacker>,
    /// Where things go.
    pub layout: Box<dyn InstallLayout>,
    /// Puts files in place.
    pub files: Box<dyn InstallFiles>,
    /// Keeps what is installed.
    pub store: Box<dyn InstalledStore>,
    /// Runs rustup and cargo.
    pub tools: Box<dyn ToolRunner>,
    /// Installs the platform's programs.
    pub products: Box<dyn ProductInstaller>,
    /// Finds what the system had before.
    pub detector: Box<dyn SystemDetector>,
}

impl Machine {
    /// The adapters in the form the engine's installer takes, telling `sink`
    /// the progress.
    #[must_use]
    pub fn adapters<'a>(&'a self, sink: &'a dyn EventSink) -> Adapters<'a> {
        Adapters {
            layout: &*self.layout,
            files: &*self.files,
            downloader: &*self.downloader,
            unpacker: &*self.unpacker,
            store: &*self.store,
            tools: &*self.tools,
            products: &*self.products,
            sink,
        }
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine").finish_non_exhaustive()
    }
}

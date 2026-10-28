//! A release source that answers what it was told.

use substrate_installer_app::ports::sources::{ListingError, ReleaseSource};
use substrate_installer_domain::releases::Release;
use substrate_installer_domain::sources::Source;

/// Lists the releases it was given for each source, newest first; a source
/// it was not given has none.
#[derive(Debug, Default, Clone)]
pub struct FakeReleases {
    entries: Vec<(Source, Vec<Release>)>,
}

impl FakeReleases {
    /// A source with no releases at all.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The same, with `releases` (newest first) published at `source`.
    #[must_use]
    pub fn with(mut self, source: Source, releases: Vec<Release>) -> Self {
        self.entries.push((source, releases));
        self
    }
}

impl ReleaseSource for FakeReleases {
    fn releases(&self, source: &Source) -> Result<Vec<Release>, ListingError> {
        Ok(self
            .entries
            .iter()
            .find(|(own, _)| own == source)
            .map(|(_, releases)| releases.clone())
            .unwrap_or_default())
    }
}

//! Refreshing the catalog from the latest Installer release.

use super::RefreshError;
use crate::folder::MeridianHome;
use crate::formats::{CATALOG_FILE_NAME, newer_revision};
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{DownloadRequest, Downloader, TransferControl};
use substrate_installer_app::ports::sources::ReleaseSource;
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::releases::{Asset, AssetName, Release};

/// The asset of an Installer release that carries the catalog.
pub const CATALOG_ASSET: &str = CATALOG_FILE_NAME;

/// The catalog the newest Installer release carries, when its `revision` is
/// above the one of `current`; it is saved in `home` for the next start.
/// None when the catalog has no installer entry, no release carries a
/// catalog, or the one it carries is not newer.
///
/// # Errors
///
/// [`RefreshError`] when the releases cannot be listed, the file cannot be
/// downloaded or saved, or it is not a catalog.
pub fn refresh(
    current: &Catalog,
    home: &MeridianHome,
    releases: &dyn ReleaseSource,
    downloader: &dyn Downloader,
    cancel: &CancelToken,
) -> Result<Option<Catalog>, RefreshError> {
    let Some(installer) = current.installer() else {
        return Ok(None);
    };
    let list = releases.releases(&installer.source)?;
    let Some(asset) = list.iter().find_map(catalog_asset) else {
        return Ok(None);
    };
    let file = home.catalog_download();
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
    downloader.download(&request, &control)?;
    let text = std::fs::read_to_string(&file).map_err(RefreshError::Files)?;
    let newer = newer_revision(&text, current.revision());
    if matches!(newer, Ok(Some(_))) {
        std::fs::rename(&file, home.catalog_file()).map_err(RefreshError::Files)?;
    } else {
        std::fs::remove_file(&file).map_err(RefreshError::Files)?;
    }
    Ok(newer?)
}

/// The catalog asset of `release`, when it has one.
fn catalog_asset(release: &Release) -> Option<&Asset> {
    let name = AssetName::new(CATALOG_ASSET).ok()?;
    release.asset(&name)
}

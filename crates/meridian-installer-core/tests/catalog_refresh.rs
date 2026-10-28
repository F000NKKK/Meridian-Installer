//! Refreshing the catalog from the latest Installer release.

use meridian_installer_core::catalog::{CATALOG_ASSET, built_in, built_in_text, refresh};
use meridian_installer_core::folder::MeridianHome;
use meridian_installer_core::testing::{FakeDownloader, FakeReleases};
use substrate_installer_app::control::CancelToken;
use substrate_installer_domain::releases::{Asset, AssetName, HttpUrl, Release};
use substrate_installer_domain::versions::Version;

const URL: &str = "https://example.test/catalog.sdata";

fn release_with_catalog() -> Release {
    Release::new(Version::new("0.2.0").expect("version")).with_asset(Asset::new(
        AssetName::new(CATALOG_ASSET).expect("name"),
        HttpUrl::new(URL).expect("url"),
    ))
}

fn text_with_revision(revision: u32) -> String {
    let built = built_in().expect("valid").revision();
    built_in_text().replacen(
        &format!("revision: {built}"),
        &format!("revision: {revision}"),
        1,
    )
}

#[test]
fn a_newer_catalog_in_the_latest_release_is_taken_and_saved() {
    let home_dir = tempfile::tempdir().expect("tempdir");
    let home = MeridianHome::new(home_dir.path());
    let current = built_in().expect("valid");
    let source = current.installer().expect("installer").source.clone();
    let releases = FakeReleases::new().with(source, vec![release_with_catalog()]);
    let newer = text_with_revision(current.revision() + 5);
    let downloader = FakeDownloader::new().with(URL, newer.into_bytes());

    let fresh = refresh(&current, &home, &releases, &downloader, &CancelToken::new())
        .expect("refreshes")
        .expect("a newer catalog");
    assert_eq!(fresh.revision(), current.revision() + 5);
    assert!(home.catalog_file().is_file());
    assert!(!home.catalog_download().exists());
}

#[test]
fn an_equal_catalog_is_not_taken() {
    let home_dir = tempfile::tempdir().expect("tempdir");
    let home = MeridianHome::new(home_dir.path());
    let current = built_in().expect("valid");
    let source = current.installer().expect("installer").source.clone();
    let releases = FakeReleases::new().with(source, vec![release_with_catalog()]);
    let downloader = FakeDownloader::new().with(URL, built_in_text().as_bytes().to_vec());

    let fresh =
        refresh(&current, &home, &releases, &downloader, &CancelToken::new()).expect("refreshes");
    assert!(fresh.is_none());
    assert!(!home.catalog_file().exists());
    assert!(!home.catalog_download().exists());
}

#[test]
fn a_release_without_a_catalog_changes_nothing() {
    let home_dir = tempfile::tempdir().expect("tempdir");
    let home = MeridianHome::new(home_dir.path());
    let current = built_in().expect("valid");
    let fresh = refresh(
        &current,
        &home,
        &FakeReleases::new(),
        &FakeDownloader::new(),
        &CancelToken::new(),
    )
    .expect("refreshes");
    assert!(fresh.is_none());
}

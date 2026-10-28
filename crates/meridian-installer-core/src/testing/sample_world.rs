//! A world to install into: the built-in catalog, one release for every
//! item, and a session over a machine of fakes and the real file system.

use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::GzEncoder;
use substrate_installer_archive::ArchiveUnpacker;
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::platforms::{Arch, Host, Os, PlatformTag};
use substrate_installer_domain::releases::{Asset, AssetName, HttpUrl, Release};
use substrate_installer_domain::sources::Source;
use substrate_installer_domain::versions::Version;
use substrate_installer_fs::DiskFiles;

use super::{FakeDownloader, FakeProducts, FakeReleases, FakeTools};
use crate::catalog::built_in;
use crate::engine::Session;
use crate::folder::MeridianHome;
use crate::formats::installed_records;
use crate::wiring::Machine;

/// A session over fakes, with the fakes that remember kept at hand.
#[derive(Debug)]
pub struct SampleWorld {
    /// The session.
    pub session: Session,
    /// The commands run by the tool runner.
    pub tools: FakeTools,
    /// The products installed.
    pub products: FakeProducts,
}

/// A `.tar.gz` of the files `(path, contents)`, all under one top folder.
///
/// # Panics
///
/// Never in practice: it writes to memory.
#[must_use]
pub fn tar_gz(files: &[(&str, &str)]) -> Vec<u8> {
    let mut builder = tar::Builder::new(GzEncoder::new(Vec::new(), Compression::fast()));
    for (path, contents) in files {
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, format!("top/{path}"), contents.as_bytes())
            .expect("writes to memory");
    }
    let encoder = builder.into_inner().expect("writes to memory");
    let mut bytes = encoder.finish().expect("writes to memory");
    bytes.flush().expect("writes to memory");
    bytes
}

impl SampleWorld {
    /// The host the sample world is on: Linux on x86-64, whatever runs the test.
    #[must_use]
    pub fn host() -> Host {
        Host::new(PlatformTag::new(Os::Linux, Arch::X86_64))
    }

    /// A world in `home`: every item of the built-in catalog has one release,
    /// `version`, whose files are small archives.
    ///
    /// # Panics
    ///
    /// When the built-in catalog is invalid, which its own test catches.
    #[must_use]
    pub fn new(home: &Path, version: &str) -> Self {
        let catalog = built_in().expect("the built-in catalog is valid");
        let (releases, downloader) = publish(&catalog, version);
        let tools = FakeTools::new();
        let products = FakeProducts::new();
        let machine = Machine {
            releases: Box::new(releases),
            downloader: Box::new(downloader),
            unpacker: Arc::new(ArchiveUnpacker),
            layout: Box::new(MeridianHome::new(home).layout()),
            files: Box::new(DiskFiles),
            store: Box::new(installed_records(MeridianHome::new(home).installed_file())),
            tools: Box::new(tools.clone()),
            products: Box::new(products.clone()),
            detector: Box::new(NothingFound),
        };
        let session = Session::new(
            Arc::new(machine),
            MeridianHome::new(home),
            catalog,
            Self::host(),
        );
        Self {
            session,
            tools,
            products,
        }
    }
}

/// A detector that finds nothing on the system.
#[derive(Debug)]
struct NothingFound;

impl substrate_installer_app::ports::detection::SystemDetector for NothingFound {
    fn detect(&self, _catalog: &Catalog) -> Vec<substrate_installer_domain::inventory::Found> {
        Vec::new()
    }
}

/// One release `version` of every source of `catalog`, and the downloader
/// that serves its files.
fn publish(catalog: &Catalog, version: &str) -> (FakeReleases, FakeDownloader) {
    let mut releases = FakeReleases::new();
    let mut downloader = FakeDownloader::new();
    let sources = catalog
        .programs()
        .map(|item| (item.id().to_string(), item.source().clone()))
        .chain(
            catalog
                .components()
                .iter()
                .map(|component| (component.id.to_string(), component.source.clone())),
        );
    for (id, source) in sources {
        let release = release_of(&id, &source, version, &mut downloader);
        releases = releases.with(source, vec![release]);
    }
    (releases, downloader)
}

/// The release of `source`: for a GitHub source the asset the host installs,
/// for a tag its source archive, for a tool just the version.
fn release_of(
    id: &str,
    source: &Source,
    version: &str,
    downloader: &mut FakeDownloader,
) -> Release {
    let release = Release::new(Version::new(version).expect("a version"));
    let name = match source {
        Source::GithubRelease { .. } => source
            .asset_suffix(&SampleWorld::host())
            .map(|suffix| format!("{id}{}", suffix.as_str())),
        Source::GitTag { .. } => Some(AssetName::source_archive().to_string()),
        _ => None,
    };
    let Some(name) = name else {
        return release;
    };
    let url = ["https://example.test", id, version, name.as_str()].join("/");
    let bytes = if has_extension(&name, "zip") {
        zip_of(id)
    } else if has_extension(&name, "gz") {
        tar_gz(&[("README", id)])
    } else {
        id.as_bytes().to_vec()
    };
    *downloader = std::mem::take(downloader).with(&url, bytes);
    release.with_asset(Asset::new(
        AssetName::new(name).expect("a name"),
        HttpUrl::new(url).expect("a url"),
    ))
}

/// A zip with one file under one top folder.
fn zip_of(id: &str) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default();
    writer
        .start_file("top/README", options)
        .expect("writes to memory");
    writer.write_all(id.as_bytes()).expect("writes to memory");
    writer.finish().expect("writes to memory").into_inner()
}

/// Whether `name` ends with `.extension`, in any case.
fn has_extension(name: &str, extension: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|own| own.eq_ignore_ascii_case(extension))
}

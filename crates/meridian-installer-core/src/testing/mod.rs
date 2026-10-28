//! Fakes of the ports and a sample machine built from them, so that a
//! session can be exercised with no network, no rustup and no desktop.

mod fake_downloader;
mod fake_products;
mod fake_releases;
mod fake_tools;
mod sample_world;

pub use fake_downloader::FakeDownloader;
pub use fake_products::FakeProducts;
pub use fake_releases::FakeReleases;
pub use fake_tools::FakeTools;
pub use sample_world::{SampleWorld, tar_gz};

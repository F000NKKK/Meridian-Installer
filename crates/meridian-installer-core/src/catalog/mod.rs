//! The catalog: what is built in, what a release brought, and the refresh
//! from the latest Installer release.

mod built_in;
mod load_error;
mod loaded;
mod notice;
mod refresh;
mod refresh_error;

pub use built_in::{built_in, built_in_text};
pub use load_error::LoadError;
pub use loaded::Loaded;
pub use notice::Notice;
pub use refresh::{CATALOG_ASSET, refresh};
pub use refresh_error::RefreshError;

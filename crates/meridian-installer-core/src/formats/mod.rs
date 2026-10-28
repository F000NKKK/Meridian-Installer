//! The file formats of the installer's own files, in one place: the catalog
//! and a release's compatibility manifest are Substrate Data text, the
//! installed records are Substrate Data binary, all through
//! `substrate-installer-data`. A format that changes changes only this module.

mod catalog_format;
mod installed_store;
mod names;

pub use catalog_format::{CatalogFormatError, NeedsFormatError, newer_revision, parse_needs};
pub use installed_store::{InstalledRecords, installed_records};
pub use names::{CATALOG_FILE_NAME, INSTALLED_FILE_NAME, parse_catalog_text};

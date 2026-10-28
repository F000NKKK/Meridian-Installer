//! The catalog the Installer starts with.

use super::{LoadError, Notice, built_in};
use crate::folder::MeridianHome;
use crate::formats::newer_revision;
use substrate_installer_domain::channel::Catalog;

/// The catalog to start with, and why a saved one was passed over.
#[derive(Debug)]
pub struct Loaded {
    /// The built-in catalog, or the saved one when it is newer.
    pub catalog: Catalog,
    /// Why the saved catalog is not used; none when it is, or there is none.
    pub notice: Option<Notice>,
}

impl Loaded {
    /// The newest of the built-in catalog and the one saved in `home` by an
    /// earlier refresh (an Installer takes the catalog of the higher
    /// `revision`).
    ///
    /// # Errors
    ///
    /// [`LoadError`] when the built-in catalog is invalid; a saved one
    /// that cannot be used is a [`notice`](Self::notice), not an error.
    pub fn from_home(home: &MeridianHome) -> Result<Self, LoadError> {
        let built_in = built_in()?;
        let path = home.catalog_file();
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::plain(built_in));
            }
            Err(source) => {
                return Ok(Self::noticed(built_in, Notice::Unreadable { path, source }));
            }
        };
        Ok(match newer_revision(&text, built_in.revision()) {
            Ok(Some(saved)) => Self::plain(saved),
            Ok(None) => Self::plain(built_in),
            Err(source) => Self::noticed(built_in, Notice::Invalid { path, source }),
        })
    }

    fn plain(catalog: Catalog) -> Self {
        Self {
            catalog,
            notice: None,
        }
    }

    fn noticed(catalog: Catalog, notice: Notice) -> Self {
        Self {
            catalog,
            notice: Some(notice),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::built_in_text;

    fn with_revision(revision: u32) -> String {
        let built = built_in().expect("valid").revision();
        built_in_text().replacen(
            &format!("revision: {built}"),
            &format!("revision: {revision}"),
            1,
        )
    }

    #[test]
    fn without_a_saved_catalog_the_built_in_one_is_used() {
        let dir = tempfile::tempdir().expect("tempdir");
        let loaded = Loaded::from_home(&MeridianHome::new(dir.path())).expect("loads");
        assert_eq!(loaded.catalog, built_in().expect("valid"));
        assert!(loaded.notice.is_none());
    }

    #[test]
    fn a_newer_saved_catalog_wins_and_an_older_one_does_not() {
        let dir = tempfile::tempdir().expect("tempdir");
        let home = MeridianHome::new(dir.path());
        let built = built_in().expect("valid").revision();
        std::fs::write(home.catalog_file(), with_revision(built + 1)).expect("write");
        let loaded = Loaded::from_home(&home).expect("loads");
        assert_eq!(loaded.catalog.revision(), built + 1);
        std::fs::write(home.catalog_file(), with_revision(built.saturating_sub(1))).expect("write");
        let older = Loaded::from_home(&home).expect("loads");
        assert_eq!(older.catalog.revision(), built);
    }

    #[test]
    fn a_broken_saved_catalog_is_a_notice_and_the_built_in_one_is_used() {
        let dir = tempfile::tempdir().expect("tempdir");
        let home = MeridianHome::new(dir.path());
        std::fs::write(home.catalog_file(), "revision = [").expect("write");
        let loaded = Loaded::from_home(&home).expect("loads");
        assert!(matches!(loaded.notice, Some(Notice::Invalid { .. })));
        assert_eq!(loaded.catalog, built_in().expect("valid"));
    }
}

//! Where Meridian Installer keeps everything.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use substrate_installer_fs::HomeLayout;

use super::LocateError;
use crate::formats::{CATALOG_FILE_NAME, INSTALLED_FILE_NAME};

/// The environment variable that moves the home folder.
pub const HOME_VARIABLE: &str = "MERIDIAN_HOME";

/// The folder under the user's home that holds everything by default.
const DEFAULT_FOLDER: &str = ".meridian";

/// Where a catalog is downloaded to before it is checked.
const CATALOG_DOWNLOAD: &str = "catalog.sdata.download";
/// Where a release's compatibility manifest is downloaded to before it is read.
const MANIFEST_DOWNLOAD: &str = "manifest.sdata.download";
/// The answers of GitHub and crates.io, with their `ETag`s.
const HTTP_CACHE: &str = "http-cache";

/// The installer's home folder, `~/.meridian`:
///
/// ```text
/// <home>/components/<id>/<version>   <home>/products/<id>/<version>
/// <home>/cache/   <home>/staging/    <home>/installed.sdata
/// ```
///
/// The first four are the engine's own layout ([`layout`](Self::layout)); the
/// rest are this installer's files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeridianHome {
    root: PathBuf,
}

impl MeridianHome {
    /// The home folder `root`.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The home folder for what the caller named, what `MERIDIAN_HOME` says
    /// and the user's home folder, in that order of precedence; an empty
    /// `MERIDIAN_HOME` counts as unset.
    ///
    /// # Errors
    ///
    /// [`LocateError::NotFound`] when none of the three says.
    pub fn resolve(
        explicit: Option<PathBuf>,
        variable: Option<OsString>,
        user_home: Option<PathBuf>,
    ) -> Result<Self, LocateError> {
        let root = explicit
            .or_else(|| {
                variable
                    .filter(|value| !value.is_empty())
                    .map(PathBuf::from)
            })
            .or_else(|| user_home.map(|home| home.join(DEFAULT_FOLDER)))
            .ok_or(LocateError::NotFound)?;
        Ok(Self::new(root))
    }

    /// The home folder of this process: `explicit`, else `MERIDIAN_HOME`,
    /// else `~/.meridian`.
    ///
    /// # Errors
    ///
    /// [`LocateError::NotFound`] when none of them is known.
    pub fn locate(explicit: Option<PathBuf>) -> Result<Self, LocateError> {
        Self::resolve(
            explicit,
            std::env::var_os(HOME_VARIABLE),
            std::env::home_dir(),
        )
    }

    /// The folder.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Where the engine puts products, components, downloads and staging.
    #[must_use]
    pub fn layout(&self) -> HomeLayout {
        HomeLayout::new(&self.root)
    }

    /// The file that records what is installed.
    #[must_use]
    pub fn installed_file(&self) -> PathBuf {
        self.root.join(INSTALLED_FILE_NAME)
    }

    /// The catalog a release brought.
    #[must_use]
    pub fn catalog_file(&self) -> PathBuf {
        self.root.join(CATALOG_FILE_NAME)
    }

    /// Where a catalog is downloaded to before it is checked.
    #[must_use]
    pub fn catalog_download(&self) -> PathBuf {
        self.root.join(CATALOG_DOWNLOAD)
    }

    /// The cache of the HTTP answers.
    #[must_use]
    pub fn http_cache(&self) -> PathBuf {
        self.root.join(HTTP_CACHE)
    }

    /// Where the compatibility manifests of releases are downloaded to.
    #[must_use]
    pub fn manifest_download(&self) -> PathBuf {
        self.root.join(MANIFEST_DOWNLOAD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_caller_beats_the_variable_beats_the_user_home() {
        let user = Some(PathBuf::from("/home/u"));
        let variable = Some(OsString::from("/opt/m"));
        let explicit = Some(PathBuf::from("/x"));
        let home = |e, v, u| MeridianHome::resolve(e, v, u).map(|h| h.root().to_path_buf());
        assert_eq!(
            home(explicit.clone(), variable.clone(), user.clone()),
            Ok("/x".into())
        );
        assert_eq!(home(None, variable, user.clone()), Ok("/opt/m".into()));
        assert_eq!(
            home(None, Some(OsString::new()), user),
            Ok("/home/u/.meridian".into())
        );
        assert_eq!(home(None, None, None), Err(LocateError::NotFound));
    }

    #[test]
    fn its_files_are_under_the_root() {
        let home = MeridianHome::new("/m");
        assert_eq!(
            home.installed_file(),
            Path::new("/m").join(INSTALLED_FILE_NAME)
        );
        assert_eq!(home.catalog_file(), Path::new("/m").join(CATALOG_FILE_NAME));
        assert_eq!(home.http_cache(), Path::new("/m/http-cache"));
        assert_eq!(home.layout().home(), Path::new("/m"));
        assert!(home.catalog_download().starts_with("/m"));
        assert!(home.manifest_download().starts_with("/m"));
    }
}

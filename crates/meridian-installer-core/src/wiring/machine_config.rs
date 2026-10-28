//! What the real machine is made from.

use std::path::PathBuf;

use substrate_installer_http::{ClientConfig, GithubToken};

use super::MachineError;
use crate::credentials::find_github_token;
use crate::folder::MeridianHome;

/// How the Installer introduces itself to GitHub.
const USER_AGENT: &str = concat!("Meridian-Installer/", env!("CARGO_PKG_VERSION"));

/// What the real machine is made from.
#[derive(Debug, Clone)]
pub struct MachineConfig {
    /// The installer's home folder.
    pub home: MeridianHome,
    /// The user's home folder: launchers are listed under it.
    pub user_home: PathBuf,
    /// The GitHub token, where there is one.
    pub token: Option<GithubToken>,
}

impl MachineConfig {
    /// The config of this process: the home folder `explicit` names, else
    /// the environment's; the token `GITHUB_TOKEN` or `gh` holds.
    ///
    /// # Errors
    ///
    /// [`MachineError`] when the home folders are not known.
    pub fn from_environment(explicit_home: Option<PathBuf>) -> Result<Self, MachineError> {
        Ok(Self {
            home: MeridianHome::locate(explicit_home)?,
            user_home: std::env::home_dir().ok_or(MachineError::NoUserHome)?,
            token: find_github_token(),
        })
    }

    /// The HTTP client's config: the Installer's name, the cache in the
    /// home folder, the token.
    #[must_use]
    pub fn client_config(&self) -> ClientConfig {
        let config = ClientConfig::new(USER_AGENT).with_cache_dir(self.home.http_cache());
        match &self.token {
            Some(token) => config.with_github_token(token.clone()),
            None => config,
        }
    }
}

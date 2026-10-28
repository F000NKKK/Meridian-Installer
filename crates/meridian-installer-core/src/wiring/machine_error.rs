//! Why the machine cannot be made.

use crate::folder::LocateError;

/// Why the machine cannot be made.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MachineError {
    /// The home folder is not known.
    #[error(transparent)]
    Home(#[from] LocateError),
    /// The user's home folder is not known, so cargo's and the launchers'
    /// places are not either.
    #[error("the user's home folder is not known")]
    NoUserHome,
    /// The operating system has no installer for its programs.
    #[error("this operating system is not supported")]
    UnsupportedSystem,
}

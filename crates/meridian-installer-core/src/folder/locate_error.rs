//! Why the home folder cannot be found.

/// Why the home folder cannot be found.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum LocateError {
    /// No folder was named, `MERIDIAN_HOME` is not set and the system has no
    /// home folder for the user.
    #[error("there is no home folder: name one with --home or set MERIDIAN_HOME")]
    NotFound,
}

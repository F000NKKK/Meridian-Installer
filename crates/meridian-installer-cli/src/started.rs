//! The session of this process, and what to tell.

use meridian_installer_core::engine::Session;

/// The session, and what to tell before using it.
#[derive(Debug)]
pub struct Started {
    /// The session.
    pub session: Session,
    /// Things that went wrong without stopping the start.
    pub notices: Vec<String>,
}

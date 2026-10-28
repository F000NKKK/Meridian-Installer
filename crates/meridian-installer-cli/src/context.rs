//! What every command works with.

use std::io::Write;

use meridian_installer_core::engine::Session;
use substrate_installer_app::EventSink;
use substrate_installer_app::control::CancelToken;

/// What every command works with: the session, where the answer goes, where
/// the progress goes and the token that cancels.
pub struct Context<'a> {
    /// The session.
    pub session: &'a Session,
    /// Things to tell before anything else (a catalog that could not be refreshed).
    pub notices: &'a [String],
    /// Whether the answer is JSON.
    pub json: bool,
    /// Where the answer goes.
    pub out: &'a mut dyn Write,
    /// Who hears the progress.
    pub events: &'a dyn EventSink,
    /// Cancels the operation.
    pub cancel: &'a CancelToken,
}

impl std::fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("json", &self.json)
            .finish_non_exhaustive()
    }
}

//! What the app holds between commands.

use std::sync::Arc;

use meridian_installer_core::activity::Operations;

/// What the app holds between commands: the operations, or why there are none
/// (no home folder, an unsupported system) — the window shows that.
#[derive(Debug)]
pub(crate) struct AppState {
    operations: Result<Arc<Operations>, String>,
}

impl AppState {
    pub(crate) fn new(operations: Result<Operations, String>) -> Self {
        Self {
            operations: operations.map(Arc::new),
        }
    }

    /// The operations; why there are none otherwise.
    pub(crate) fn operations(&self) -> Result<&Arc<Operations>, String> {
        self.operations.as_ref().map_err(Clone::clone)
    }
}

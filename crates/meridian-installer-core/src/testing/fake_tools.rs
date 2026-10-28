//! A tool runner that runs nothing and remembers the commands.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use substrate_installer_app::ports::tools::{ToolCommand, ToolControl, ToolError, ToolRunner};

/// Remembers the commands it was asked to run; clones share the memory.
#[derive(Debug, Default, Clone)]
pub struct FakeTools {
    commands: Arc<Mutex<Vec<ToolCommand>>>,
}

impl FakeTools {
    /// A runner that has run nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The commands run so far.
    #[must_use]
    pub fn commands(&self) -> Vec<ToolCommand> {
        self.commands
            .lock()
            .map(|own| own.clone())
            .unwrap_or_default()
    }
}

impl ToolRunner for FakeTools {
    fn run(&self, command: &ToolCommand, control: &ToolControl<'_>) -> Result<PathBuf, ToolError> {
        control.cancel.check()?;
        (control.output)("fake tool ran");
        if let Ok(mut own) = self.commands.lock() {
            own.push(command.clone());
        }
        Ok(PathBuf::from("fake-bin"))
    }
}

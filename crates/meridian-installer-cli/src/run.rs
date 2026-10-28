//! Running a command.

use crate::command_line::Command;
use crate::commands::{install, list, plan, remove, status, update};
use crate::{CliError, Context};

/// Runs `command`, writing its answer to `context.out`.
///
/// # Errors
///
/// [`CliError`] when the command fails.
pub fn run(command: &Command, context: &mut Context<'_>) -> Result<(), CliError> {
    match command {
        Command::List => list(context),
        Command::Plan(args) => plan(context, args),
        Command::Install(args) => install(context, args),
        Command::Update(args) => update(context, args),
        Command::Remove(args) => remove(context, args),
        Command::Status => status(context),
    }
}

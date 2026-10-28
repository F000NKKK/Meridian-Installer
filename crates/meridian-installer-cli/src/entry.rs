//! The process: parse, run, report.

use std::io::Write;
use std::process::ExitCode;

use clap::Parser;
use substrate_installer_app::control::CancelToken;

use crate::command_line::Cli;
use crate::{CliError, Context, ProgressSink, run, start};

/// Runs the command line of this process and tells how it ended.
#[must_use]
pub fn main_exit() -> ExitCode {
    let cli = Cli::parse();
    match execute(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            report(&error);
            ExitCode::FAILURE
        }
    }
}

fn execute(cli: &Cli) -> Result<(), CliError> {
    let started = start(cli)?;
    let cancel = CancelToken::new();
    let on_interrupt = cancel.clone();
    // Without a handler the default of the system applies: the process dies.
    ctrlc::set_handler(move || on_interrupt.cancel()).ok();
    let progress = ProgressSink::new(cli.json);
    let mut stdout = std::io::stdout().lock();
    let mut context = Context {
        session: &started.session,
        notices: &started.notices,
        json: cli.json,
        out: &mut stdout,
        events: &progress,
        cancel: &cancel,
    };
    run(&cli.command, &mut context)
}

/// Writes the error and its causes to standard error.
fn report(error: &CliError) {
    let mut stderr = std::io::stderr().lock();
    writeln!(stderr, "error: {error}").ok();
    let mut cause = std::error::Error::source(error);
    while let Some(own) = cause {
        writeln!(stderr, "  caused by: {own}").ok();
        cause = own.source();
    }
}

//! The commands, over a session of fakes.

use clap::Parser;
use meridian_installer_cli::command_line::Cli;
use meridian_installer_cli::{CliError, Context, run};
use meridian_installer_core::engine::Session;
use meridian_installer_core::testing::SampleWorld;
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::tools::ToolCommand;

/// Runs the command line `words` and returns what it wrote.
fn execute(session: &Session, words: &[&str]) -> Result<String, CliError> {
    let cli =
        Cli::try_parse_from(std::iter::once("meridian-installer").chain(words.iter().copied()))
            .expect("a command line");
    let mut out = Vec::new();
    let events = |_| {};
    let cancel = CancelToken::new();
    let mut context = Context {
        session,
        notices: &[],
        json: cli.json,
        out: &mut out,
        events: &events,
        cancel: &cancel,
    };
    run(&cli.command, &mut context)?;
    Ok(String::from_utf8(out).expect("text"))
}

#[test]
fn plan_shows_the_changes_and_installs_nothing() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    let text = execute(&world.session, &["plan", "meridian.workload.game"]).expect("plans");
    assert!(text.contains("install meridian.cli 0.1.0"), "{text}");
    assert!(text.contains("install rust.toolchain 0.1.0"), "{text}");
    assert!(!home.path().join("components").exists());
}

#[test]
fn install_status_update_and_remove_work_end_to_end() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    execute(&world.session, &["install", "meridian.workload.game"]).expect("installs");
    assert!(world.tools.commands().iter().any(|command| matches!(command,
        ToolCommand::InstallCrate { name, version } if name.as_str() == "meridian-cli" && version.to_string() == "0.1.0"
    )));

    let status = execute(&world.session, &["status"]).expect("status");
    assert!(
        status.contains("meridian.cli 0.1.0: up to date"),
        "{status}"
    );

    let newer = SampleWorld::new(home.path(), "0.2.0");
    let later = execute(&newer.session, &["status"]).expect("status");
    assert!(later.contains("update available (0.2.0)"), "{status}");
    execute(&newer.session, &["update", "meridian.cli"]).expect("updates");
    assert!(newer.tools.commands().iter().any(|command| matches!(command,
        ToolCommand::InstallCrate { name, version } if name.as_str() == "meridian-cli" && version.to_string() == "0.2.0"
    )));

    let refused = execute(&newer.session, &["remove", "rust.toolchain"]);
    assert!(
        matches!(refused, Err(CliError::StillRequired { .. })),
        "{refused:?}"
    );
    execute(&newer.session, &["remove", "meridian.cli"]).expect("removes");
    assert!(
        newer
            .tools
            .commands()
            .iter()
            .any(|command| matches!(command,
                ToolCommand::UninstallCrate { name } if name.as_str() == "meridian-cli"
            ))
    );
}

#[test]
fn list_and_status_speak_json() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    let text = execute(&world.session, &["--json", "list"]).expect("lists");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json");
    assert_eq!(json["products"][0]["id"], "meridian.idle");
    assert_eq!(json["workloads"].as_array().map(Vec::len), Some(2));
    let listing = execute(&world.session, &["list"]).expect("lists");
    assert!(listing.contains("Game development"), "{listing}");
    let status = execute(&world.session, &["status"]).expect("status");
    assert!(status.contains("Nothing is installed."), "{status}");
}

#[test]
fn an_unknown_target_is_an_error() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    let error = execute(&world.session, &["plan", "nothing.like.it"]).expect_err("unknown");
    assert!(matches!(error, CliError::NotInCatalog(_)));
}

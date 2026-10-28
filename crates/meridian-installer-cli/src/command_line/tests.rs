//! The command line parses.

use clap::Parser;

use super::*;

#[test]
fn install_takes_targets_and_options() {
    let cli = Cli::try_parse_from([
        "meridian-installer",
        "--json",
        "install",
        "meridian.workload.game",
        "--without",
        "meridian.examples",
        "--mode",
        "download-first",
    ])
    .expect("parses");
    assert!(cli.json);
    let Command::Install(install) = cli.command else {
        panic!("an install");
    };
    assert_eq!(
        install.targets.targets[0].as_str(),
        "meridian.workload.game"
    );
    assert_eq!(install.targets.without.len(), 1);
    assert!(matches!(install.apply.mode, ModeArg::DownloadFirst));
}

#[test]
fn a_bad_id_and_a_missing_target_are_refused() {
    assert!(Cli::try_parse_from(["meridian-installer", "install", "Not An Id"]).is_err());
    assert!(Cli::try_parse_from(["meridian-installer", "remove"]).is_err());
    assert!(Cli::try_parse_from(["meridian-installer", "update"]).is_ok());
}

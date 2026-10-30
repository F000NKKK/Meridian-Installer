//! Installing, updating and removing through a session of fakes.

use meridian_installer_core::display::Snapshot;
use meridian_installer_core::testing::SampleWorld;
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::installing::ApplyOptions;
use substrate_installer_app::plans::for_removal;
use substrate_installer_app::ports::tools::ToolCommand;
use substrate_installer_domain::changes::Change;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::ids::{ComponentId, ItemId, WorkloadId};
use substrate_installer_domain::versions::Version;

fn game_selection() -> Selection {
    Selection::default().with_workload(WorkloadId::new("meridian.workload.game").expect("id"))
}

fn component(id: &str) -> ItemId {
    ItemId::Component(ComponentId::new(id).expect("id"))
}

fn install_game(world: &SampleWorld) {
    let session = &world.session;
    let available = session.available();
    assert!(
        available.problems().is_empty(),
        "{:?}",
        available.problems()
    );
    let installed = session.installed().expect("records");
    let plan = session.plan_selection(&game_selection(), &installed, &available);
    assert!(!plan.is_empty());
    let sink = |_| {};
    session
        .apply(&plan, ApplyOptions::default(), &CancelToken::new(), &sink)
        .expect("the plan is carried out");
}

#[test]
fn the_game_workload_installs_its_components_in_order() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    install_game(&world);

    let installed = world.session.installed().expect("records");
    for id in [
        "rust.toolchain",
        "meridian.cli",
        "meridian.sdk",
        "meridian.examples",
    ] {
        assert!(
            installed.iter().any(|found| found.id == component(id)),
            "{id}"
        );
    }
    assert!(
        !installed
            .iter()
            .any(|found| found.id == component("meridian.assets"))
    );
    assert!(world.tools.commands().iter().any(|command| matches!(command,
        ToolCommand::InstallCrate { name, version } if name.as_str() == "meridian-cli" && version.to_string() == "0.1.0"
    )));
    assert!(
        home.path()
            .join("components/meridian.sdk/0.1.0/README")
            .is_file()
    );
    assert!(
        !world.tools.commands().is_empty(),
        "rustup was asked for the toolchain"
    );
}

#[test]
fn a_newer_release_updates_and_a_removal_removes() {
    let home = tempfile::tempdir().expect("tempdir");
    install_game(&SampleWorld::new(home.path(), "0.1.0"));

    let newer = SampleWorld::new(home.path(), "0.2.0");
    let session = &newer.session;
    let available = session.available();
    let installed = session.installed().expect("records");
    let plan = session.plan_update(&[], &installed, &available);
    assert!(plan.changes.iter().any(
        |change| matches!(change, Change::Update { id, .. } if *id == component("meridian.cli"))
    ));
    let sink = |_| {};
    session
        .apply(&plan, ApplyOptions::default(), &CancelToken::new(), &sink)
        .expect("updates");
    assert!(newer.tools.commands().iter().any(|command| matches!(command,
        ToolCommand::InstallCrate { name, version } if name.as_str() == "meridian-cli" && version.to_string() == "0.2.0"
    )));
    assert!(!home.path().join("components/meridian.cli/0.1.0").exists());

    let after_update = session.installed().expect("records");
    let removal = for_removal(&component("meridian.cli"), &after_update);
    session
        .apply(
            &removal,
            ApplyOptions::default(),
            &CancelToken::new(),
            &sink,
        )
        .expect("removes");
    assert!(
        newer
            .tools
            .commands()
            .iter()
            .any(|command| matches!(command,
                ToolCommand::UninstallCrate { name } if name.as_str() == "meridian-cli"
            ))
    );
    let after_removal = session.installed().expect("records");
    assert!(
        !after_removal
            .iter()
            .any(|found| found.id == component("meridian.cli"))
    );
}

#[test]
fn the_installer_updates_itself_first() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.2.0");
    let session = world
        .session
        .clone()
        .with_running_version(Version::new("0.1.0").expect("version"));
    let available = session.available();
    let installed = session.installed().expect("records");
    let snapshot = Snapshot::of(&session, &installed, &available, Vec::new());
    assert_eq!(snapshot.installer_update.as_deref(), Some("0.2.0"));

    let plan = session.plan_update(&[], &installed, &available);
    let first = plan.changes.first().expect("a change");
    assert!(matches!(
        first,
        Change::Update {
            id: ItemId::Product(_),
            ..
        }
    ));
    let sink = |_| {};
    session
        .apply(&plan, ApplyOptions::default(), &CancelToken::new(), &sink)
        .expect("updates");
    assert_eq!(world.products.installed(), ["installer 0.2.0"]);
}

#[test]
fn a_snapshot_shows_the_catalog_with_versions() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    install_game(&world);
    let session = &world.session;
    let available = session.available();
    let installed = session.installed().expect("records");
    let snapshot = Snapshot::of(session, &installed, &available, vec!["note".into()]);
    let cli = snapshot
        .components
        .iter()
        .find(|c| c.id == "meridian.cli")
        .expect("cli");
    assert_eq!(cli.installed.as_deref(), Some("0.1.0"));
    assert!(cli.managed);
    let idle = &snapshot.products[0];
    assert_eq!(idle.installed, None);
    assert_eq!(idle.latest.as_deref(), Some("0.1.0"));
    assert_eq!(snapshot.problems, ["note"]);
    let json = serde_json::to_value(&snapshot).expect("json");
    assert!(json["products"][0]["updateAvailable"].is_boolean());
}

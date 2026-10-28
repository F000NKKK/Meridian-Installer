//! What a window does, over a session of fakes.

use std::sync::{Arc, Mutex};

use meridian_installer_core::activity::{ApplyRequest, Emit, ModeView, OperationError, Operations};
use meridian_installer_core::display::{EventView, SelectionRequest};
use meridian_installer_core::testing::SampleWorld;

fn request() -> ApplyRequest {
    ApplyRequest {
        product: "meridian.idle".into(),
        selection: SelectionRequest {
            workloads: vec!["meridian.workload.game".into()],
            unchecked: vec![],
            checked: vec![],
        },
        mode: ModeView::InstallWhileDownloading,
        keep_downloads: false,
    }
}

type Heard = Arc<Mutex<Vec<(String, EventView)>>>;

fn collector() -> (Emit, Heard) {
    let heard = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&heard);
    let emit: Emit = Arc::new(move |label, event| {
        sink.lock().expect("lock").push((label.to_owned(), event));
    });
    (emit, heard)
}

#[test]
fn a_product_is_installed_on_a_thread_and_the_events_end_with_finished() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    let operations = Arc::new(Operations::new(world.session.clone(), Vec::new()));
    let first = operations.snapshot(false).expect("snapshot");
    assert!(first.products[0].installed.is_none());

    let plan = operations.plan(&request()).expect("plan");
    assert!(plan.changes.len() >= 5, "{:?}", plan.changes);

    let (emit, heard) = collector();
    operations
        .start_apply(&request(), emit)
        .expect("starts")
        .join()
        .expect("finishes");
    let events = heard.lock().expect("lock");
    assert!(events.iter().all(|(label, _)| label == "meridian.idle"));
    assert!(matches!(
        events.last(),
        Some((_, EventView::Finished { ok: true, .. }))
    ));
    drop(events);

    let after = operations.snapshot(false).expect("snapshot");
    assert_eq!(after.products[0].installed.as_deref(), Some("0.1.0"));
    assert!(
        operations
            .program_of("meridian.idle")
            .expect("program")
            .is_some()
    );
    let selection = operations.selection_of("meridian.idle").expect("selection");
    assert!(
        selection
            .workloads
            .contains(&"meridian.workload.game".to_owned())
    );
}

#[test]
fn an_uninstall_removes_the_product_and_leaves_its_components() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    let operations = Arc::new(Operations::new(world.session.clone(), Vec::new()));
    operations.snapshot(false).expect("snapshot");
    let (emit, _) = collector();
    operations
        .start_apply(&request(), emit)
        .expect("starts")
        .join()
        .expect("done");

    let (leaving, _) = collector();
    operations
        .start_uninstall("meridian.idle", leaving)
        .expect("starts")
        .join()
        .expect("done");
    let after = operations.snapshot(false).expect("snapshot");
    assert!(after.products[0].installed.is_none());
    assert!(
        after
            .components
            .iter()
            .any(|c| c.id == "meridian.cli" && c.installed.is_some())
    );
}

#[test]
fn a_name_that_is_not_a_product_is_refused() {
    let home = tempfile::tempdir().expect("tempdir");
    let world = SampleWorld::new(home.path(), "0.1.0");
    let operations = Operations::new(world.session.clone(), Vec::new());
    assert!(matches!(
        operations.selection_of("meridian.cli"),
        Err(OperationError::NotAProduct(_))
    ));
    assert!(matches!(
        operations.selection_of("Bad Id"),
        Err(OperationError::Id(_))
    ));
}

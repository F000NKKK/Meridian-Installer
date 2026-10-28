//! Telling the progress of an operation.

use std::io::Write;

use meridian_installer_core::display::EventView;
use substrate_installer_app::EventSink;
use substrate_installer_domain::events::Event;

/// Tells the progress of an operation on standard error: one line per step
/// as text, or one JSON object per event.
#[derive(Debug, Clone, Copy)]
pub struct ProgressSink {
    json: bool,
}

impl ProgressSink {
    /// A sink that writes JSON lines when `json`, else text.
    #[must_use]
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

impl EventSink for ProgressSink {
    fn emit(&self, event: Event) {
        let line = if self.json {
            EventView::of(&event).and_then(|view| serde_json::to_string(&view).ok())
        } else {
            text_of(&event)
        };
        if let Some(line) = line {
            // A closed standard error is nobody listening.
            writeln!(std::io::stderr().lock(), "{line}").ok();
        }
    }
}

/// The line that tells `event`; none for a chunk of a download.
fn text_of(event: &Event) -> Option<String> {
    Some(match event {
        Event::Started { packages, .. } => format!("Starting: {packages} package(s)"),
        Event::Downloading { item, done, total } if Some(*done) == *total => {
            format!("Downloaded {item}")
        }
        Event::Installing { item, index, of } => format!("Installing {item} ({index} of {of})"),
        Event::Output { line, .. } => format!("  | {line}"),
        Event::Installed { item, version } => format!("Installed {item} {version}"),
        Event::Removed { item } => format!("Removed {item}"),
        Event::Finished { outcome } => format!("Finished: {outcome:?}"),
        _ => return None,
    })
}

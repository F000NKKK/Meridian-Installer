//! A progress event, as the window reads it.

use serde::Serialize;
use substrate_installer_domain::events::{Event, Outcome};

/// One thing that happened while a plan was carried out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum EventView {
    /// The plan starts.
    Started {
        /// Packages it changes.
        packages: usize,
        /// Bytes to fetch.
        bytes: u64,
    },
    /// Bytes of a download arrived.
    Downloading {
        /// The package.
        component: String,
        /// Bytes so far.
        done: u64,
        /// Bytes in all.
        total: Option<u64>,
    },
    /// A package is being installed.
    Installing {
        /// The package.
        component: String,
        /// Its number, from 1.
        index: usize,
        /// How many there are.
        of: usize,
    },
    /// A line a tool printed.
    Output {
        /// The package.
        component: String,
        /// The line.
        line: String,
    },
    /// A package is installed.
    Installed {
        /// The package.
        component: String,
        /// Its version.
        version: String,
    },
    /// A package is removed.
    Removed {
        /// The package.
        component: String,
    },
    /// The plan ended.
    Finished {
        /// Whether every change was made.
        ok: bool,
        /// Whether the user cancelled it.
        cancelled: bool,
        /// Why it failed.
        error: Option<String>,
    },
}

impl EventView {
    /// The view of `event`; none for an event this version does not know.
    #[must_use]
    pub fn of(event: &Event) -> Option<Self> {
        Some(match event {
            Event::Started { packages, bytes } => Self::Started {
                packages: *packages,
                bytes: *bytes,
            },
            Event::Downloading { item, done, total } => Self::Downloading {
                component: item.to_string(),
                done: *done,
                total: *total,
            },
            Event::Installing { item, index, of } => Self::Installing {
                component: item.to_string(),
                index: *index,
                of: *of,
            },
            Event::Output { item, line } => Self::Output {
                component: item.to_string(),
                line: line.clone(),
            },
            Event::Installed { item, version } => Self::Installed {
                component: item.to_string(),
                version: version.to_string(),
            },
            Event::Removed { item } => Self::Removed {
                component: item.to_string(),
            },
            Event::Finished { outcome } => Self::finished(outcome),
            _ => return None,
        })
    }

    fn finished(outcome: &Outcome) -> Self {
        Self::Finished {
            ok: outcome.is_completed(),
            cancelled: matches!(outcome, Outcome::Cancelled),
            error: match outcome {
                Outcome::Failed { reason } => Some(reason.clone()),
                _ => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use substrate_installer_domain::ids::{ComponentId, ItemId};

    use super::*;

    #[test]
    fn events_serialize_with_a_kind() {
        let item = ItemId::Component(ComponentId::new("meridian.cli").expect("id"));
        let event = Event::Downloading {
            item,
            done: 5,
            total: None,
        };
        let json = serde_json::to_value(EventView::of(&event)).expect("json");
        assert_eq!(json["kind"], "downloading");
        assert_eq!(json["component"], "meridian.cli");
        let failed = Event::Finished {
            outcome: Outcome::Failed { reason: "x".into() },
        };
        let ended = serde_json::to_value(EventView::of(&failed)).expect("json");
        assert_eq!(ended["ok"], false);
        assert_eq!(ended["error"], "x");
    }
}

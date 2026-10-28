//! The events the Installer's backend sends its window.

/// An operation's progress: `{ product, event }`.
pub(crate) const PROGRESS: &str = "installer-progress";

/// Every event, for the window's copy: (name in code, event).
pub(crate) const ALL: &[(&str, &str)] = &[("progress", PROGRESS)];

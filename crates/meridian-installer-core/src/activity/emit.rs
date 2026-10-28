//! Who hears an operation's progress.

use std::sync::Arc;

use crate::display::EventView;

/// Who hears the progress of an operation, called with the label of the
/// operation (a product's id) and the event, from the thread doing the work.
pub type Emit = Arc<dyn Fn(&str, EventView) + Send + Sync>;

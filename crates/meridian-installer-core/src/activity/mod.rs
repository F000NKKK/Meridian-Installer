//! What a window does with the engine: ask for a snapshot, plan a
//! selection, and run an operation on a thread while it can be paused and
//! cancelled.

mod apply_request;
mod emit;
mod mode_view;
mod operation_error;
mod operations;
mod starting;

pub use apply_request::ApplyRequest;
pub use emit::Emit;
pub use mode_view::ModeView;
pub use operation_error::{OperationError, describe};
pub use operations::Operations;

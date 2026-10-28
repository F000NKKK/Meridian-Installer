//! The machine the installer runs on, as planning needs it.

mod detect;
mod detect_error;
mod library_scan;

pub use detect::detect;
pub use detect_error::DetectError;
pub use library_scan::{parse_library_cache, scan_libraries};

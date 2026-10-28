//! The folder Meridian Installer keeps everything in.

mod locate_error;
mod meridian_home;

pub use locate_error::LocateError;
pub use meridian_home::{HOME_VARIABLE, MeridianHome};

//! One module per subcommand.

mod install;
mod list;
mod plan;
mod planned;
mod remove;
mod status;
mod update;

pub(crate) use install::install;
pub(crate) use list::list;
pub(crate) use plan::plan;
pub(crate) use remove::remove;
pub(crate) use status::status;
pub(crate) use update::update;

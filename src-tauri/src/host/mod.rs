//! What the window and the backend share by name: the commands (a domain,
//! Substrate's infrastructure) and the events. `build.rs` includes these
//! files for the permissions and the window's copy (`src/host.generated.ts`).

pub(crate) mod commands;
pub(crate) mod events;

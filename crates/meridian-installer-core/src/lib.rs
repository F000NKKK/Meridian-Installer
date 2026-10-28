//! Meridian Installer's composition root, over Substrate's installer engine.
//!
//! - [`folder`]: the folder everything lives in (`~/.meridian`);
//! - [`host`]: the machine, as planning needs it;
//! - [`credentials`]: the GitHub token (`GITHUB_TOKEN`, `gh auth token`);
//! - [`catalog`]: the built-in catalog and its refresh from the latest
//!   Installer release;
//! - [`wiring`]: every port of the engine, with its adapter;
//! - [`engine`]: the use cases of the engine over one machine, catalog and host;
//! - [`display`]: what a window or a terminal shows of it, as serializable data.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod catalog;
pub mod credentials;
pub mod display;
pub mod engine;
pub mod folder;
pub mod formats;
pub mod host;
#[cfg(feature = "testing")]
pub mod testing;
pub mod wiring;

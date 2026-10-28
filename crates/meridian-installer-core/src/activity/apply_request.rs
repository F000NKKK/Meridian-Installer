//! What the Modify window asks for.

use serde::Deserialize;

use super::ModeView;
use crate::display::SelectionRequest;

/// What the Modify window asks for: install or modify `product` to match
/// `selection`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyRequest {
    /// The product's id.
    pub product: String,
    /// The checkboxes.
    pub selection: SelectionRequest,
    /// When downloads happen.
    #[serde(default)]
    pub mode: ModeView,
    /// Whether downloads stay in the cache.
    #[serde(default)]
    pub keep_downloads: bool,
}

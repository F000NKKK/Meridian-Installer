//! The Modify window's checkboxes, as the window sends them.

use serde::{Deserialize, Serialize};
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::ids::IdError;

/// The Modify window's checkboxes, as ids in text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionRequest {
    /// The workloads checked.
    pub workloads: Vec<String>,
    /// Recommended components left unchecked.
    pub unchecked: Vec<String>,
    /// Optional and individual components checked.
    pub checked: Vec<String>,
}

impl SelectionRequest {
    /// The selection the text names.
    ///
    /// # Errors
    ///
    /// [`IdError`] for the first text that is not an id.
    pub fn into_selection(self) -> Result<Selection, IdError> {
        Ok(Selection {
            workloads: self
                .workloads
                .iter()
                .map(|id| id.parse())
                .collect::<Result<_, _>>()?,
            unchecked: self
                .unchecked
                .iter()
                .map(|id| id.parse())
                .collect::<Result<_, _>>()?,
            checked: self
                .checked
                .iter()
                .map(|id| id.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<&Selection> for SelectionRequest {
    fn from(selection: &Selection) -> Self {
        let texts = |ids: &mut dyn Iterator<Item = String>| ids.collect();
        Self {
            workloads: texts(&mut selection.workloads.iter().map(ToString::to_string)),
            unchecked: texts(&mut selection.unchecked.iter().map(ToString::to_string)),
            checked: texts(&mut selection.checked.iter().map(ToString::to_string)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_selection_survives_the_round_trip() {
        let request = SelectionRequest {
            workloads: vec!["meridian.workload.game".into()],
            unchecked: vec![],
            checked: vec!["rust.targets".into()],
        };
        let selection = request.clone().into_selection().expect("valid ids");
        assert_eq!(SelectionRequest::from(&selection), request);
    }

    #[test]
    fn a_bad_id_is_refused() {
        let request = SelectionRequest {
            checked: vec!["Not An Id".into()],
            ..SelectionRequest::default()
        };
        assert!(request.into_selection().is_err());
    }
}

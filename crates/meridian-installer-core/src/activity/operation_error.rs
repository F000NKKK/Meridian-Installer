//! Why an operation could not start.

use std::error::Error;

use substrate_installer_app::ports::records::StoreError;
use substrate_installer_domain::ids::IdError;

/// Why an operation could not start (once it has, it ends with a `finished`
/// event).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OperationError {
    /// Another operation is running.
    #[error("another operation is running")]
    Busy,
    /// A name is not an id.
    #[error(transparent)]
    Id(#[from] IdError),
    /// The catalog has no such product.
    #[error("`{0}` is not a product of the catalog")]
    NotAProduct(String),
    /// The records could not be read.
    #[error(transparent)]
    Store(#[from] StoreError),
}

/// `error` and its causes, in one line for a person.
#[must_use]
pub fn describe(error: &dyn Error) -> String {
    let mut text = error.to_string();
    let mut cause = error.source();
    while let Some(own) = cause {
        text.push_str(": ");
        text.push_str(&own.to_string());
        cause = own.source();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_causes_follow_the_error() {
        let error = StoreError::Read("denied".into());
        assert_eq!(
            describe(&error),
            "the installed components could not be read: denied"
        );
    }
}

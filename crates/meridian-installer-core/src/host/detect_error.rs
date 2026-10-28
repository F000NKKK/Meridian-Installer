//! Why the host cannot be told.

/// Why the host cannot be told.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum DetectError {
    /// The operating system or the processor is not one the installer has a
    /// name for.
    #[error("this operating system or processor is not supported")]
    UnsupportedPlatform,
}

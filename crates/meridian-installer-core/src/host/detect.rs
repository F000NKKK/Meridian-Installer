//! Finding the host.

use substrate_installer_domain::platforms::Host;
use substrate_installer_process::host_platform;

use super::{DetectError, scan_libraries};

/// The machine this program runs on: its platform and the shared libraries
/// its system has.
///
/// # Errors
///
/// [`DetectError::UnsupportedPlatform`] when the platform has no name in the
/// installer's vocabulary.
pub fn detect() -> Result<Host, DetectError> {
    let platform = host_platform().ok_or(DetectError::UnsupportedPlatform)?;
    Ok(Host::new(platform).with_libraries(scan_libraries()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_machine_running_the_tests_is_a_host() {
        let host = detect().expect("tests run on a supported platform");
        assert_eq!(host.platform.os.to_string(), std::env::consts::OS);
    }
}

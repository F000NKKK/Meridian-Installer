//! The token the GitHub CLI holds.

use substrate_process::command;

/// What `gh auth token` prints, when `gh` is installed and signed in.
#[must_use]
pub fn gh_auth_token() -> Option<String> {
    let output = command("gh").args(["auth", "token"]).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|text| !text.is_empty())
}

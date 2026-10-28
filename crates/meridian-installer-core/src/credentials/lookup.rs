//! Choosing the token.

use substrate_installer_http::GithubToken;

use super::gh_auth_token;

/// The environment variable that holds a token.
const TOKEN_VARIABLE: &str = "GITHUB_TOKEN";

/// The token `from_environment` says, else the one `from_gh` finds (called
/// only when the environment has none); blank values are no token.
#[must_use]
pub fn github_token(
    from_environment: Option<&str>,
    from_gh: impl FnOnce() -> Option<String>,
) -> Option<GithubToken> {
    from_environment
        .and_then(GithubToken::new)
        .or_else(|| GithubToken::new(&from_gh()?))
}

/// The token of this process: `GITHUB_TOKEN`, else `gh auth token`.
#[must_use]
pub fn find_github_token() -> Option<GithubToken> {
    let variable = std::env::var(TOKEN_VARIABLE).ok();
    github_token(variable.as_deref(), gh_auth_token)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn the_environment_wins_and_gh_is_not_asked() {
        let asked = Cell::new(false);
        let token = github_token(Some(" abc "), || {
            asked.set(true);
            Some("def".into())
        });
        assert!(token.is_some());
        assert!(!asked.get());
    }

    #[test]
    fn a_blank_variable_falls_back_to_gh() {
        assert!(github_token(Some("  "), || Some("def\n".into())).is_some());
        assert!(github_token(None, || Some(String::new())).is_none());
        assert!(github_token(None, || None).is_none());
    }
}

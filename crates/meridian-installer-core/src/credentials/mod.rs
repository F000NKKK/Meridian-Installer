//! Finding a GitHub token, which lifts the API's limit from 60 requests an
//! hour to 5000.

mod gh_cli;
mod lookup;

pub use gh_cli::gh_auth_token;
pub use lookup::{find_github_token, github_token};

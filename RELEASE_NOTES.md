# Meridian Installer 0.2.3

Fixes downloading Meridian IDLE and other assets from private GitHub releases:

- Uses the asset API endpoint with binary content negotiation and the configured GitHub token.
- Removes authorization on redirects, so storage/CDN hosts never receive the GitHub token.
- Preserves resumable transfers and verifies the published SHA-256 digest.
- Keeps the underlying HTTP status in download errors.

Private repositories still require an account with access. Existing token discovery uses
`GITHUB_TOKEN`, or the token from `gh auth token`. The source repository's visibility is unchanged.

Includes substrate-installer-app/http 0.1.3; the CLI's public crates.io installation and
catalog revision 2 from 0.2.2 are retained.

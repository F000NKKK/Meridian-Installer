# Continue this work

The requested work is: release Meridian IDLE and the Engine SDK, fix Meridian Installer,
and make the Installer install itself. The user resumed local development while environment publication is deferred.
The original branches were pushed, but GitHub API access prevented creating PRs.
Releases have **not** been published.

## Changes

- Substrate-Platform branch `codex/installer-first-run-bundles`, commit `9467fd8`:
  retain the complete local AppImage/macOS app, register launchers, preserve Windows
  native-setup placement, and repair launchers idempotently. Six regression tests.
- This Installer change wires release-only first-run installation and persists the
  installed record; opening an older download cannot downgrade a newer install.
- Notifications now float below the title bar, wrap, scroll, dismiss and close with Escape.
- Plans are invalidated during recalculation, stale responses ignored, unread requirements
  block installation, and initial installation keeps its progress card visible.
- Pause/resume/cancel, failure reporting and self-update are covered by browser tests.

The Installer pins three installer crates to the above Substrate commit to use the new API.
Replace those patches with a published Substrate version after its next crates.io release.

## Environment and checks

Source `/workspace/.cloud-setup/env.sh` for Rust 1.99 and the local GTK/WebKit/ALSA sysroot.
The local installation script is `/workspace/.cloud-setup/install.sh`; neither it nor setup
requires resetting tracked files or creating a worktree. Existing checkouts are isolated.

Run in this repository:

```sh
npm ci
# With sibling checkouts directly under /workspace, repair the lockfile's UI link:
ln -sfn /workspace/Substrate-Platform/ui node_modules/substrate-platform-ui
npm run typecheck
npm test
npm run build
CHROMIUM_EXECUTABLE_PATH=/usr/bin/chromium npm run test:e2e
cargo test --locked --workspace --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
```

Browser tests use the actual frontend with a deterministic mock IPC boundary, not real
package downloads. Native Linux placement is tested using temporary real files and launchers.
Windows/macOS copies were checked through adapter tests on Linux, not native OS execution.

Do not export a shared `CARGO_TARGET_DIR` while running Meridian CLI tests: their nested
fixture packages reuse the same binary name and contaminate each other's artifacts.
Use `cargo test --target-dir /workspace/.cache/cargo-target` instead.

Git clone and push work through the platform HTTPS proxy. GitHub API requests in this
running instance return CONNECT 403. Secret requirements were saved for `GITHUB_TOKEN`
and `CARGO_REGISTRY_TOKEN`; bindings are saved in the draft but were absent in this running
process at the latest check. Never print secrets. Recheck runtime readiness and real API
access after configuration application before releasing anything.

## Product direction carried from the previous chat

A project has one `.mproj`, with no `.msln` or nested subprojects. Cargo is only an optional
native code module. ADR 050 in Engine remains Proposed in the inspected checkout; the user
selected option A. The next development phase is the data-driven runner, camera and render
settings, then converting examples. Viewport/glTF previews, picking, outlines, grids, undo,
redo and save were described in the supplied history; verify the current source before
claiming completion. Do not merge or publish based solely on that historical text.

## Local follow-up: operation recovery

The follow-up fixes removal of successfully uninstalled cards, visible self-update
failures with retry, retrying launch of an installed update, and errors from pause
and cancellation without reporting a state the backend did not accept. Playwright
regressions also cover release-refresh failure and retry. These follow-up changes
are local; no new pushes or publication were performed.

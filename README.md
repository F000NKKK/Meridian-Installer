<div align="center">

<img src="src-tauri/icons/128x128.png" alt="Meridian Installer" width="96">

# Meridian Installer

### Installs Meridian's SDK and tools as separate, versioned components

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Rust 1.99](https://img.shields.io/badge/Rust-1.99-orange.svg)](rust-toolchain.toml)
[![Tauri 2](https://img.shields.io/badge/Tauri-2-24C8DB.svg)](https://tauri.app/)
![Version 0.2.1](https://img.shields.io/badge/version-0.2.1-informational)

English | [Русский](README.ru.md)

</div>

---

## Overview

**Meridian Installer** is a desktop app, in the manner of Visual Studio Installer, that installs
[Meridian Engine](https://github.com/F000NKKK/Meridian-Engine)'s products and tools. It has an
Installed and an Available tab, product cards with Modify, Update and Uninstall, workloads
("Game development", "Engine development"), individual components, and a progress view with
pause and cancel. It is built over
[Substrate Platform](https://github.com/F000NKKK/Substrate-Platform)'s installer crates; this
repository holds the catalog, Meridian's home and the window.

| Part | Where | What it gives you |
|---|---|---|
| `meridian-installer-core` | [`crates/meridian-installer-core`](crates/meridian-installer-core) | The composition root: home layout, host detection, GitHub token, adapters, catalog, sessions and operations. |
| `meridian-installer` (CLI) | [`crates/meridian-installer-cli`](crates/meridian-installer-cli) | Headless `list`, `plan`, `install`, `update`, `remove`, `status`. |
| App | [`src-tauri`](src-tauri), [`src`](src) | The Tauri 2 backend and the React window, English and Russian. |
| Catalog | [`catalog/catalog.sdata`](catalog/catalog.sdata) | What can be installed: the Installer, Meridian IDLE, the CLI, the SDK, examples, assets, Rust. |

## What it installs

- **Meridian IDLE**, the IDE (a product).
- **Components**: the `meridian` CLI, the engine SDK (a tagged source release), the examples and
  sample assets, the Rust toolchain and the Rust targets.
- **Workloads** pick components for a job: *Game development* and *Engine development*.

Everything lives under `~/.meridian` (`MERIDIAN_HOME` moves it). GitHub's rate limit is lifted by
`GITHUB_TOKEN` or a signed-in `gh`. What a repository's release must publish is in the
[release contract](docs/release-contract.md).

## Development

Clone this repository and [Substrate-Platform](https://github.com/F000NKKK/Substrate-Platform)
side by side.

```bash
(cd ../Substrate-Platform/ui && npm ci)
npm install
npm run typecheck && npm test
cargo test --workspace --exclude meridian-installer   # the whole workspace where Tauri's libraries exist
cargo run -p meridian-installer-cli -- list           # the headless installer
npm run tauri dev                                      # the app
```

See [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md) and the [ADRs](docs/adr).

## Status

Early. The installer core and CLI are tested end to end with fakes; the app needs a first
real-world run. Not done yet: placing the Installer itself, resuming an interrupted installation
(see the TODOs in the [release contract](docs/release-contract.md)).

## License

[AGPL-3.0](LICENSE).

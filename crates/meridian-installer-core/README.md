# meridian-installer-core

Meridian Installer's composition root: it wires Substrate's installer engine
(`substrate-installer-*`) into one `Session` for the Installer's home folder
(`~/.meridian`), the built-in catalog and this machine, and turns the engine's
state into views that serialize for the window.

**Layer and context:** interface · installer

- For: the home layout (`MeridianHome`), host detection, the GitHub token
  (`GITHUB_TOKEN`, `gh auth token`), the built-in catalog and its refresh from
  the latest Installer release, the `Machine` of real adapters, the `Session`
  (list, plan, apply, update) and the `view` types the terminal and the
  Tauri window show.
- Not for: the use cases themselves (Substrate's `substrate-installer-app`), the
  adapters (the other `substrate-installer-*` crates), or any window code.

## Usage

```rust
use meridian_installer_core::catalog::built_in;

let catalog = built_in()?;
assert!(catalog.installer().is_some());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Depends on / used by

Depends on the `substrate-installer-*` crates by path (`../Substrate-Platform`).
Used by `meridian-installer-cli` and by the Tauri app in `src-tauri`.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).

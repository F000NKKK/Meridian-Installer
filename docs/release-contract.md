# Release contract

What each repository's release must publish for Meridian Installer to install it.
The catalog ([`catalog/catalog.sdata`](../catalog/catalog.sdata)) names the sources; this
page says what a source must look like. Files are Substrate Data
([ADR 002](adr/002-substrate-data-files.md)); JSON is only GitHub's API.

## Platform tags

An asset is chosen for the machine by the **platform tag** the catalog maps to the ending of the
asset's name:

| Tag | System |
|---|---|
| `linux-x86_64` | Linux, 64-bit x86 |
| `linux-aarch64` | Linux, arm64 |
| `windows-x86_64` | Windows, 64-bit x86 |
| `macos-x86_64` | macOS, Intel |
| `macos-aarch64` | macOS, Apple silicon |

Windows on arm64 has no tag yet (TODO: add `windows-aarch64` once the CLI ships it).

## Releases

A component's version is the release's tag with a leading `v` removed, and must be a semantic
version. Draft and pre-release releases are not offered. Every asset the installer downloads
must carry the SHA-256 digest GitHub computes for it (the `digest` field of the release asset);
the installer verifies a download against it and refuses a mismatch.

| Item | Repository | Asset (ending of the name) |
|---|---|---|
| Meridian Installer | `F000NKKK/Meridian-Installer` | `_amd64.AppImage` (Linux x86_64), `_x64-setup.exe` (Windows), `_universal.app.tar.gz` (both macOS tags), and `catalog.sdata` |
| Meridian IDLE | `F000NKKK/Meridian-IDLE` | the same three endings, plus `_amd64.deb` for systems with `dpkg` |
| `meridian.cli` | crates.io `meridian-cli` | installed by `cargo install --locked` after the Rust toolchain |
| `meridian.examples` | `F000NKKK/Meridian-Engine` | `meridian-examples.zip` |
| `meridian.assets` | `F000NKKK/Meridian-Engine` | `meridian-assets.zip` |
| `meridian.sdk` | `F000NKKK/Meridian-Engine` | none: the tag's source archive (a git-tag source) |

An archive holds the files at its root (or under one folder, which the unpacker strips). The CLI source is the public crates.io package, installed into Cargo's bin directory as `meridian` (`meridian.exe` on Windows). Its GitHub repository may remain private.

## The Installer's catalog

The Installer's own release carries `catalog.sdata`, the file in this repository's `catalog/`.
Installed copies refresh from the newest release that has it; a catalog is taken only when its
`revision` is greater than the one they already have (the built-in one counts). Raise `revision`
in the same commit that changes the catalog.

## Compatibility manifest (optional)

A release may carry `meridian-release.sdata`: what it needs of other components.

```
{
  requires: {
    "meridian.cli": ">=0.3.0",
    "meridian.sdk": ">=0.5.0",
  },
}
```

`requires` maps a component id to a version requirement (`>=`, `>`, `<`, `<=`, `=`, `^`, `~`
and comma-joined ranges). The format is strict: an unknown key or a malformed requirement makes the
manifest unreadable, and the installer then says so instead of guessing. A release with no
manifest has no requirements. The Modify window shows an unmet requirement as a conflict before
anything is installed.

## First launch

Release builds retain their local bundle without downloading it again.

- **Placement.** Release builds retain their local bundle without downloading it again:
  the original Linux AppImage under the Meridian home with a desktop launcher, the complete
  macOS `.app` with an Applications link, or the Windows executable placed by NSIS. Development
  builds skip this step. The installed path is recorded for launching and subsequent updates;
  opening an older download preserves a newer installed copy. A placement failure appears in
  the notifications without preventing the rest of the Installer from opening.
## TODO

- **Interrupted installations** are not resumed: Substrate has no pending journal, so an
  installation that stops half-way is retried from the start by running it again.
- **Empty folders** are left in the home after a removal.

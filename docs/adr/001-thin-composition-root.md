# ADR 001: A thin composition root over Substrate's installer crates

## Status

Accepted (2026-10-28).

## Context

Meridian needs an installer like Visual Studio Installer: products and components with versions,
workloads, plans, progress. Yog Installer already does this, and Substrate-Platform now has
the generic engine as `substrate-installer-*` crates (domain, app, http, archive, fs, process,
data). Another copy of the engine here would drift from them.

## Decision

1. Everything generic is Substrate's: the catalog model, planning, the installation use cases,
   downloads, unpacking, the home layout, the installed record. This repository holds what is
   about Meridian: the catalog, the home (`~/.meridian`, `MERIDIAN_HOME`), host detection,
   the GitHub token, the wiring of adapters and the user interface.
2. `meridian-installer-core` is the composition root (the interface layer in Craftbook's terms):
   `wiring` builds the real adapters, `engine::Session` runs the use cases, `activity::Operations`
   runs an operation on a thread and reports events. Nothing in it is a port or a domain rule.
3. The command-line (`meridian-installer-cli`) and the Tauri app (`src-tauri`) are two faces of
   the same core. Both are tested against fakes (`testing` feature) instead of the network.
4. A missing piece in Substrate is reported and written down as a TODO
   ([release contract](../release-contract.md)), never patched here.

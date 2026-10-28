# ADR 002: Our own files are Substrate Data

## Status

Accepted (2026-10-28). Direction set by the owner.

## Decision

The catalog (`catalog.sdata`), the release compatibility manifest (`meridian-release.sdata`),
the installed record (`~/.meridian/installed.sdata`, binary) and any other file this repository
owns (settings, window state) use Substrate Data, through `substrate-installer-data` and
`substrate-data`. JSON appears only where somebody else defines it: GitHub's API, the CLI's
`--json` output and Tauri's IPC. The only place that names the format is
`meridian-installer-core::formats`, so a change of format is a change of that module.

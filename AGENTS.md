# AGENTS.md — Meridian Installer operating guide

Read this before working in the repository. `CLAUDE.md` mirrors it.

## What this is

Meridian Installer installs Meridian's SDK and tools as separate, versioned components, in the
manner of Visual Studio Installer. It is a thin composition root over Substrate Platform's
installer crates ([ADR 001](docs/adr/001-thin-composition-root.md)):

- `crates/meridian-installer-core`: the home (`~/.meridian`), host detection, the GitHub token,
  the wiring of Substrate's adapters, the built-in catalog and its refresh, the engine `Session`
  and the `activity` operations the front ends call;
- `crates/meridian-installer-cli`: the headless `meridian-installer` binary
  (list, plan, install, update, remove, status);
- `src-tauri/`: the Tauri 2 app, one plugin (`meridian-installer`) over the core;
- `src/`: the React window on `substrate-platform-ui`;
- `catalog/catalog.sdata`: what can be installed, and from where.

Substrate-Platform comes from a sibling `../Substrate-Platform` checkout, by path.

## Architecture rules

- **Generic goes to Substrate.** Planning, downloads, unpacking, the installed record and the
  catalog model are Substrate's. A missing piece is reported and written as a TODO in
  [`docs/release-contract.md`](docs/release-contract.md), not copied here.
- **Our files are Substrate Data** ([ADR 002](docs/adr/002-substrate-data-files.md)). Only
  `meridian-installer-core::formats` names the format. JSON is for GitHub's API, the CLI's
  `--json` output and Tauri's IPC only. Do not add `toml` or `serde_json` for our own files.
- **The catalog is data.** A new component is an entry in `catalog/catalog.sdata` (raise
  `revision`); the release contract says what its repository must publish.
- **Commands per domain.** The command list is `src-tauri/src/host/commands.rs`; `build.rs`
  generates permissions and `src/host.generated.ts` from it. Every event is declared once in
  `host/events.rs`.
- **Async only for genuine I/O.** Operations run on a thread; the front end never blocks.
- **No Yog code.** Yog Installer is a model, not a dependency.
- Rust follows Craftbook (`.claude/rules/craftbook.md`): files of at most 400 lines, typed
  errors, no `unwrap`, `#[expect]` instead of `#[allow]`.

## Checks

`src-tauri` needs the Tauri system libraries; without them, exclude it.

```sh
npm run typecheck && npm test
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings   # --exclude meridian-installer without GTK
cargo test --workspace --all-features                                    # likewise
python3 <craftbook>/tools/craftcheck check . && python3 <craftbook>/tools/craftcheck deps .
```

Run `npm run tauri build -- --no-bundle` when you change the Tauri wiring, permissions or `build.rs`.

## Conventions

- Code reads like the code around it.
- Every new interface string gets a Russian translation in `src/locales/ru.ts`.
- Commits follow Conventional Commits and name the area: `feat(activity): …`, `fix(catalog): …`.
- Update docs in the same change that makes them wrong.

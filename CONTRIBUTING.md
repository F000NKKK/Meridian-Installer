# Contributing to Meridian Installer

Bug reports, ideas, documentation fixes and code are all welcome.

## Reporting a bug

Open an [issue](https://github.com/F000NKKK/Meridian-Installer/issues/new). Include what you
did, what you expected and what happened instead, your operating system, and the output of
`meridian-installer status --json` if you can run it.

## Making a change

1. Fork the repository and create a branch from `master`.
2. Clone [Substrate-Platform](https://github.com/F000NKKK/Substrate-Platform) next to it; the
   installer builds against it by relative path.
3. Make the change, with tests (the core and the CLI are tested with fakes, no network).
4. Check that everything builds and passes:

   ```bash
   npm install && npm run typecheck && npm test
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   npm run tauri dev        # run the app
   ```

5. Open a pull request that explains what changed and why.

A new component or product is an entry in [`catalog/catalog.sdata`](catalog/catalog.sdata)
(raise `revision`) and a line in the [release contract](docs/release-contract.md). Generic
installer behavior belongs in Substrate-Platform. Every new interface string needs a Russian
translation in `src/locales/ru.ts`.

## Style

- Rust follows [Craftbook](https://github.com/F000NKKK/craftbook); see `.claude/rules/craftbook.md`.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  `feat(scope): …`, `fix(scope): …`, `docs: …`, `ci: …`.

## License

By contributing you agree that your contribution is licensed under the [AGPL-3.0](LICENSE).

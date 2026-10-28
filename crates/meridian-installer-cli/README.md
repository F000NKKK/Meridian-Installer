# meridian-installer-cli

The headless Meridian Installer: the `meridian-installer` command, for a
terminal or CI, over `meridian-installer-core`'s session.

**Layer and context:** interface · installer

- For: `list`, `plan`, `install`, `update`, `remove` and `status`, with typed
  arguments, text or `--json` output and progress on standard error.
- Not for: any installing logic (Substrate's `substrate-installer-app`) or any
  wiring (`meridian-installer-core`).

## Usage

```text
meridian-installer list
meridian-installer plan meridian.workload.game --without meridian.examples
meridian-installer install meridian.workload.game
meridian-installer update
meridian-installer remove meridian.cli
meridian-installer status --json
```

`--home DIR` moves the home folder (default `MERIDIAN_HOME`, else
`~/.meridian`); `--no-refresh` skips the look for a newer catalog.

```rust
use clap::Parser;
use meridian_installer_cli::command_line::Cli;

let cli = Cli::try_parse_from(["meridian-installer", "status"])?;
assert!(!cli.json);
# Ok::<(), clap::Error>(())
```

## Depends on / used by

Depends on `meridian-installer-core`. Used by nobody: it is a binary.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).

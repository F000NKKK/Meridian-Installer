# meridian-installer

Meridian Installer's Tauri 2 app: the window (`../src`) over
`meridian-installer-core`. Its commands (`src/commands.rs`) are one-liners over
the core's `Operations`; the lists of commands and events are in `src/host/`,
and `build.rs` writes the window's copy (`../src/host.generated.ts`) and the
permissions from them.

**Layer and context:** interface · installer-app

- For: the window, the commands, the events, the bundle (AppImage, NSIS setup,
  universal macOS app).
- Not for: installing logic or wiring (`meridian-installer-core`).

## Depends on / used by

Depends on `meridian-installer-core` and Substrate's `tauri-plugin-substrate`.
Used by nobody: it is the app.

## License

AGPL-3.0-only; see [LICENSE](../LICENSE).

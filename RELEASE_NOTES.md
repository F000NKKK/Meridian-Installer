# Meridian Installer 0.2.1

Linux desktop integration fixes:

- Retains the bundled PNG next to the installed AppImage and registers it in the launcher.
- Adds the application description to the installed desktop entry and bundle metadata.
- Uses the desktop file identity `dev.meridian.installer` as the GTK/Wayland application id,
  allowing the desktop to associate the running window with its icon and launcher.
- Repairs the entry on repeated startup and removes the old empty launcher entry.
- The updated Linux process waits for its parent to exit before registering the GTK id.

Uses `substrate-installer-fs 0.1.3`; the other Substrate Installer packages remain at 0.1.2.

# Meridian Installer 0.2.4

Fixes missing launcher icons for privately installed Debian applications such as
Meridian IDLE. Launcher entries now resolve the PNG shipped inside the extracted
package's hicolor icon directory, rather than relying on a system-wide icon theme.
Explicit Installer presentation icons keep their existing priority.

Includes substrate-installer-fs 0.1.4. After updating Installer, reinstall the
IDLE package to regenerate its launcher entry. This fix covers the application
menu shortcut; window/compositor icon matching is a separate verification.

Private GitHub releases still require an authorized token. Download authentication,
redirect protection and SHA-256 verification from 0.2.3 are retained.

# Meridian Installer 0.2.2

- Catalog revision 2 installs Meridian CLI from its public crates.io package, so CLI 0.2.0 does not require private GitHub access. Cargo builds the CLI using the Rust toolchain workload.
- Resolves the installed executable from Cargo metadata when its name differs from the package name (`meridian-cli` installs `meridian`).
- SDK and examples use published Meridian Engine releases, including SDK 0.7.0; IDLE updates are offered once its completed release is published.
- Preserves the application icon, description and Linux desktop identity fixes from 0.2.1.

Uses substrate-installer-process 0.1.3 and substrate-installer-fs 0.1.3; other installer packages remain at 0.1.2.

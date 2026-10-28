//! The Installer's commands: the window calls them as `installer.installer.<command>`.

use substrate_host_contract::commands::Domain;

/// The Installer's one domain.
pub(crate) const INSTALLER: Domain = Domain {
    key: "installer",
    plugin: "meridian-installer",
    commands: &[
        "snapshot",
        "selection_of",
        "plan",
        "apply",
        "uninstall",
        "cancel",
        "pause",
        "launch",
        "self_update",
    ],
};

/// Every domain.
pub(crate) const ALL: &[&Domain] = &[&INSTALLER];

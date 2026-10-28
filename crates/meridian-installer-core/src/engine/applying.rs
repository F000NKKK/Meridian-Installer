//! Carrying out a plan.

use substrate_installer_app::EventSink;
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::installing::{ApplyError, ApplyOptions, Installer};
use substrate_installer_domain::planning::Plan;

use super::Session;

impl Session {
    /// Carries out `plan` through the machine, telling `sink` the progress;
    /// `cancel` pauses and cancels it.
    ///
    /// # Errors
    ///
    /// [`ApplyError`]: cancelled, or what stopped the plan (what was done
    /// stays done and recorded).
    pub fn apply(
        &self,
        plan: &Plan,
        options: ApplyOptions,
        cancel: &CancelToken,
        sink: &dyn EventSink,
    ) -> Result<(), ApplyError> {
        let adapters = self.machine.adapters(sink);
        Installer::new(adapters, &self.catalog, &self.host).apply(plan, options, cancel)
    }
}

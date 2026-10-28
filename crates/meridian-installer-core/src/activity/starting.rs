//! Starting an operation on a thread of its own.

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use substrate_installer_app::installing::{ApplyOptions, DownloadCache};
use substrate_installer_app::listing::Available;
use substrate_installer_app::plans::{Situation, for_program, for_removal};
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::ids::{ItemId, ProductId};
use substrate_installer_domain::inventory::Found;
use substrate_installer_domain::planning::Plan;

use super::operations::product_id;
use super::{ApplyRequest, Emit, OperationError, Operations};
use crate::display::EventView;
use crate::engine::Session;

/// The plan that makes `selection` installed, with `product` at its newest.
pub(super) fn plan_of(
    session: &Session,
    product: &ProductId,
    selection: &Selection,
    installed: &[Found],
    available: &Available,
) -> Plan {
    let mut plan = session.plan_selection(selection, installed, available);
    let own = for_program(
        &ItemId::Product(product.clone()),
        &Situation {
            installed,
            available,
        },
    );
    plan.changes.extend(own.changes);
    plan.releases.extend(own.releases);
    plan
}

/// What a thread carries out.
struct Job {
    session: Session,
    label: ProductId,
    plan: Plan,
    options: ApplyOptions,
}

impl Operations {
    /// Installs or modifies a product to match `request`, on a thread. The
    /// progress goes to `emit`, ending with a `finished` event.
    ///
    /// # Errors
    ///
    /// [`OperationError`] when another operation runs or the request does
    /// not name a product; failures after that are in the `finished` event.
    pub fn start_apply(
        self: &Arc<Self>,
        request: &ApplyRequest,
        emit: Emit,
    ) -> Result<JoinHandle<()>, OperationError> {
        let (session, available, installed) = self.situation()?;
        let selection = request.selection.clone().into_selection()?;
        let id = product_id(&session, &request.product)?;
        let plan = plan_of(&session, &id, &selection, &installed, &available);
        let options = ApplyOptions {
            mode: request.mode.into(),
            downloads: if request.keep_downloads {
                DownloadCache::Keep
            } else {
                DownloadCache::Discard
            },
        };
        self.spawn(
            Job {
                session,
                label: id,
                plan,
                options,
            },
            emit,
        )
    }

    /// Uninstalls a product (its components stay: others may use them), on a thread.
    ///
    /// # Errors
    ///
    /// As [`start_apply`](Self::start_apply).
    pub fn start_uninstall(
        self: &Arc<Self>,
        product: &str,
        emit: Emit,
    ) -> Result<JoinHandle<()>, OperationError> {
        let (session, _, installed) = self.situation()?;
        let id = product_id(&session, product)?;
        let plan = for_removal(&ItemId::Product(id.clone()), &installed);
        self.spawn(
            Job {
                session,
                label: id,
                plan,
                options: ApplyOptions::default(),
            },
            emit,
        )
    }

    /// Updates the Installer itself to its newest release, on a thread.
    ///
    /// # Errors
    ///
    /// As [`start_apply`](Self::start_apply); [`OperationError::NotAProduct`]
    /// when the catalog has no installer entry.
    pub fn start_self_update(
        self: &Arc<Self>,
        emit: Emit,
    ) -> Result<JoinHandle<()>, OperationError> {
        let (session, available, installed) = self.situation()?;
        let own = session
            .catalog()
            .installer()
            .map(|installer| installer.id.clone())
            .ok_or_else(|| OperationError::NotAProduct("installer".to_owned()))?;
        let plan = for_program(
            &ItemId::Product(own.clone()),
            &Situation {
                installed: &installed,
                available: &available,
            },
        );
        self.spawn(
            Job {
                session,
                label: own,
                plan,
                options: ApplyOptions::default(),
            },
            emit,
        )
    }

    fn spawn(self: &Arc<Self>, job: Job, emit: Emit) -> Result<JoinHandle<()>, OperationError> {
        let Job {
            session,
            label,
            plan,
            options,
        } = job;
        let cancel = self.begin()?;
        let operations = Arc::clone(self);
        Ok(thread::spawn(move || {
            let sink = |event: Event| {
                if let Some(view) = EventView::of(&event) {
                    emit(label.as_str(), view);
                }
            };
            // How it ended is in the `finished` event the engine tells.
            session.apply(&plan, options, &cancel, &sink).ok();
            operations.end();
        }))
    }
}

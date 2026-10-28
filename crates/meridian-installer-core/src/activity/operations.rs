//! The state a window holds between its commands.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::listing::Available;
use substrate_installer_domain::choice::Selection;
use substrate_installer_domain::ids::{ItemId, ProductId};
use substrate_installer_domain::inventory::Found;

use super::{ApplyRequest, OperationError};
use crate::display::{PlanView, SelectionRequest, Snapshot};
use crate::engine::Session;

/// What a window holds between its commands: the session, what was last
/// found published, and the token of the operation running.
#[derive(Debug)]
pub struct Operations {
    state: Mutex<State>,
    running: Mutex<Option<CancelToken>>,
}

#[derive(Debug)]
struct State {
    session: Session,
    available: Option<Arc<Available>>,
    notices: Vec<String>,
}

pub(super) fn locked<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Operations {
    /// Operations over `session`; `notices` are told with the first snapshot.
    #[must_use]
    pub fn new(session: Session, notices: Vec<String>) -> Self {
        Self {
            state: Mutex::new(State {
                session,
                available: None,
                notices,
            }),
            running: Mutex::new(None),
        }
    }

    /// The session now.
    #[must_use]
    pub fn session(&self) -> Session {
        locked(&self.state).session.clone()
    }

    /// Everything the window shows. With `refresh`, or the first time, the
    /// catalog is refreshed and every source is asked again; otherwise what
    /// was found last is shown with what is installed now.
    ///
    /// # Errors
    ///
    /// [`OperationError`] when the records cannot be read.
    pub fn snapshot(&self, refresh: bool) -> Result<Snapshot, OperationError> {
        let mut state = locked(&self.state);
        if refresh || state.available.is_none() {
            if refresh && let Err(error) = state.session.refresh_catalog(&CancelToken::new()) {
                state
                    .notices
                    .push(format!("the catalog was not refreshed: {error}"));
            }
            state.available = Some(Arc::new(state.session.available()));
        }
        let available = state.available.clone().unwrap_or_default();
        let installed = state.session.installed()?;
        Ok(Snapshot::of(
            &state.session,
            &installed,
            &available,
            state.notices.clone(),
        ))
    }

    /// The checkboxes that show what is installed of `product`.
    ///
    /// # Errors
    ///
    /// [`OperationError`] for a name that is not a product, or unreadable records.
    pub fn selection_of(&self, product: &str) -> Result<SelectionRequest, OperationError> {
        let session = self.session();
        let id = product_id(&session, product)?;
        let installed = session.installed()?;
        Ok(SelectionRequest::from(&Selection::of_installed(
            session.catalog(),
            &id,
            &installed,
        )))
    }

    /// What `request` would do, and whether it fits what the releases need.
    ///
    /// # Errors
    ///
    /// [`OperationError`] for a name that is not an id or a product, or unreadable records.
    pub fn plan(&self, request: &ApplyRequest) -> Result<PlanView, OperationError> {
        let (session, available, installed) = self.situation()?;
        let selection = request.selection.clone().into_selection()?;
        let id = product_id(&session, &request.product)?;
        let plan = super::starting::plan_of(&session, &id, &selection, &installed, &available);
        let compatibility =
            session.compatibility(&plan, &installed, &available, &CancelToken::new());
        Ok(PlanView::of(
            &session,
            &plan,
            Some(&selection),
            &compatibility,
        ))
    }

    /// Pauses (or resumes) the operation running, if one is.
    pub fn pause(&self, paused: bool) {
        if let Some(token) = locked(&self.running).as_ref() {
            if paused {
                token.pause();
            } else {
                token.resume();
            }
        }
    }

    /// Cancels the operation running, if one is.
    pub fn cancel(&self) {
        if let Some(token) = locked(&self.running).as_ref() {
            token.cancel();
        }
    }

    /// Where the installed product starts from.
    ///
    /// # Errors
    ///
    /// [`OperationError`] for a name that is not a product, or unreadable records.
    pub fn program_of(&self, product: &str) -> Result<Option<std::path::PathBuf>, OperationError> {
        let session = self.session();
        let id = ItemId::Product(product_id(&session, product)?);
        let installed = session.installed()?;
        Ok(installed
            .into_iter()
            .filter(|found| found.id == id)
            .max_by(|a, b| a.version.compare(&b.version))
            .and_then(|found| found.path))
    }

    /// The session, what is published and what is installed.
    pub(super) fn situation(
        &self,
    ) -> Result<(Session, Arc<Available>, Vec<Found>), OperationError> {
        let mut state = locked(&self.state);
        let available = state
            .available
            .get_or_insert_with(|| Arc::new(Available::new()))
            .clone();
        let installed = state.session.installed()?;
        Ok((state.session.clone(), available, installed))
    }

    /// Reserves the right to run an operation: its token.
    pub(super) fn begin(&self) -> Result<CancelToken, OperationError> {
        let mut running = locked(&self.running);
        if running.is_some() {
            return Err(OperationError::Busy);
        }
        let token = CancelToken::new();
        *running = Some(token.clone());
        Ok(token)
    }

    /// Gives the right back.
    pub(super) fn end(&self) {
        *locked(&self.running) = None;
    }
}

/// `text` as the id of a product (or the Installer) of the catalog.
pub(super) fn product_id(session: &Session, text: &str) -> Result<ProductId, OperationError> {
    let id = ProductId::new(text)?;
    session
        .catalog()
        .item(&ItemId::Product(id.clone()))
        .map(|_| id)
        .ok_or_else(|| OperationError::NotAProduct(text.to_owned()))
}

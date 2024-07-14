use axum::Router;
use std::sync::Arc;

use crate::state::ApplicationState;

mod handlers;
mod v1;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}

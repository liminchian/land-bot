use axum::{extract::State, http::StatusCode};
use std::sync::Arc;

use crate::state::ApplicationState;

pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello, World!設置檔案在 {}\n\n",
        state.settings.load().config.location.clone().unwrap()
    ))
}

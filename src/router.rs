use std::sync::Arc;

use axum::{debug_handler, response::IntoResponse, routing::get, Router};

use crate::{App, AppState};

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(AppState(app))
}

#[debug_handler(state = AppState)]
async fn root(AppState(_): AppState) -> impl IntoResponse {
    "Hello, world!"
}

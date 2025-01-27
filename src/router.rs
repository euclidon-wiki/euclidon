use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{controllers::root, App, AppState};

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .route("/", get(root::get))
        .with_state(AppState(app))
}

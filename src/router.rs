use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    controllers::{assets, root},
    App, AppState,
};

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .route("/", get(root::get))
        .route("/assets/{*path}", get(assets::get))
        .with_state(AppState(app))
}

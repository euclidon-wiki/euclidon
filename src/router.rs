use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    controllers::{assets, root, wiki},
    App, AppState,
};

pub fn build_router(app: Arc<App>) -> Router {
    build_base_router()
        .nest("/w", build_wiki_router())
        .with_state(AppState(app))
}

fn build_base_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root::get))
        .route("/assets/{*path}", get(assets::get))
}

fn build_wiki_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root::get))
        .route("/login", get(wiki::login::get).post(wiki::login::post))
}

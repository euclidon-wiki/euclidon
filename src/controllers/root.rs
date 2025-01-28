use axum::{
    debug_handler,
    response::{Html, IntoResponse, Response},
};
use serde_json::json;
use tera::Context;

use crate::{AppState, Error};

#[debug_handler(state = AppState)]
pub async fn get(AppState(app): AppState) -> Result<Response, Error> {
    Ok(Html::from(
        app.renderer
            .render("index", &Context::from_serialize(json!({}))?)?,
    )
    .into_response())
}

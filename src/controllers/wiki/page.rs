use axum::{
    extract::Path,
    response::{Html, IntoResponse, Redirect, Response},
};
use serde_json::json;
use tera::Context;

use crate::{AppState, Error};

pub async fn get(AppState(app): AppState, Path(path): Path<String>) -> Result<Response, Error> {
    let path = path.trim().replace(char::is_whitespace, "_");
    let title = path.replace('_', " ");
    Ok(Html::from(Response::builder().body(app.renderer.render(
        "page/view",
        &Context::from_serialize(json!({
            "site": {
                "title": "Euclidon"
            },
            "page": {
                "title": title,
            }
        }))?,
    )?)?)
    .into_response())
}

pub async fn post(AppState(_app): AppState, Path(path): Path<String>) -> Result<Response, Error> {
    let path = path.trim().replace(char::is_whitespace, "_");
    Ok(Redirect::to(&format!("w/page/{path}")).into_response())
}

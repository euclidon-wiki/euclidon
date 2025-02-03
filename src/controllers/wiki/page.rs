use axum::{
    debug_handler,
    extract::{Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use serde_json::json;
use tera::Context;

use crate::{AppState, Error};

#[debug_handler(state = AppState)]
pub async fn get(
    AppState(app): AppState,
    Path(path): Path<String>,
    Query(action): Query<Action>,
) -> Result<Response, Error> {
    let path = path.trim().replace(char::is_whitespace, "_");
    let template = match action.kind {
        ActionKind::View | ActionKind::Submit => "page/view",
        ActionKind::Edit => "page/edit",
    };

    let title = path.replace('_', " ");
    Ok(Html::from(Response::builder().body(app.renderer.render(
        template,
        &Context::from_serialize(json!({
            "site": {
                "title": "Euclidon"
            },
            "page": {
                "title": title,
                "content": "Hello, there!\n\nNext line"
            }
        }))?,
    )?)?)
    .into_response())
}

#[debug_handler(state = AppState)]
pub async fn post(
    AppState(_app): AppState,
    Path(path): Path<String>,
    Query(action): Query<Action>,
) -> Result<Response, Error> {
    let path = path.trim().replace(char::is_whitespace, "_");

    let query = match action.kind {
        ActionKind::View | ActionKind::Submit => "",
        ActionKind::Edit => "&action=edit",
    };

    Ok(Redirect::to(&format!("w/page/{path}{query}")).into_response())
}

#[derive(Debug, Deserialize)]
pub struct Action {
    #[serde(default, rename = "action")]
    pub kind: ActionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionKind {
    #[default]
    View,
    Edit,
    Submit,
}

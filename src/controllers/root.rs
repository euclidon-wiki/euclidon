use axum::{
    debug_handler,
    response::{IntoResponse, Response},
};

use crate::Error;

#[debug_handler(state = AppState)]
pub async fn get() -> Result<Response, Error> {
    Ok("Hello, world!".into_response())
}

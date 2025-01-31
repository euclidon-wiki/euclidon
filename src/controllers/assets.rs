use axum::{debug_handler, extract::Path, http::header::CONTENT_TYPE, response::Response};

use crate::{asset::Loc, AppState, Error};

#[debug_handler(state = AppState)]
pub async fn get(AppState(app): AppState, Path(path): Path<String>) -> Result<Response, Error> {
    let asset = app.assets.load(Loc::new(path))?;
    Ok(Response::builder()
        .header(CONTENT_TYPE, asset.kind.mime_type())
        .body(asset.data.to_vec().into())?)
}

use axum::response::{IntoResponse, Response};

use crate::asset::AssetError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Asset(#[from] AssetError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),

    #[error(transparent)]
    Tera(#[from] tera::Error),

    #[error(transparent)]
    Pool(#[from] diesel::r2d2::PoolError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("Internal server error: {self}").into_response()
    }
}

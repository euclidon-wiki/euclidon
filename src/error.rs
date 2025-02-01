use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),

    #[error(transparent)]
    Http(#[from] axum::http::Error),

    #[error(transparent)]
    Tera(#[from] tera::Error),

    #[error(transparent)]
    Pool(#[from] diesel::r2d2::PoolError),
    #[error(transparent)]
    Query(#[from] diesel::result::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("Internal server error: {self}").into_response()
    }
}

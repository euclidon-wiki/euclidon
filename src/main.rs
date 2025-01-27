use std::env;

use axum::{
    debug_handler,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use euclidon::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenvy::dotenv()?;

    let router = Router::new().route("/", get(root));

    let server_url = env::var("SERVER_URL")?;
    let listener = TcpListener::bind(&server_url).await?;

    println!("> server listening on: {server_url}");
    Ok(axum::serve(listener, router).await?)
}

#[debug_handler]
async fn root() -> Result<Response, Error> {
    Ok("Hello, world!".into_response())
}

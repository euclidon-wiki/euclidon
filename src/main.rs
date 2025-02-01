use std::sync::Arc;

use euclidon::{app::Config, App, Error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenvy::dotenv()?;

    let app = Arc::new(App::new(Config::load()?)?);
    let router = euclidon::build_router(app.clone());
    _ = euclidon::spawn_tasks(app.clone());

    let server_url = &app.config.server_url;
    let listener = TcpListener::bind(server_url).await?;

    println!("> server listening on: {server_url}");
    Ok(axum::serve(listener, router).await?)
}

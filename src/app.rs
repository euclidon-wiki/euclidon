use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

use crate::{db::Db, Error};

use self::detail::ConfigBuilder;

pub struct App {
    pub db: Db,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(Self {
            db: Db::new(&config)?,
            config,
        })
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &App {
        &self.0
    }
}

pub struct Config {
    pub server_url: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        Self::builder().build()
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[doc(hidden)]
mod detail {
    use crate::{app::Config, Error};

    #[derive(Default)]
    pub struct ConfigBuilder {
        server_url: Option<String>,
        database_url: Option<String>,
    }

    impl ConfigBuilder {
        pub fn build(self) -> Result<Config, Error> {
            Ok(Config {
                server_url: self
                    .server_url
                    .map_or_else(|| std::env::var("SERVER_URL"), Ok)?,
                database_url: self
                    .database_url
                    .map_or_else(|| std::env::var("DATABASE_URL"), Ok)?,
            })
        }

        pub fn server_url(mut self, server_url: String) -> Self {
            self.server_url = Some(server_url);
            self
        }
    }
}

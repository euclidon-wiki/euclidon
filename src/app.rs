use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

use crate::Error;

use self::detail::ConfigBuilder;

pub struct App {
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(Self { config })
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
    }

    impl ConfigBuilder {
        pub fn build(self) -> Result<Config, Error> {
            Ok(Config {
                server_url: self
                    .server_url
                    .map_or_else(|| std::env::var("SERVER_URL"), Ok)?,
            })
        }

        pub fn server_url(mut self, server_url: String) -> Self {
            self.server_url = Some(server_url);
            self
        }
    }
}

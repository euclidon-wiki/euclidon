use std::{ops::Deref, path::PathBuf, sync::Arc};

use axum::extract::{FromRequestParts, State};

use crate::{asset::Assets, db::Db, render::Renderer, Error};

use self::detail::ConfigBuilder;

pub struct App {
    pub config: Config,

    pub assets: Assets,
    pub renderer: Renderer,
    pub db: Db,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        let assets = Assets::new(&config);
        Ok(Self {
            db: Db::new(&config)?,
            renderer: Renderer::new(&assets)?,
            assets,

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

    pub assets_dir: PathBuf,
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
    use std::path::PathBuf;

    use crate::{app::Config, Error};

    #[derive(Default)]
    pub struct ConfigBuilder {
        server_url: Option<String>,
        database_url: Option<String>,

        assets_dir: Option<PathBuf>,
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

                assets_dir: self
                    .assets_dir
                    .unwrap_or_else(|| PathBuf::from("assets/")),
            })
        }

        pub fn server_url(mut self, server_url: String) -> Self {
            self.server_url = Some(server_url);
            self
        }

        pub fn database_url(mut self, database_url: String) -> Self {
            self.database_url = Some(database_url);
            self
        }

        pub fn assets_dir(mut self, assets_dir: PathBuf) -> Self {
            self.assets_dir = Some(assets_dir);
            self
        }
    }
}

mod loc;

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub use self::loc::Loc;
use crate::{app::Config, Error};

#[derive(Debug, Clone)]
pub struct Asset {
    pub kind: AssetKind,
    pub data: Box<[u8]>,
}

impl Asset {
    fn new(kind: AssetKind, data: Box<[u8]>) -> Self {
        Self { kind, data }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssetKind {
    #[default]
    None,
    Json,

    Css,
    JavaScript,
}

impl AssetKind {
    pub fn from_extension(extension: Option<&str>) -> Self {
        match extension {
            Some("css") => Self::Css,
            Some("js") => Self::JavaScript,
            Some("json") => Self::Json,

            _ => Self::None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::None => "application/octet-stream",
            Self::Json => "application/json",

            Self::Css => "text/css",
            Self::JavaScript => "text/javascript",
        }
    }
}

pub struct Assets {
    root: PathBuf,

    cache: Cache,
}

impl Assets {
    pub fn new(config: &Config) -> Self {
        Self {
            root: config.assets_dir.clone(),

            cache: Cache::new(),
        }
    }

    pub fn load_transient(&self, loc: &Loc) -> Result<Arc<Asset>, Error> {
        let path = self.root.join(&loc.path);
        Ok(Arc::new(Asset::new(
            AssetKind::from_extension(path.extension().map(OsStr::to_str).flatten()),
            fs::read(path)?.into_boxed_slice(),
        )))
    }

    pub fn load(&self, loc: Loc) -> Result<Arc<Asset>, Error> {
        self.cache.get_or_else(loc, |loc| self.load_transient(loc))
    }

    pub fn reload(&mut self, loc: Loc) -> Result<Arc<Asset>, Error> {
        _ = self.cache.remove(&loc);
        self.load(loc)
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear()
    }
}

struct Cache {
    cache: RwLock<HashMap<Loc, Arc<Asset>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_or_else<F>(&self, loc: Loc, load: F) -> Result<Arc<Asset>, Error>
    where
        F: FnOnce(&Loc) -> Result<Arc<Asset>, Error>,
    {
        Ok(if let Some(asset) = self.get(&loc) {
            asset
        } else {
            let asset = load(&loc)?;
            self.cache
                .write()
                .expect("RwLock poisoned")
                .entry(loc)
                .insert_entry(asset)
                .get()
                .clone() // clone the Arc, not the asset
        })
    }

    pub fn get(&self, loc: &Loc) -> Option<Arc<Asset>> {
        self.cache
            .read()
            .expect("RwLock poisoned")
            .get(loc)
            .cloned() // clone the Arc, not the asset
    }

    pub fn remove(&mut self, loc: &Loc) -> Option<Arc<Asset>> {
        self.cache.write().expect("RwLock poisoned").remove(loc)
    }

    pub fn clear(&mut self) {
        self.cache.write().expect("RwLock poisoned").clear();
    }
}

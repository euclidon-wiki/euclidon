mod loc;

use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
    path::PathBuf,
};

pub use self::loc::Loc;
use crate::{app::Config, Error};

#[derive(Debug, Clone)]
pub struct Asset {
    pub data: Box<[u8]>,
}

impl Asset {
    fn new(data: Box<[u8]>) -> Self {
        Self { data }
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

    pub fn load_transient(&self, loc: &Loc) -> Result<Asset, Error> {
        Ok(Asset::new(
            fs::read(self.root.join(&loc.path))?.into_boxed_slice(),
        ))
    }

    pub fn load(&mut self, loc: Loc) -> Result<&mut Asset, Error> {
        self.cache.cached_load(loc, |loc| {
            // because partial borrow of self is not yet in stable Rust, you can't simply
            // call self.load_transient(loc) here, even though their function bodies are
            // exactly the same.
            // Fix your language, Rust!
            Ok(Asset::new(
                fs::read(self.root.join(&loc.path))?.into_boxed_slice(),
            ))
        })
    }

    pub fn reload(&mut self, loc: Loc) -> Result<&mut Asset, Error> {
        _ = self.cache.remove(&loc);
        self.load(loc)
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear()
    }
}

pub struct Cache {
    cache: HashMap<Loc, Asset>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn cached_load<F>(&mut self, loc: Loc, load: F) -> Result<&mut Asset, Error>
    where
        F: FnOnce(&Loc) -> Result<Asset, Error>,
    {
        Ok(match self.cache.entry(loc) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let asset = load(e.key())?;
                e.insert(asset)
            }
        })
    }

    pub fn remove(&mut self, loc: &Loc) -> Option<Asset> {
        self.cache.remove(loc)
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

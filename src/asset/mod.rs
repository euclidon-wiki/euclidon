mod error;
mod loc;

use std::{collections::HashMap, fs, iter::once, path::PathBuf};

pub use self::{
    error::AssetError,
    loc::{Loc, Ns},
};
use crate::app::Config;

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
    namespaces: HashMap<Ns, PathBuf>,
}

impl Assets {
    pub fn new(config: &Config) -> Self {
        let namespaces = Self::load_namespaces(config.assets_dir.clone());
        Self { namespaces }
    }

    pub fn load(&self, loc: Loc) -> Result<Asset, AssetError> {
        let path = self.path_of(&loc)?;
        Ok(Asset::new(fs::read(path)?.into_boxed_slice()))
    }

    pub fn path_of(&self, loc: &Loc) -> Result<PathBuf, AssetError> {
        self.namespaces
            .get(&loc.namespace)
            .ok_or(AssetError::Ns(loc.namespace))
            .map(|root| root.join(&loc.path))
    }
}

impl Assets {
    fn load_namespaces(assets_dir: PathBuf) -> HashMap<Ns, PathBuf> {
        HashMap::from_iter(once((Ns::EUCLIDON, assets_dir)))
    }
}

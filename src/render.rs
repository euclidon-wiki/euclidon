use tera::{Context, Tera};

use crate::{
    asset::{Assets, Loc, Ns},
    Error,
};

pub struct Renderer {
    tera: Tera,
}

impl Renderer {
    pub fn new(assets: &Assets) -> Result<Self, Error> {
        Ok(Self {
            tera: Self::new_tera(assets)?,
        })
    }

    pub fn render(&self, name: &str, context: &Context) -> Result<String, Error> {
        Ok(self.tera.render(name, context)?)
    }
}

impl Renderer {
    fn new_tera(assets: &Assets) -> Result<Tera, Error> {
        let mut tera = Tera::default();
        let asset = assets.load(Loc::new(
            Ns::EUCLIDON,
            "templates/index.html.tera".to_string(),
        ))?;

        tera.add_raw_templates(vec![(
            "index",
            std::str::from_utf8(&asset.data)
                .unwrap_or_else(|e| panic!("template not valid utf-8: {e}")),
        )])?;

        Ok(tera)
    }
}

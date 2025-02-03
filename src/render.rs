use tera::{Context, Tera};

use crate::{
    asset::{Assets, Loc},
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
        tera.add_raw_templates(vec![
            Self::load_template(
                assets,
                "index".to_string(),
                "templates/index.html.tera".to_string(),
            )?,
            Self::load_template(
                assets,
                "login".to_string(),
                "templates/login.html.tera".to_string(),
            )?,
            Self::load_template(
                assets,
                "page/base".to_string(),
                "templates/page/base.html.tera".to_string(),
            )?,
            Self::load_template(
                assets,
                "page/view".to_string(),
                "templates/page/view.html.tera".to_string(),
            )?,
            Self::load_template(
                assets,
                "page/edit".to_string(),
                "templates/page/edit.html.tera".to_string(),
            )?,
            Self::load_template(
                assets,
                "page/not-found".to_string(),
                "templates/page/not-found.html.tera".to_string(),
            )?,
        ])?;

        Ok(tera)
    }

    fn load_template(
        assets: &Assets,
        name: String,
        path: String,
    ) -> Result<(String, String), Error> {
        Ok((
            name,
            std::str::from_utf8(&assets.load_transient(&Loc::new(path))?.data)
                .unwrap_or_else(|e| panic!("template not valid utf-8: {e}"))
                .to_string(),
        ))
    }
}

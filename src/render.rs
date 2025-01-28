use tera::{Context, Tera};

use crate::Error;

pub struct Renderer {
    tera: Tera,
}

impl Renderer {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            tera: Self::new_tera()?,
        })
    }

    pub fn render(&self, name: &str, context: &Context) -> Result<String, Error>
    {
        Ok(self.tera.render(name, context)?)
    }
}

impl Renderer {
    fn new_tera() -> Result<Tera, Error> {
        let mut tera = Tera::default();
        tera.add_template_files(vec![(
            "assets/euclidon/templates/index.html.tera",
            Some("index"),
        )])?;

        Ok(tera)
    }
}

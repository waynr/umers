use std::include_str;

use tera::{Context, Tera};

use crate::errors::Result;

pub struct Generator {
    tera: Tera,
}

impl Generator {
    pub fn new() -> Result<Self> {
        let standard_tmpl = include_str!("templates/standard.tex");
        let mut tera = Tera::default();

        tera.add_raw_template("standard.tex", standard_tmpl)?;

        Ok(Self { tera })
    }

    pub fn generate(&mut self, ctx: &Context) -> Result<String> {
        Ok(self.tera.render("standard.tex", ctx)?)
    }
}

use std::fs::File;
use std::include_str;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use tera::{Context, Tera};

use crate::errors::Error;
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

    pub fn generate(&mut self, ctx: &Context, output_path: &PathBuf) -> Result<()> {
        match output_path.extension() {
            Some(v) => match v.to_str() {
                Some("pdf") => self.generate_pdf(ctx, output_path),
                Some("tex") => self.generate_tex(ctx, output_path),
                _ => Err(Error::UnsupportedOutputFileType(
                    v.to_string_lossy().to_string(),
                )),
            },
            None => Err(Error::EmptyOutputPath),
        }
    }

    pub fn generate_tex(&mut self, ctx: &Context, output_path: &PathBuf) -> Result<()> {
        let output = self.tera.render("standard.tex", ctx)?;
        let mut output_file = File::create(output_path)?;
        output_file.write_all(output.as_bytes())?;
        Ok(())
    }

    pub fn generate_pdf(&mut self, ctx: &Context, output_path: &PathBuf) -> Result<()> {
        let tex_path = output_path.with_extension("tex");
        self.generate_tex(ctx, &tex_path)?;
        let tex_path_str = tex_path
            .to_str()
            .ok_or_else(|| Error::InvalidPathError(tex_path.clone()))?;
        Command::new("latexmk")
            .args(["-pdf", tex_path_str])
            .output()?;
        Ok(())
    }
}

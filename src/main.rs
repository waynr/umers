use std::include_str;
use std::path::PathBuf;

use clap;
use tera::{Context, Tera};

#[macro_use]
extern crate lazy_static;

mod data;
use crate::data::Data;

mod errors;
use crate::errors::{Error, Result};

fn cli() -> Result<clap::ArgMatches> {
    // TODO: figure out how to retrieve cargo info here
    //Ok(clap::Command::new(clap::crate_name())
    //    .author(clap::crate_authors("\n"))
    //
    //    .version(clap::crate_version())
    //    .about(clap::crate_description())
    //

    let mut cmd = clap::Command::new("umers")
        .author("wayne warren")
        .version("0.0.1")
        .about("fancy resume generator")
        .args([clap::Arg::new("verbose")
            .short('v')
            .help("verbosity")
            .action(clap::ArgAction::Count)]);

    cmd = cmd.subcommand(
        clap::Command::new("generate")
            .about("generate")
            .arg(clap::Arg::new("data_files").action(clap::ArgAction::Append)),
    );

    Ok(cmd.get_matches())
}

fn generate_impl(matches: &clap::ArgMatches) -> Result<()> {
    let values = match matches.try_get_many::<String>("data_files")? {
        Some(vs) => vs,
        None => return Err(Error::NoDataFiles),
    };

    let mut d = Data::new();
    for v in values {
        let path = PathBuf::from(v);
        let tmp = Data::try_from(path.clone())?;
        d = d.merge(&tmp);
    }

    let standard_tmpl = include_str!("templates/standard.tex");
    let mut tera = Tera::default();
    tera.add_raw_template("standard.tex", standard_tmpl)?;
    for template in tera.get_template_names() {
        println!("{:0}", template);
    }

    let context = Context::from_serialize(d)?;
    let output = tera.render("standard.tex", &context)?;
    println!("{:0}", output);
    Ok(())
}

fn main() -> Result<()> {
    let matches = cli()?;

    match matches.subcommand() {
        Some(("generate", submatches)) => generate_impl(submatches)?,
        _ => (),
    };
    Ok(())
}

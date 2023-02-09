use std::include_str;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use clap;
use tera::{Context, Tera};

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
            .arg(
                clap::Arg::new("data_files")
                    .help("yaml input data files")
                    .action(clap::ArgAction::Append),
            )
            .arg(
                clap::Arg::new("output_file")
                    .long("output-file")
                    .short('o')
                    .action(clap::ArgAction::Set),
            ),
    );

    Ok(cmd.get_matches())
}

fn generate_impl(matches: &clap::ArgMatches) -> Result<()> {
    let data_files = match matches.try_get_many::<String>("data_files")? {
        Some(vs) => vs,
        None => return Err(Error::NoDataFiles),
    };

    let default_output_filename = String::from("output.tex");
    let output_filename = match matches.try_get_one::<String>("output_file")? {
        Some(v) => v,
        None => &default_output_filename,
    };

    let mut d = Data::new();
    for data_file_name in data_files {
        let path = PathBuf::from(data_file_name);
        let tmp = Data::try_from(path.clone())?;
        d = d.merge(&tmp);
    }

    let standard_tmpl = include_str!("templates/standard.tex");
    let mut tera = Tera::default();
    tera.add_raw_template("standard.tex", standard_tmpl)?;

    let context = Context::from_serialize(d)?;
    let output = tera.render("standard.tex", &context)?;
    let mut output_file = File::create(output_filename)?;
    output_file.write_all(output.as_bytes())?;
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

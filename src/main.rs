use std::fs::File;
use std::io::prelude::*;

use clap;

extern crate lazy_static;

mod data;
use crate::data::Data;

mod generator;
use crate::generator::Generator;

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

    let context = Data::try_from(data_files)?.context()?;
    let mut generator = Generator::new()?;
    let output = generator.generate(&context)?;
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

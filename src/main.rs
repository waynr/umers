use std::path::PathBuf;

use clap;

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
    for v in values {
        let path = PathBuf::from(v);
        let d = Data::try_from(path.clone())?;
    }
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

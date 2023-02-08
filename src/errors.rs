use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing data files")]
    NoDataFiles,

    /// Wrapper around `io::Error`.
    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("matches error")]
    ClapMatchesError(#[from] clap::parser::MatchesError),
    
    #[error("parsing yaml")]
    SerdeYamlError(#[from] serde_yaml::Error),
}

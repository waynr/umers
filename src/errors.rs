use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing data files")]
    NoDataFiles,

    #[error("empty output path")]
    EmptyOutputPath,

    #[error("unsupported output file type '{0}'")]
    UnsupportedOutputFileType(String),

    #[error("invalid path {0}")]
    InvalidPathError(std::path::PathBuf),

    /// Wrapper around `io::Error`.
    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("matches error")]
    ClapMatchesError(#[from] clap::parser::MatchesError),
    
    #[error("parsing yaml")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("populating context from serializable")]
    TeraError(#[from] tera::Error),
}

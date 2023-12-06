use std::{error::Error, fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum RunnerError {
    IoError { path: PathBuf, err: io::Error },
    DownloadError(reqwest::Error),
}

impl Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RunnerError::*;

        match self {
            IoError { path, err } => write!(f, "I/O error at {path:?}: {err}"),
            DownloadError(err) => write!(f, "Error downloading input: {err}"),
        }
    }
}

impl Error for RunnerError {}

pub type RunnerResult<T> = Result<T, RunnerError>;

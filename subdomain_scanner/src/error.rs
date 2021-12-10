use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: sub_scan <google.com>")]
    CliUsage,

    #[error("Reqwest: {0}")]
    Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

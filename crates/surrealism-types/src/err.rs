use anyhow::Result;
use surrealdb::sql::Kind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tried to reconstruct a datetime, but it is out of bounds")]
    InvalidDatetime,

    #[error("Found `{0}` but expected a value of kind `{1}`")]
    UnexpectedType(Kind, Kind),

    #[error("Expected to recieve {0} arguments, but instead got {1}")]
    InvalidArgs(usize, usize),

    #[error("Tried to transfer a kind which is not supported")]
    UnsupportedKind,
}

pub trait PrefixError<T> {
    fn prefix_err<F, S>(self, prefix: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: std::fmt::Display;
}

impl<T, E> PrefixError<T> for std::result::Result<T, E>
where
    E: std::fmt::Display + Send + Sync + 'static,
{
    fn prefix_err<F, S>(self, prefix: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: std::fmt::Display,
    {
        self.map_err(|e| anyhow::anyhow!(format!("{}: {}", prefix(), e)))
    }
}

impl<T> PrefixError<T> for Option<T> {
    fn prefix_err<F, S>(self, prefix: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: std::fmt::Display,
    {
        self.ok_or_else(|| anyhow::anyhow!(format!("{}: None", prefix())))
    }
}

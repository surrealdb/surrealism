use thiserror::Error;
use surrealdb::sql::Kind;

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
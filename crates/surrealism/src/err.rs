use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to operate on function registry")]
    RegistryLocked,
}

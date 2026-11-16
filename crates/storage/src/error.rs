use std::io;

use thiserror::Error;

pub type Result<T, E = StorageError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum StorageError {
    /// Object store error
    #[error(transparent)]
    ObjectStoreError(#[from] object_store::Error),
    /// I/O error
    #[error(transparent)]
    Io(#[from] io::Error),
}

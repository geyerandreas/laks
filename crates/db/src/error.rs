use laks_auth::error::AuthError;

pub type Result<T, E = DatabaseError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("password hashing failed: {0}")]
    PasswordHash(#[from] AuthError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectDatabaseError {
    #[error("Project {0} not found")]
    ProjectNotFound(String),
    #[error("No package files found")]
    NoPackageFilesFound,
    #[error("No packages found")]
    NoPackagesFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

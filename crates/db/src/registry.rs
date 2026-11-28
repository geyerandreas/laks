use crate::error::{ProjectDatabaseError, Result};
use laks_common::normalize_name;
use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(Debug, Serialize, FromRow)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub normalized_name: String,
}

#[derive(Debug, FromRow)]
pub struct PackageFile {
    pub id: i32,
    pub package_id: i32,
    pub filename: String,
    pub version: String,
    pub requires_python: Option<String>,
    pub sha256_hash: String,
    pub file_path: String,
    pub uploaded_at: String,
}

pub async fn get_package(
    pool: &SqlitePool,
    package_name: &str,
) -> Result<Package, ProjectDatabaseError> {
    let normalized_name = normalize_name(package_name);
    sqlx::query_as::<_, Package>("SELECT * FROM Packages WHERE normalized_name = $1")
        .bind(&normalized_name)
        .fetch_one(pool)
        .await
        .map_err(|_| ProjectDatabaseError::ProjectNotFound(normalized_name))
}

pub async fn get_package_files(
    pool: &SqlitePool,
    package_id: i32,
) -> Result<Vec<PackageFile>, ProjectDatabaseError> {
    sqlx::query_as("SELECT * FROM PackageFiles WHERE package_id = $1 ORDER BY filename")
        .bind(package_id)
        .fetch_all(pool)
        .await
        .map_err(|_| ProjectDatabaseError::NoPackageFilesFound)
}

pub async fn get_packages(pool: &SqlitePool) -> Result<Vec<String>, ProjectDatabaseError> {
    sqlx::query_scalar("SELECT name FROM Packages ORDER BY name ASC")
        .fetch_all(pool)
        .await
        .map_err(|_| ProjectDatabaseError::NoPackagesFound)
}

pub async fn add_package(
    pool: &SqlitePool,
    package_name: &str,
) -> Result<Package, ProjectDatabaseError> {
    let result = sqlx::query_as(
        "INSERT INTO Packages (name, normalized_name)
        VALUES ($1, $2)
        ON CONFLICT(normalized_name) DO UPDATE SET name = excluded.name
        RETURNING *",
    )
    .bind(package_name)
    .bind(normalize_name(package_name))
    .fetch_one(pool)
    .await?;
    Ok(result)
}

pub async fn add_package_file(
    pool: &SqlitePool,
    package_id: i32,
    filename: &str,
    version: &str,
    requires_python: Option<&str>,
    sha256_hash: &str,
    file_path: &str,
) -> Result<PackageFile, ProjectDatabaseError> {
    let result = sqlx::query_as(
        "INSERT INTO PackageFiles
            (package_id, filename, version, requires_python, sha256_hash, file_path)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT(package_id, filename) DO UPDATE SET
            version = excluded.version,
            requires_python = excluded.requires_python,
            sha256_hash = excluded.sha256_hash,
            file_path = excluded.file_path
        RETURNING *",
    )
    .bind(package_id)
    .bind(filename)
    .bind(version)
    .bind(requires_python)
    .bind(sha256_hash)
    .bind(file_path)
    .fetch_one(pool)
    .await?;
    Ok(result)
}

use std::str::Utf8Error;
use crate::db::migrations::{MigrationDef, Version};
use crate::db::sqlite::Migrator;
use thiserror::Error;

mod sqlite;
pub mod migrations;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLite db error: {0}")]
    SQLiteError(#[from]
                #[source] sqlx::Error),
    #[error("failed to get migrator file: no file found")]
    MigratorNoFileError,
    #[error("migration malformed: {0}")]
    MigrationMalformedError(String),
    #[error("failed to parse UTF8: {0}")]
    UTF8Error(#[from] #[source] Utf8Error)
}

pub enum Database {
    SQLite(sqlite::Database)
}

pub async fn build_db(
    name: &str,
    up: Vec<MigrationDef>,
    down: Vec<MigrationDef>,
    schema_version: Option<Version>,
) -> Result<Database, DbError> {
    let config = sqlite::DbConfig::read(name);
    match sqlite::build_main_db(config).await {
        Ok(db) => {
            let migrator = Migrator::new(&db, up, down).await?;
            migrator.migrate_up(&db, schema_version).await?;
            Ok(Database::SQLite(db))
        }
        Err(e) => Err(e),
    }
}
use std::fmt::{Display, Formatter};
use crate::db::migrations::{MigrationDef, Version};
use crate::db::sqlite::Migrator;

mod sqlite;
pub mod migrations;

#[derive(Debug)]
pub struct DbError {
    source: String,
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "db error: {}", self.source)
    }
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
        },
        Err(e) => Err(e),
    }
}
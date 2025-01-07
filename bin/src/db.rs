use actor_core::db::migrations::MigrationDef;
use actor_core::db::{build_db, Database, DbError};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "resources/migrations/sqlite/main/"]
struct MigrationDefs;

pub(crate) async fn build_main_db() -> Result<Database, DbError> {
    let mut ups = vec![];
    for file_name in MigrationDefs::iter()
        .filter(|file_name| { file_name.ends_with(".up.sql") }) {
        ups.push(MigrationDef::new(file_name.to_string(), MigrationDefs::get(file_name.as_ref()).unwrap())?);
    }
    let mut downs = vec![];
    for file_name in MigrationDefs::iter()
        .filter(|file_name| { file_name.ends_with(".down.sql") }) {
        downs.push(MigrationDef::new(file_name.to_string(), MigrationDefs::get(file_name.as_ref()).unwrap())?);
    }
    
    build_db(
        "main",
        ups,
        downs,
        None,
    ).await
}

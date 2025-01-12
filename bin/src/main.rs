mod db;
mod modules;
mod http;
mod registry;

use std::env;
use crate::db::build_main_db;
use crate::registry::registry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    println!("Initialising main logger");

    println!("current dir: {}", env::current_dir()?.to_str().unwrap());

    // let default_logger = actor_core::logging::default_logger();
    let main_logger = actor_core::logging::main_logger();

    log::set_logger(main_logger).unwrap();

    log::info!(">>>>>>> logging via log!");

    let db = build_main_db().await.unwrap();
    
    let registry = registry();
    registry.start().await?;

    log::info!("Application stopped");
    Ok(())
}

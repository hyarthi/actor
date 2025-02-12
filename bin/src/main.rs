mod db;
mod modules;
mod http;
mod registry;

use std::env;
use crate::registry::build_registry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("current dir: {}", env::current_dir()?.to_str().unwrap());

    let main_logger = actor_core::logging::main_logger();
    log::set_logger(main_logger).unwrap();
    log::info!("Actor is starting up...");

    let registry = build_registry().await;
    registry.start().await?;

    log::info!("Application stopped");
    Ok(())
}

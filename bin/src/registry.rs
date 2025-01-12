use crate::http;
use actor_core::registry::ServiceRegistry;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref REGISTRY: Arc<ServiceRegistry> = Arc::new(build_registry());
}

pub fn registry() -> &'static ServiceRegistry {
    REGISTRY.as_ref()
}

fn build_registry() -> ServiceRegistry {
    log::debug!("Building registry");
    let mut registry = ServiceRegistry::default();

    log::debug!("Registering HTTP service");
    registry.register_service(http::server());

    log::debug!("Registry build completed");
    registry
}

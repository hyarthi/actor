use std::sync::Arc;
use lazy_static::lazy_static;
use actor_core::http::HttpServer;

lazy_static! {
    static ref HTTP_SERVER: Arc<HttpServer> = Arc::new(HttpServer::default());
}

pub fn server() -> &'static HttpServer {
    HTTP_SERVER.as_ref()
}
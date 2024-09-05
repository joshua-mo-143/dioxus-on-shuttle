use dioxus_fullstack::prelude::*;
use std::path::{Path, PathBuf};
use dioxus::prelude::*;
use dioxus_test_app::app;

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<MyService, shuttle_runtime::Error> {
    Ok(MyService {})
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct MyService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = axum::Router::new()
            // Server side render the application, serve static assets, and register server functions
            .serve_dioxus_application(ServeConfig::default(), app)
            .into_make_service();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();

        // Start your service and bind to the socket address
        Ok(())
    }
}

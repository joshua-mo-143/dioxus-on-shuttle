use axum::routing::get;
use axum::Extension;
use dioxus_fullstack::prelude::*;
use dioxus_test_app::app;
use sqlx::PgPool;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> Result<MyService, shuttle_runtime::Error> {
    Ok(MyService { pool })
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct MyService {
    pool: PgPool,
}

async fn hello_world() -> String {
    "Hello world from Shuttle! 05/12/2024".to_string()
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = axum::Router::new()
            .route("/api/hello", get(hello_world))
            .serve_dioxus_application(ServeConfig::default(), app)
            .into_make_service();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();

        // Start your service and bind to the socket address
        Ok(())
    }
}

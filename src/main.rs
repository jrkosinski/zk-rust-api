use rust_api::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod services;

// Import controller handlers and their macro-generated path constants
use controllers::zk_controller::{get_zk, __get_zk_route};

use crate::services::zk_service::{self, ZKService};

/// Root endpoint handler that returns a welcome message.
#[get("/")]
async fn root() -> &'static str {
    "Welcome to RustAPI!"
}

/// Main entry point for the rust_api REST API server.
/// Demonstrates FastAPI-style routing with decorator macros and dependency injection.
#[tokio::main]
async fn main() {
    initialize_tracing();
    let container = setup_container();
    let app = build_router(&container);

    // Start the server using RustAPI framework
    RustAPI::new(app)
        .port(3000)  // Configurable port (default is 3000)
        .serve()
        .await
        .expect("Failed to start server");
}

/// Initializes the tracing subscriber for logging
fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Sets up the DI container with all services
fn setup_container() -> Container {
    let mut container = Container::new();

    // Register services
    container.register_factory(|| ZKService::new());

    container
}

/// Builds the application router using FastAPI-style route decorators
fn build_router(container: &Container) -> Router {
    // Resolve services from container
    let zk_service = container.resolve::<ZKService>().unwrap();

    let zk_router  = Router::new()
        .route(__get_zk_route, routing::get(get_zk))
        .with_state(zk_service);

    // Merge all routers together
    router::build()
        .route(__root_route, routing::get(root))
        .merge(zk_router)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

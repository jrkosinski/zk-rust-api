use rust_api::prelude::*;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod services;

// Import controller handlers and their macro-generated path constants
use controllers::health_controller::{__health_check_route, health_check};
use controllers::zk_controller::{__get_zk_route, get_zk};
use controllers::merkle_tree_controller::{__add_to_tree_route, add_to_tree};
use crate::services::health_service::HealthService;
use crate::services::zk_service::ZKService;
use crate::services::merkle_tree_service::MerkleTreeService;

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
    container.register_factory(HealthService::new);
    container.register_factory(MerkleTreeService::new);

    container
}

/// Builds the application router using FastAPI-style route decorators
fn build_router(container: &Container) -> Router {
    // Resolve services from container
    let health_service = container.resolve::<HealthService>().unwrap();
    let tree_service = container.resolve::<MerkleTreeService>().unwrap();

    // ZKService depends on MerkleTreeService, so we create it manually
    let zk_service = Arc::new(ZKService::new(tree_service.clone()));

    // Build separate routers for each service with their own state
    let health_router = Router::new()
        .route(__health_check_route, routing::get(health_check))
        .with_state(health_service);

    let zk_router = Router::new()
        .route(__get_zk_route, routing::get(get_zk))
        .with_state(zk_service);

    let tree_router = Router::new()
        .route(__add_to_tree_route, routing::post(add_to_tree))
        .with_state(tree_service);

    // Merge all routers together
    router::build()
        .route(__root_route, routing::get(root))
        .merge(health_router)
        .merge(zk_router)
        .merge(tree_router)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

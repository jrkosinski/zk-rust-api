use rust_api::prelude::*;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod services;

// Import controllers
use crate::controllers::health_controller::HealthController;
use crate::controllers::merkle_tree_controller::MerkleTreeController;
use crate::controllers::zk_controller::ZKController;

// Import services
use crate::services::health_service::HealthService;
use crate::services::merkle_tree_service::MerkleTreeService;
use crate::services::zk_service::ZKService;

/// Root endpoint handler that returns a welcome message.
#[get("/")]
async fn root() -> &'static str {
    "Welcome to ZK RustAPI!"
}

/// Main entry point for the rust_api REST API server.
/// Demonstrates FastAPI-style routing with decorator macros and dependency injection.
#[tokio::main]
async fn main() {
    initialize_tracing();

    // Create services
    let health_service = Arc::new(HealthService::new());
    let tree_service = Arc::new(MerkleTreeService::new());
    let zk_service = Arc::new(ZKService::new(tree_service.clone()));

    // Build the router using RouterPipeline
    let app = RouterPipeline::new()
        .route(__root_route, root)
        .mount::<HealthController>(health_service)
        .mount::<MerkleTreeController>(tree_service)
        .mount::<ZKController>(zk_service)
        .map(|r| r.nest_service("/static", ServeDir::new("static")))
        .map(|r| r.layer(TraceLayer::new_for_http()))
        .map(|r| r.layer(CorsLayer::permissive()))
        .build()
        .expect("Failed to build router");

    // Start the server using RustAPI framework
    RustAPI::new(app)
        .port(3000)
        .serve()
        .await
        .expect("Failed to start server");
}

/// Initializes the tracing subscriber for logging
fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zk_rust_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

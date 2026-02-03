use rust_api::prelude::*;
use std::sync::Arc;
use crate::services::health_service::{HealthService, HealthResponse};

/// Health check endpoint that returns the service status.
/// Uses dependency injection to access the HealthService.
#[get("/health")]
pub async fn health_check(
    State(service): State<Arc<HealthService>>
) -> Json<HealthResponse> {
    let response = service.health_check();
    Json(response)
}

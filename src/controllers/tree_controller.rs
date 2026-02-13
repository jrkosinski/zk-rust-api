use crate::services::tree_service::{TreeResponse, TreeService};
use rust_api::prelude::*;
use std::sync::Arc;

/// Uses dependency injection to access the TreeService.
#[post("/tree")]
pub async fn add_to_tree(State(service): State<Arc<TreeService>>) -> Json<TreeResponse> {
    let response = service.add_to_tree();
    Json(response)
}

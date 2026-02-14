use crate::services::merkle_tree_service::{TreeResponse, MerkleTreeService};
use rust_api::prelude::*;
use std::sync::Arc;

/// Uses dependency injection to access the MerkleTreeService.
#[post("/tree")]
pub async fn add_to_tree(State(service): State<Arc<MerkleTreeService>>) -> Json<TreeResponse> {
    let response = service.add_to_tree();
    Json(response)
}

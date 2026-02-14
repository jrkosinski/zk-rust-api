use crate::services::merkle_tree_service::{TreeResponse, MerkleTreeService};
use rust_api::prelude::*;
use std::sync::Arc;

/// Request body for adding a value to the tree.
#[derive(Debug, Deserialize)]
pub struct AddToTreeRequest {
    /// The value to add as a leaf in the tree
    pub value: u64,
}

/// Adds a new value to the Merkle tree and returns the new root hash.
/// Uses dependency injection to access the MerkleTreeService.
///
/// # Request Body
/// ```json
/// {
///   "value": 90
/// }
/// ```
///
/// # Response
/// Returns a TreeResponse containing the new root hash after rebuilding the tree.
#[post("/tree")]
pub async fn add_to_tree(
    State(service): State<Arc<MerkleTreeService>>,
    Json(request): Json<AddToTreeRequest>,
) -> Json<TreeResponse> {
    let response = service.add_to_tree(request.value);
    Json(response)
}

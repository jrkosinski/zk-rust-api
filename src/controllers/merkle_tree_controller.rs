use crate::services::merkle_tree_service::{
    parse_fp_hex, RegisterRequest, TreeResponse, MerkleTreeService,
};
use rust_api::prelude::*;
use std::sync::Arc;

/// Request body for adding a value to the tree.
#[derive(Debug, Deserialize)]
pub struct AddToTreeRequest {
    /// The value to add as a leaf in the tree
    pub value: u64,
}

/// Registers a new commitment in the Merkle tree.
/// The client computes `commitment = Poseidon(secret)` locally and sends only the commitment.
/// The server never learns the secret.
///
/// # Request Body
/// ```json
/// { "commitment": "<64-char hex Fp>" }
/// ```
///
/// # Response
/// Returns a TreeResponse containing the new root hash, or 400 on invalid commitment.
#[post("/register")]
pub async fn register(
    State(service): State<Arc<MerkleTreeService>>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    match parse_fp_hex(&request.commitment) {
        Some(commitment) => (StatusCode::OK, Json(service.register_commitment(commitment))).into_response(),
        None => (StatusCode::BAD_REQUEST, "invalid commitment: expected 64-char hex (32 bytes)").into_response(),
    }
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

/// Generates a visualization of the current Merkle tree and returns the image URL.
/// Uses dependency injection to access the MerkleTreeService.
///
/// # Response
/// Returns a TreeVisualizationResponse containing the URL to the generated image.
/// The image shows the tree structure with nodes and hash values (truncated).
#[get("/tree/visualize")]
pub async fn visualize_tree(
    State(service): State<Arc<MerkleTreeService>>,
) -> impl IntoResponse {
    match service.visualize_tree() {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

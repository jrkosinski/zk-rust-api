use crate::services::zk_service::{ZKProofResponse, ZKService};
use rust_api::prelude::*;
use std::sync::Arc;

/// Request body for the ZK proof endpoint.
/// The secret is the private value whose Poseidon commitment must be in the tree.
#[derive(Deserialize)]
pub struct ZKProofRequest {
    pub secret: u64,
}

/// Proves knowledge of a secret whose Poseidon commitment is in the Merkle tree.
/// The secret is used as a private ZK witness and is never stored or logged.
///
/// # Request Body
/// ```json
/// { "secret": 42 }
/// ```
///
/// # Response
/// Returns `{ "proof": true }` if Poseidon(secret) is in the tree and the ZK circuit verifies.
#[post("/zk")]
pub async fn post_zk(
    State(service): State<Arc<ZKService>>,
    Json(request): Json<ZKProofRequest>,
) -> Json<ZKProofResponse> {
    Json(service.zk_proof(request.secret))
}

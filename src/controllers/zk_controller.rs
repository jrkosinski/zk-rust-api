use crate::services::zk_service::{ZKProofResponse, ZKService};
use rust_api::prelude::*;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ZKQuery {
    leaf: u64,
}

#[get("/zk")]
pub async fn get_zk(
    State(service): State<Arc<ZKService>>,
    Query(params): Query<ZKQuery>,
) -> Json<ZKProofResponse> {
    let response = service.zk_proof(params.leaf);
    Json(response)
}

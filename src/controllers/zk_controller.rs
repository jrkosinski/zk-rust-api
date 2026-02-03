use rust_api::prelude::*;
use std::sync::Arc;
use crate::services::zk_service::{ZKService, ZKProofResponse};

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

use crate::services::zk_service::{ZKProofResponse, ZKService};
use rust_api::prelude::*;
use std::sync::Arc;

#[get("/zk")]
pub async fn get_zk(State(service): State<Arc<ZKService>>) -> Json<ZKProofResponse> {
    let response = service.zk_proof();
    Json(response)
}

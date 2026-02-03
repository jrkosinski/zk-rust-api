use rust_api::prelude::*;

/// Response type for the health check endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct ZKProofResponse {
    pub proof: bool,
}

pub struct ZKService {}

impl Injectable for ZKService {}

impl ZKService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn zk_proof(&self) -> ZKProofResponse {
        ZKProofResponse { proof: false }
    }
}

use rust_api::prelude::*;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Circuit, ConstraintSystem, Advice, Column, Instance, Selector},
    pasta::Fp,
    poly::Rotation
};
use halo2_proofs::{
    dev::MockProver,
};

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
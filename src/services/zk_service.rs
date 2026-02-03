use rust_api::prelude::*;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Circuit, ConstraintSystem, Advice, Column, Instance, Selector},
    pasta::Fp,
};
use halo2_proofs::{
    dev::MockProver,
};

/// Response type for the health check endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct ZKProofResponse {
    pub proof: bool,
}

#[derive(Clone, Debug)]
struct MerkleCircuit {
    leaf: Value<Fp>,
    path_1: Value<Fp>,
    path_2: Value<Fp>,
}

#[derive(Clone, Debug)]
struct MerkleConfig {
    advice: Column<Advice>,
    pub instance: Column<Instance>,
    pub s: Selector,
}

impl Circuit<Fp> for MerkleCircuit {
    type Config = MerkleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            leaf: Value::unknown(),
            path_1: Value::unknown(),
            path_2: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        let s = meta.selector();

        MerkleConfig { advice, instance, s }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> std::result::Result<(), plonk::Error> {
        //TODO: implement 
        Ok(())
    }
}

pub struct ZKService {}

impl Injectable for ZKService {}

impl ZKService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn zk_proof(&self, leaf_val: u64) -> ZKProofResponse {
        let leaf = Fp::from(leaf_val);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);
        let root = Fp::from(10u64) + s1 + s2;

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            path_1: Value::known(s1),
            path_2: Value::known(s2),
        };

        let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
        
        ZKProofResponse { proof: prover.verify().is_ok() }
    }
}
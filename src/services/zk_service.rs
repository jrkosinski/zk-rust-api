use halo2_proofs::dev::MockProver;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Instance, Selector},
    poly::Rotation,
};
use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, P128Pow5T3},
    Hash, Pow5Chip, Pow5Config,
};
use rust_api::prelude::*;

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
    poseidon: Pow5Config<Fp, 3, 2>,
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

        // Allow equality constraints / copying between cells.
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        //poseidon config - needs state columns for hashing
        let state = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        let partial_sbox = meta.advice_column();

        //enable equality on state columns so we can copy values between regions
        for col in &state {
            meta.enable_equality(*col);
        }

        //round constants need more columns
        let rc_a = [
            meta.fixed_column(),
            meta.fixed_column(),
            meta.fixed_column(),
        ];
        let rc_b = [
            meta.fixed_column(),
            meta.fixed_column(),
            meta.fixed_column(),
        ];

        let poseidon = Pow5Chip::<Fp, 3, 2>::configure::<P128Pow5T3>(
            meta,
            state,
            partial_sbox,
            rc_a,
            rc_b,
        );

        // Gate enforces:
        // h1 = leaf + path1
        // h2 = h1 + path2
        //
        // Layout (single advice column, 5 rows starting at offset):
        // row 0: leaf
        // row 1: path1
        // row 2: h1
        // row 3: path2
        // row 4: h2
        meta.create_gate("merkle add constraints", |meta| {
            let s = meta.query_selector(s);

            let leaf = meta.query_advice(advice, Rotation::cur());
            let path1 = meta.query_advice(advice, Rotation::next());
            let h1 = meta.query_advice(advice, Rotation(2));
            let path2 = meta.query_advice(advice, Rotation(3));
            let h2 = meta.query_advice(advice, Rotation(4));

            vec![
                s.clone() * (h1.clone() - leaf - path1),
                s * (h2 - h1 - path2),
            ]
        });

        MerkleConfig {
            advice,
            instance,
            s,
            poseidon
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> std::result::Result<(), plonk::Error> {
        // Initialize the hasher using the Hash trait
        let hasher = Hash::<_, _, P128Pow5T3, ConstantLength<2>, 3, 2>::init(
            Pow5Chip::<Fp, 3, 2>::construct(config.poseidon.clone()),
            layouter.namespace(|| "init hasher"),
        )?;

        // Assign leaf and path_1 as cells first
        let leaf_cell = layouter.assign_region(
            || "assign leaf",
            |mut region| {
                region.assign_advice(|| "leaf", config.advice, 0, || self.leaf)
            },
        )?;

        let path_1_cell = layouter.assign_region(
            || "assign path_1",
            |mut region| {
                region.assign_advice(|| "path_1", config.advice, 0, || self.path_1)
            },
        )?;

        // Hash leaf + path_1 to get h1
        let h1_cell = hasher.hash(
            layouter.namespace(|| "poseidon h1"),
            [leaf_cell, path_1_cell],
        )?;

        // Assign path_2 as a cell
        let path_2_cell = layouter.assign_region(
            || "assign path_2",
            |mut region| {
                region.assign_advice(|| "path_2", config.advice, 0, || self.path_2)
            },
        )?;

        // Hash h1 + path_2 to get h2
        let hasher2 = Hash::<_, _, P128Pow5T3, ConstantLength<2>, 3, 2>::init(
            Pow5Chip::<Fp, 3, 2>::construct(config.poseidon.clone()),
            layouter.namespace(|| "init hasher2"),
        )?;

        let h2_cell_result = hasher2.hash(
            layouter.namespace(|| "poseidon h2"),
            [h1_cell.clone(), path_2_cell],
        )?;


        let h2_final = layouter.assign_region(
            || "merkle chain (depth=2)",
            |mut region| {
                let offset = 0;

                // Enable the gate at the start row of this 5-row block
                config.s.enable(&mut region, offset)?;

                region.assign_advice(|| "leaf", config.advice, offset, || self.leaf)?;
                region.assign_advice(|| "path1", config.advice, offset + 1, || self.path_1)?;
                region.assign_advice(|| "h1", config.advice, offset + 2, || h1_cell.value().copied())?;
                region.assign_advice(|| "path2", config.advice, offset + 3, || self.path_2)?;
                let h2 = region.assign_advice(|| "h2", config.advice, offset + 4, || h2_cell_result.value().copied())?;

                Ok(h2)
            },
        )?;

        // Constrain h2 == public root (instance[0])
        layouter.constrain_instance(h2_final.cell(), config.instance, 0)?;

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

        ZKProofResponse {
            proof: prover.verify().is_ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zk_proof_with_correct_value() {
        let service = ZKService::new();
        let response = service.zk_proof(10);

        assert!(response.proof, "Expected proof to be true for correct leaf value of 10");
    }

    #[test]
    fn test_zk_proof_with_incorrect_value() {
        let service = ZKService::new();
        let response = service.zk_proof(15);

        assert!(!response.proof, "Expected proof to be false for incorrect leaf value of 15");
    }

    #[test]
    fn test_zk_proof_with_zero() {
        let service = ZKService::new();
        let response = service.zk_proof(0);

        assert!(!response.proof, "Expected proof to be false for incorrect leaf value of 0");
    }

    #[test]
    fn test_zk_proof_with_large_incorrect_value() {
        let service = ZKService::new();
        let response = service.zk_proof(1000);

        assert!(!response.proof, "Expected proof to be false for incorrect leaf value of 1000");
    }
}

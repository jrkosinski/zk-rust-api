use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, Hash as PoseidonHash, P128Pow5T3},
    Hash, Pow5Chip, Pow5Config,
};
use halo2_proofs::dev::MockProver;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Instance},
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

        //allow equality constraints / copying between cells
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

        //round constants need many fixed columns for P128Pow5T3
        //allocate enough columns to store all round constants
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

        //mark these columns as constant columns
        meta.enable_constant(rc_a[0]);
        meta.enable_constant(rc_a[1]);
        meta.enable_constant(rc_a[2]);
        meta.enable_constant(rc_b[0]);
        meta.enable_constant(rc_b[1]);
        meta.enable_constant(rc_b[2]);

        let poseidon =
            Pow5Chip::<Fp, 3, 2>::configure::<P128Pow5T3>(meta, state, partial_sbox, rc_a, rc_b);

        //note: poseidon chip provides its own constraints for the hash computation
        //we don't need additional gates for the merkle tree logic
        //the constraints are: h1 = poseidon(leaf, path1) and h2 = poseidon(h1, path2)

        MerkleConfig {
            advice,
            instance,
            poseidon,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> std::result::Result<(), plonk::Error> {
        //initialize the hasher using the Hash trait
        let hasher = Hash::<_, _, P128Pow5T3, ConstantLength<2>, 3, 2>::init(
            Pow5Chip::<Fp, 3, 2>::construct(config.poseidon.clone()),
            layouter.namespace(|| "init hasher"),
        )?;

        //assign leaf and path_1 as cells first
        let leaf_cell = layouter.assign_region(
            || "assign leaf",
            |mut region| region.assign_advice(|| "leaf", config.advice, 0, || self.leaf),
        )?;

        let path_1_cell = layouter.assign_region(
            || "assign path_1",
            |mut region| region.assign_advice(|| "path_1", config.advice, 0, || self.path_1),
        )?;

        //hash leaf + path_1 to get h1
        let h1_cell =
            hasher.hash(layouter.namespace(|| "poseidon h1"), [leaf_cell, path_1_cell])?;

        //assign path_2 as a cell
        let path_2_cell = layouter.assign_region(
            || "assign path_2",
            |mut region| region.assign_advice(|| "path_2", config.advice, 0, || self.path_2),
        )?;

        //hash h1 + path_2 to get h2
        let hasher2 = Hash::<_, _, P128Pow5T3, ConstantLength<2>, 3, 2>::init(
            Pow5Chip::<Fp, 3, 2>::construct(config.poseidon.clone()),
            layouter.namespace(|| "init hasher2"),
        )?;

        let h2_cell =
            hasher2.hash(layouter.namespace(|| "poseidon h2"), [h1_cell.clone(), path_2_cell])?;

        //constrain h2 == public root (instance[0])
        layouter.constrain_instance(h2_cell.cell(), config.instance, 0)?;

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

        //compute the expected root for the correct leaf value (10)
        //this is the fixed merkle root we're checking against
        let correct_leaf = Fp::from(10u64);
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([correct_leaf, s1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h1, s2]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            path_1: Value::known(s1),
            path_2: Value::known(s2),
        };

        //k=8 gives 2^8=256 rows which is enough for poseidon operations
        //the circuit proves that the provided leaf hashes to expected_root
        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();

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

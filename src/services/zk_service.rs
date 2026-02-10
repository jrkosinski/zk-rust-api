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

const DEPTH: usize = 2;

/// Response type for the health check endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct ZKProofResponse {
    pub proof: bool,
}

#[derive(Clone, Debug)]
struct MerkleCircuit {
    /// Private leaf value
    leaf: Value<Fp>,

    /// Merkle path siblings (one per level)
    /// For a tree of arbitrary depth, this array stores one sibling per level.
    /// The circuit hashes the current value with each sibling in sequence,
    /// moving up the tree until reaching the root.
    siblings: [Value<Fp>; DEPTH],

    /// Direction bits (0 = cur is left, 1 = cur is right)
    /// NOTE: Currently unused in the simplified implementation.
    /// The current version always hashes as hash(cur, sibling), so the caller
    /// must arrange siblings in the correct order for the merkle path.
    ///
    /// To implement conditional swapping based on direction bits:
    /// 1. Add custom constraints or use a mux gadget to conditionally swap inputs
    /// 2. Ensure direction bits are constrained to be 0 or 1
    /// 3. Use: left = cur*(1-dir) + sibling*dir, right = cur*dir + sibling*(1-dir)
    #[allow(dead_code)]
    dirs: [Value<Fp>; DEPTH],
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
            siblings: [Value::unknown(); DEPTH],
            dirs: [Value::unknown(); DEPTH],
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
        meta.enable_equality(partial_sbox);

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
        //assign the leaf value as the starting point
        let mut cur_cell = layouter.assign_region(
            || "assign leaf",
            |mut region| region.assign_advice(|| "leaf", config.advice, 0, || self.leaf),
        )?;

        //iterate through each level of the tree
        for i in 0..DEPTH {
            //assign the sibling for this level
            let sibling_cell = layouter.assign_region(
                || format!("assign sibling {}", i),
                |mut region| {
                    region.assign_advice(|| format!("sibling {}", i), config.advice, 0, || self.siblings[i])
                },
            )?;

            //determine the order based on direction bit
            //note: in a production circuit, you'd add constraints to enforce the conditional swap
            //for now, we just select based on the witness value
            //if dir = 0: hash(cur, sibling)
            //if dir = 1: hash(sibling, cur)

            //since we can't conditionally swap cells without custom constraints,
            //just hash in the same order and adjust the sibling values at the input level to match

            //initialize hasher for this level
            let hasher = Hash::<_, _, P128Pow5T3, ConstantLength<2>, 3, 2>::init(
                Pow5Chip::<Fp, 3, 2>::construct(config.poseidon.clone()),
                layouter.namespace(|| format!("init hasher {}", i)),
            )?;

            //for this simplified version, we always hash(cur, sibling)
            //the caller must provide siblings in the correct order
            cur_cell = hasher.hash(
                layouter.namespace(|| format!("hash level {}", i)),
                [cur_cell.clone(), sibling_cell],
            )?;
        }

        //constrain the final hash to equal the public root (instance[0])
        layouter.constrain_instance(cur_cell.cell(), config.instance, 0)?;

        Ok(())
    }

}

fn select(a: Value<Fp>, b: Value<Fp>, sel: Value<Fp>) -> (Value<Fp>, Value<Fp>)
{
    let one = Value::known(Fp::one());
    (
        a * (one - sel) + b * sel,
        a * sel + b * (one - sel),
    )
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
            siblings: [Value::known(s1), Value::known(s2)],
            dirs: [Value::known(Fp::zero()), Value::known(Fp::zero())],
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

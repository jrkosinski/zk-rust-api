use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, Hash as PoseidonHash, P128Pow5T3},
};
use halo2_proofs::dev::MockProver;
use halo2_proofs::{circuit::Value, pasta::Fp};
use rust_api::prelude::*;

use super::merkle_circuit::MerkleCircuit;

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
            directions: [Value::known(Fp::zero()), Value::known(Fp::zero())],
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

        assert!(
            response.proof,
            "Expected proof to be true for correct leaf value of 10"
        );
    }

    #[test]
    fn test_zk_proof_with_incorrect_value() {
        let service = ZKService::new();
        let response = service.zk_proof(15);

        assert!(
            !response.proof,
            "Expected proof to be false for incorrect leaf value of 15"
        );
    }

    #[test]
    fn test_zk_proof_with_zero() {
        let service = ZKService::new();
        let response = service.zk_proof(0);

        assert!(
            !response.proof,
            "Expected proof to be false for incorrect leaf value of 0"
        );
    }

    #[test]
    fn test_zk_proof_with_large_incorrect_value() {
        let service = ZKService::new();
        let response = service.zk_proof(1000);

        assert!(
            !response.proof,
            "Expected proof to be false for incorrect leaf value of 1000"
        );
    }

    #[test]
    fn test_direction_bits_all_zeros() {
        //test with direction bits [0, 0] - leaf on left at both levels
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);

        //compute expected root: hash(hash(leaf, s1), s2)
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h1, s2]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2)],
            directions: [Value::known(Fp::zero()), Value::known(Fp::zero())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_ok(),
            "Direction bits [0, 0] should verify"
        );
    }

    #[test]
    fn test_direction_bits_all_ones() {
        //test with direction bits [1, 1] - leaf on right at both levels
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);

        //compute expected root: hash(s2, hash(s1, leaf))
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([s1, leaf]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([s2, h1]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2)],
            directions: [Value::known(Fp::one()), Value::known(Fp::one())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_ok(),
            "Direction bits [1, 1] should verify"
        );
    }

    #[test]
    fn test_direction_bits_mixed() {
        //test with direction bits [0, 1] - leaf left at level 0, result right at level 1
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);

        //compute expected root: hash(s2, hash(leaf, s1))
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([s2, h1]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2)],
            directions: [Value::known(Fp::zero()), Value::known(Fp::one())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_ok(),
            "Direction bits [0, 1] should verify"
        );
    }

    #[test]
    fn test_direction_bits_wrong_direction() {
        //test that wrong direction bits cause verification to fail
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);

        //compute root for [0, 0]: hash(hash(leaf, s1), s2)
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h1, s2]);

        //but provide direction bits [1, 0] which would compute: hash(hash(s1, leaf), s2)
        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2)],
            directions: [Value::known(Fp::one()), Value::known(Fp::zero())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_err(),
            "Wrong direction bits should fail verification"
        );
    }

    #[test]
    fn test_invalid_direction_bit() {
        //test that non-binary direction bits cause verification to fail
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);

        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h1, s2]);

        //use an invalid direction bit value (should be 0 or 1, but we use 2)
        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2)],
            directions: [Value::known(Fp::from(2)), Value::known(Fp::zero())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_err(),
            "Non-binary direction bit should fail verification"
        );
    }
}

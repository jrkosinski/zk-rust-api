use halo2_proofs::dev::MockProver;
use halo2_proofs::{circuit::Value, pasta::Fp};
use rust_api::prelude::*;
use std::sync::Arc;

use super::merkle_circuit::{MerkleCircuit, DEPTH};
use super::merkle_tree_service::MerkleTreeService;

/// Response type for zero-knowledge proof verification.
/// Contains a boolean indicating whether the proof is valid.
#[derive(Debug, Serialize, Deserialize)]
pub struct ZKProofResponse {
    pub proof: bool,
}

/// Service for generating and verifying zero-knowledge proofs using Merkle trees.
/// Uses MerkleTreeService to access the shared default Merkle tree.
pub struct ZKService {
    tree_service: Arc<MerkleTreeService>,
}

impl Injectable for ZKService {}

impl ZKService {
    /// Creates a new ZKService with a reference to the MerkleTreeService.
    /// The tree is accessed from MerkleTreeService, which maintains the shared default tree.
    pub fn new(tree_service: Arc<MerkleTreeService>) -> Self {
        Self { tree_service }
    }

    /// Generates a zero-knowledge proof that a given leaf value exists in the Merkle tree.
    /// Returns a ZKProofResponse indicating whether the proof is valid.
    ///
    /// # Arguments
    /// * `leaf_val` - The leaf value to prove membership for
    ///
    /// # Returns
    /// A ZKProofResponse with proof=true if the leaf exists in the tree and verification succeeds,
    /// or proof=false if the leaf doesn't exist or verification fails.
    pub fn zk_proof(&self, leaf_val: u64) -> ZKProofResponse {
        //access the tree from the tree service
        self.tree_service.with_tree(|tree| {
            //try to find the leaf in the tree
            let leaf_index = tree
                .leaves()
                .iter()
                .position(|&l| l == Fp::from(leaf_val));

            //if the leaf is not in the tree, the proof will fail
            let (leaf, siblings, directions, expected_root) = if let Some(idx) = leaf_index {
                //generate proof for the found leaf
                let proof = tree.generate_proof(idx).unwrap();

                //convert siblings and directions to arrays for the circuit
                //the proof returns Vecs, but the circuit needs fixed-size arrays matching DEPTH
                let siblings_array: [Fp; DEPTH] = proof.siblings
                    .try_into()
                    .expect("Tree depth doesn't match circuit DEPTH constant");
                let directions_array: [Fp; DEPTH] = proof.directions
                    .try_into()
                    .expect("Tree depth doesn't match circuit DEPTH constant");

                (
                    proof.leaf,
                    siblings_array,
                    directions_array,
                    proof.root,
                )
            } else {
                //leaf not in tree - use dummy values that will fail verification
                let leaf = Fp::from(leaf_val);
                (
                    leaf,
                    [Fp::zero(); DEPTH],
                    [Fp::zero(); DEPTH],
                    tree.root(),
                )
            };

            let circuit = MerkleCircuit {
                leaf: Value::known(leaf),
                siblings: siblings.map(|s| Value::known(s)),
                directions: directions.map(|d| Value::known(d)),
            };

            //k=8 gives 2^8=256 rows which is enough for poseidon operations
            //the circuit proves that the provided leaf hashes to expected_root
            let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();

            ZKProofResponse {
                proof: prover.verify().is_ok(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_gadgets::poseidon::{
        primitives::{ConstantLength, Hash as PoseidonHash, P128Pow5T3},
    };

    #[test]
    fn test_zk_proof_with_correct_value() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        let response = service.zk_proof(10);

        assert!(
            response.proof,
            "Expected proof to be true for correct leaf value of 10"
        );
    }

    #[test]
    fn test_zk_proof_with_incorrect_value() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        let response = service.zk_proof(15);

        assert!(
            !response.proof,
            "Expected proof to be false for incorrect leaf value of 15"
        );
    }

    #[test]
    fn test_zk_proof_with_zero() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        let response = service.zk_proof(0);

        assert!(
            !response.proof,
            "Expected proof to be false for incorrect leaf value of 0"
        );
    }

    #[test]
    fn test_zk_proof_with_large_incorrect_value() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        let response = service.zk_proof(1000);

        assert!(
            !response.proof,
            "Expected proof to be false for incorrect leaf value of 1000"
        );
    }

    #[test]
    fn test_zk_proof_with_last_level_value() {
        //test with a value from the last level of the tree (depth 3, 8 leaves)
        //the tree has leaves [10, 20, 30, 40, 50, 60, 70, 80]
        //testing with 70, which is at index 6 (second to last leaf)
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        let response = service.zk_proof(70);

        assert!(
            response.proof,
            "Expected proof to be true for leaf value 70 at index 6 on last level"
        );
    }

    #[test]
    fn test_zk_proof_with_last_leaf() {
        //test with the very last leaf in the tree (index 7)
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        let response = service.zk_proof(80);

        assert!(
            response.proof,
            "Expected proof to be true for last leaf value 80 at index 7"
        );
    }

    #[test]
    fn test_direction_bits_all_zeros() {
        //test with direction bits [0, 0, 0] - leaf on left at all three levels
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);
        let s3 = Fp::from(40);

        //compute expected root: hash(hash(hash(leaf, s1), s2), s3)
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let h2 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([h1, s2]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h2, s3]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2), Value::known(s3)],
            directions: [Value::known(Fp::zero()), Value::known(Fp::zero()), Value::known(Fp::zero())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_ok(),
            "Direction bits [0, 0, 0] should verify"
        );
    }

    #[test]
    fn test_direction_bits_all_ones() {
        //test with direction bits [1, 1, 1] - leaf on right at all three levels
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);
        let s3 = Fp::from(40);

        //compute expected root: hash(s3, hash(s2, hash(s1, leaf)))
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([s1, leaf]);
        let h2 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([s2, h1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([s3, h2]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2), Value::known(s3)],
            directions: [Value::known(Fp::one()), Value::known(Fp::one()), Value::known(Fp::one())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_ok(),
            "Direction bits [1, 1, 1] should verify"
        );
    }

    #[test]
    fn test_direction_bits_mixed() {
        //test with direction bits [0, 1, 0] - mixed directions across three levels
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);
        let s3 = Fp::from(40);

        //compute expected root: hash(hash(s2, hash(leaf, s1)), s3)
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let h2 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([s2, h1]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h2, s3]);

        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2), Value::known(s3)],
            directions: [Value::known(Fp::zero()), Value::known(Fp::one()), Value::known(Fp::zero())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_ok(),
            "Direction bits [0, 1, 0] should verify"
        );
    }

    #[test]
    fn test_direction_bits_wrong_direction() {
        //test that wrong direction bits cause verification to fail
        let leaf = Fp::from(10u64);
        let s1 = Fp::from(20);
        let s2 = Fp::from(30);
        let s3 = Fp::from(40);

        //compute root for [0, 0, 0]: hash(hash(hash(leaf, s1), s2), s3)
        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let h2 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([h1, s2]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h2, s3]);

        //but provide direction bits [1, 0, 0] which would compute: hash(hash(hash(s1, leaf), s2), s3)
        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2), Value::known(s3)],
            directions: [Value::known(Fp::one()), Value::known(Fp::zero()), Value::known(Fp::zero())],
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
        let s3 = Fp::from(40);

        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, s1]);
        let h2 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([h1, s2]);
        let expected_root =
            PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash([h2, s3]);

        //use an invalid direction bit value (should be 0 or 1, but we use 2)
        let circuit = MerkleCircuit {
            leaf: Value::known(leaf),
            siblings: [Value::known(s1), Value::known(s2), Value::known(s3)],
            directions: [Value::known(Fp::from(2)), Value::known(Fp::zero()), Value::known(Fp::zero())],
        };

        let prover = MockProver::run(8, &circuit, vec![vec![expected_root]]).unwrap();
        assert!(
            prover.verify().is_err(),
            "Non-binary direction bit should fail verification"
        );
    }
}

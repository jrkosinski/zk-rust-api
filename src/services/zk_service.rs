use halo2_proofs::dev::MockProver;
use halo2_proofs::{circuit::Value, pasta::Fp};
use rust_api::prelude::*;
use std::sync::Arc;

use super::merkle_circuit::{MerkleCircuit, DEPTH};
use super::merkle_tree_service::{poseidon_commit, MerkleTreeService};

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

    /// Generates a zero-knowledge proof that the caller knows a secret whose Poseidon commitment
    /// is in the Merkle tree, without revealing which commitment or the secret itself.
    ///
    /// # Arguments
    /// * `secret` - The private secret value known by the prover
    ///
    /// # Returns
    /// ZKProofResponse with proof=true if Poseidon(secret) is in the tree and verification passes.
    pub fn zk_proof(&self, secret: u64) -> ZKProofResponse {
        self.tree_service.with_tree(|tree| {
            let commitment = poseidon_commit(secret);
            let circuit = self.build_circuit(secret, commitment, tree);
            let root = tree.root();

            //k=9 gives 2^9=512 rows; needed for two Poseidon hashes (commitment + path levels)
            let prover = MockProver::run(9, &circuit, vec![vec![root]]).unwrap();
            ZKProofResponse { proof: prover.verify().is_ok() }
        })
    }

    /// Builds the MerkleCircuit for the given secret, looking up its commitment in the tree.
    /// Returns a circuit with dummy witnesses if the commitment is not found.
    fn build_circuit(&self, secret: u64, commitment: Fp, tree: &super::merkle_tree::MerkleTree) -> MerkleCircuit {
        let leaf_index = tree.leaves().iter().position(|&l| l == commitment);

        let (siblings, directions) = if let Some(idx) = leaf_index {
            self.extract_proof_arrays(tree, idx)
        } else {
            //commitment not in tree — dummy witnesses will cause verification to fail
            ([Fp::zero(); DEPTH], [Fp::zero(); DEPTH])
        };

        MerkleCircuit {
            secret: Value::known(Fp::from(secret)),
            siblings: siblings.map(Value::known),
            directions: directions.map(Value::known),
        }
    }

    /// Extracts fixed-size sibling and direction arrays from a Merkle proof at a given index.
    fn extract_proof_arrays(&self, tree: &super::merkle_tree::MerkleTree, idx: usize) -> ([Fp; DEPTH], [Fp; DEPTH]) {
        let proof = tree.generate_proof(idx).expect("valid index");
        let siblings: [Fp; DEPTH] = proof.siblings.try_into().expect("depth mismatch");
        let directions: [Fp; DEPTH] = proof.directions.try_into().expect("depth mismatch");
        (siblings, directions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::merkle_tree_service::poseidon_commit;

    //seed secrets used in MerkleTreeService::new(): [42, 99, 7, 13, 55, 77, 100, 200]

    #[test]
    fn test_zk_proof_with_valid_secret() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        //secret 42 is a seed secret — its commitment is in the tree
        assert!(service.zk_proof(42).proof, "proof should succeed for seed secret 42");
    }

    #[test]
    fn test_zk_proof_with_another_valid_secret() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        //secret 99 is also a seed secret
        assert!(service.zk_proof(99).proof, "proof should succeed for seed secret 99");
    }

    #[test]
    fn test_zk_proof_with_invalid_secret() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        //secret 1 is not a seed secret
        assert!(!service.zk_proof(1).proof, "proof should fail for unknown secret 1");
    }

    #[test]
    fn test_zk_proof_with_zero_secret() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        assert!(!service.zk_proof(0).proof, "proof should fail for secret 0");
    }

    #[test]
    fn test_zk_proof_last_seed_secret() {
        let tree_service = Arc::new(MerkleTreeService::new());
        let service = ZKService::new(tree_service);
        //secret 200 is the last seed secret (index 7)
        assert!(service.zk_proof(200).proof, "proof should succeed for seed secret 200");
    }

    #[test]
    fn test_zk_proof_after_register() {
        //verify that a newly registered commitment can be proved
        let tree_service = Arc::new(MerkleTreeService::new());
        let commitment = poseidon_commit(999);
        tree_service.register_commitment(commitment);

        let service = ZKService::new(tree_service);
        assert!(service.zk_proof(999).proof, "proof should succeed for freshly registered secret 999");
    }
}

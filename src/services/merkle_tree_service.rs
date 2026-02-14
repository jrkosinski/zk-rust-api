use rust_api::prelude::*;
use crate::services::merkle_tree::MerkleTree;
use std::sync::Mutex;

/// Response type for the tree endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreeResponse {
    pub data: String,
}

pub struct MerkleTreeService {
    tree: Mutex<MerkleTree>,
}

impl Injectable for MerkleTreeService {}

impl MerkleTreeService {
    /// Creates a new MerkleTreeService with a default Merkle tree.
    /// The tree is initialized with example leaves [10, 20, 30, 40, 50, 60, 70, 80].
    /// This is the same default tree previously used in ZKService.
    pub fn new() -> Self {
        let tree = MerkleTree::new(vec![10u64, 20, 30, 40, 50, 60, 70, 80]);
        Self {
            tree: Mutex::new(tree),
        }
    }

    /// Adds a new leaf value to the Merkle tree and rebuilds it.
    /// Returns the new root hash after the tree is rebuilt.
    ///
    /// # Arguments
    /// * `value` - The u64 value to add as a leaf
    ///
    /// # Returns
    /// TreeResponse containing the new root hash as a hex string
    pub fn add_to_tree(&self, value: u64) -> TreeResponse {
        self.with_tree_mut(|tree| {
            tree.add(value);
            let root = tree.root();
            TreeResponse {
                data: format!("{:?}", root),
            }
        })
    }

    /// Returns a read-only reference to the MerkleTree.
    /// Note: This requires locking the mutex. Use carefully to avoid deadlocks.
    pub fn with_tree<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&MerkleTree) -> R,
    {
        let tree = self.tree.lock().unwrap();
        f(&tree)
    }

    /// Provides mutable access to the tree through a closure.
    pub fn with_tree_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut MerkleTree) -> R,
    {
        let mut tree = self.tree.lock().unwrap();
        f(&mut tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::pasta::Fp;

    #[test]
    fn test_add_to_tree_and_verify() {
        //create a new service with the default tree [10, 20, 30, 40, 50, 60, 70, 80]
        let service = MerkleTreeService::new();

        //verify initial state - value 90 should NOT be in the tree
        let initial_contains_90 = service.with_tree(|tree| {
            tree.leaves().iter().any(|&leaf| leaf == Fp::from(90u64))
        });
        assert!(!initial_contains_90, "Value 90 should not be in tree initially");

        //add value 90 to the tree
        let response = service.add_to_tree(90);

        //verify the response is not empty
        assert!(!response.data.is_empty(), "Response should contain root hash");

        //verify the value was actually added to the tree
        let now_contains_90 = service.with_tree(|tree| {
            tree.leaves().iter().any(|&leaf| leaf == Fp::from(90u64))
        });
        assert!(now_contains_90, "Value 90 should be in the tree after adding");

        //verify we can find the value at a specific position (should be at the end before padding)
        let leaf_index = service.with_tree(|tree| {
            tree.leaves().iter().position(|&leaf| leaf == Fp::from(90u64))
        });
        assert!(leaf_index.is_some(), "Should be able to find index of value 90");

        //verify we can generate a proof for the newly added value
        let proof_result = service.with_tree(|tree| {
            tree.generate_proof(leaf_index.unwrap())
        });
        assert!(proof_result.is_some(), "Should be able to generate proof for value 90");
    }

    #[test]
    fn test_add_many_to_tree_and_verify() {
        //create a new service with the default tree [10, 20, 30, 40, 50, 60, 70, 80]
        let service = MerkleTreeService::new();

        //verify initial state - value 90 should NOT be in the tree
        let initial_contains_90 = service.with_tree(|tree| {
            tree.leaves().iter().any(|&leaf| leaf == Fp::from(90u64))
        });
        assert!(!initial_contains_90, "Value 90 should not be in tree initially");

        //add several values to the tree
        let response = service.add_to_tree(410);
        let response = service.add_to_tree(190);
        let response = service.add_to_tree(90);
        let response = service.add_to_tree(290);
        let response = service.add_to_tree(240);

        //verify the response is not empty
        assert!(!response.data.is_empty(), "Response should contain root hash");

        //verify the value was actually added to the tree
        let now_contains_90 = service.with_tree(|tree| {
            tree.leaves().iter().any(|&leaf| leaf == Fp::from(90u64))
        });
        assert!(now_contains_90, "Value 90 should be in the tree after adding");

        //verify we can find the value at a specific position (should be at the end before padding)
        let leaf_index = service.with_tree(|tree| {
            tree.leaves().iter().position(|&leaf| leaf == Fp::from(90u64))
        });
        assert!(leaf_index.is_some(), "Should be able to find index of value 90");

        //verify we can generate a proof for the newly added value
        let proof_result = service.with_tree(|tree| {
            tree.generate_proof(leaf_index.unwrap())
        });
        assert!(proof_result.is_some(), "Should be able to generate proof for value 90");
    }

    #[test]
    fn test_add_multiple_values() {
        let service = MerkleTreeService::new();

        //add multiple values
        service.add_to_tree(90);
        service.add_to_tree(100);
        service.add_to_tree(110);

        //verify all values are present
        service.with_tree(|tree| {
            let leaves_contain_90 = tree.leaves().iter().any(|&l| l == Fp::from(90u64));
            let leaves_contain_100 = tree.leaves().iter().any(|&l| l == Fp::from(100u64));
            let leaves_contain_110 = tree.leaves().iter().any(|&l| l == Fp::from(110u64));

            assert!(leaves_contain_90, "Tree should contain 90");
            assert!(leaves_contain_100, "Tree should contain 100");
            assert!(leaves_contain_110, "Tree should contain 110");
        });
    }

    #[test]
    fn test_root_changes_after_add() {
        let service = MerkleTreeService::new();

        //get initial root
        let initial_root = service.with_tree(|tree| tree.root());

        //add a value
        service.add_to_tree(90);

        //get new root
        let new_root = service.with_tree(|tree| tree.root());

        //verify root changed
        assert_ne!(initial_root, new_root, "Root should change after adding a value");
    }
}

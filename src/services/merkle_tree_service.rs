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

    pub fn add_to_tree(&self) -> TreeResponse {
        TreeResponse {
            data: "ok".to_string(),
        }
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

use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, Hash as PoseidonHash, P128Pow5T3},
};
use halo2_proofs::pasta::Fp;

/// Represents a leaf value in the Merkle tree.
/// Can be either an unhashed value (which will be hashed) or a pre-hashed Fp value.
#[derive(Clone, Debug)]
pub enum LeafValue {
    /// Unhashed value that will be hashed using Poseidon
    Unhashed(u64),
    /// Pre-hashed Fp value that will be used as-is
    Hashed(Fp),
}

impl From<u64> for LeafValue {
    fn from(val: u64) -> Self {
        LeafValue::Unhashed(val)
    }
}

impl From<Fp> for LeafValue {
    fn from(val: Fp) -> Self {
        LeafValue::Hashed(val)
    }
}

/// Represents a Merkle proof for a specific leaf.
/// Contains the sibling nodes and direction bits needed to reconstruct the path to the root.
#[derive(Clone, Debug)]
pub struct MerkleProof {
    /// The leaf value being proven
    pub leaf: Fp,
    /// Sibling nodes along the path from leaf to root
    pub siblings: Vec<Fp>,
    /// Direction bits: 0 = leaf/current is left, 1 = leaf/current is right
    pub directions: Vec<Fp>,
    /// The root hash
    pub root: Fp,
}

/// A Merkle tree implementation using Poseidon hash.
/// Supports arbitrary depth (automatically calculated from number of leaves).
/// Pads with zeros if the number of leaves is not a power of 2.
#[derive(Clone, Debug)]
pub struct MerkleTree {
    /// All leaves at the bottom level (may include zero-padding)
    leaves: Vec<Fp>,
    /// All nodes in the tree, organized by levels (0 = leaves, last = root)
    /// Each level contains the hashes at that level
    levels: Vec<Vec<Fp>>,
    /// The depth of the tree (number of levels from leaf to root, not including leaf level)
    depth: usize,
}

impl MerkleTree {
    /// Creates a new Merkle tree from a list of leaf values.
    /// Leaf values can be either unhashed (u64) or pre-hashed (Fp).
    /// If the number of leaves is not a power of 2, pads with zeros.
    ///
    /// # Arguments
    /// * `leaves` - Vector of leaf values (can be mixed unhashed and hashed)
    ///
    /// # Example
    /// ```
    /// let tree = MerkleTree::new(vec![10.into(), 20.into(), 30.into()]);
    /// ```
    pub fn new<T: Into<LeafValue>>(leaves: Vec<T>) -> Self {
        let mut converted_leaves: Vec<Fp> = leaves
            .into_iter()
            .map(|leaf| match leaf.into() {
                LeafValue::Unhashed(val) => Fp::from(val),
                LeafValue::Hashed(fp) => fp,
            })
            .collect();

        // Pad with zeros if not a power of 2
        let padded_size = converted_leaves.len().next_power_of_two();
        converted_leaves.resize(padded_size, Fp::zero());

        let depth = (padded_size as f64).log2() as usize;

        let mut tree = MerkleTree {
            leaves: converted_leaves,
            levels: Vec::new(),
            depth,
        };

        tree.build();
        tree
    }

    /// Adds a new leaf to the tree and rebuilds it.
    /// The leaf can be either unhashed (u64) or pre-hashed (Fp).
    ///
    /// # Arguments
    /// * `leaf` - The leaf value to add
    ///
    /// # Example
    /// ```
    /// tree.add(40u64);
    /// ```
    pub fn add<T: Into<LeafValue>>(&mut self, leaf: T) {
        let fp_leaf = match leaf.into() {
            LeafValue::Unhashed(val) => Fp::from(val),
            LeafValue::Hashed(fp) => fp,
        };

        // Remove trailing zeros from padding
        while self.leaves.last() == Some(&Fp::zero()) && self.leaves.len() > 1 {
            self.leaves.pop();
        }

        // Add the new leaf
        self.leaves.push(fp_leaf);

        // Pad to next power of 2
        let padded_size = self.leaves.len().next_power_of_two();
        self.leaves.resize(padded_size, Fp::zero());

        // Recalculate depth and rebuild
        self.depth = (padded_size as f64).log2() as usize;
        self.build();
    }

    /// Builds the tree by computing all internal nodes from leaves to root.
    /// Uses Poseidon hash to combine pairs of nodes at each level.
    fn build(&mut self) {
        self.levels.clear();

        // Level 0 is the leaves
        self.levels.push(self.leaves.clone());

        // Build each level up to the root
        let mut current_level = self.leaves.clone();

        for _ in 0..self.depth {
            let mut next_level = Vec::new();

            // Hash pairs of nodes to create the next level
            for chunk in current_level.chunks(2) {
                let left = chunk[0];
                let right = chunk[1];

                let hash = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
                    .hash([left, right]);

                next_level.push(hash);
            }

            self.levels.push(next_level.clone());
            current_level = next_level;
        }
    }

    /// Returns the root hash of the tree.
    pub fn root(&self) -> Fp {
        self.levels.last().unwrap()[0]
    }

    /// Generates a Merkle proof for the leaf at the given index.
    /// The proof contains the sibling nodes and direction bits needed to verify
    /// that the leaf is part of the tree.
    ///
    /// # Arguments
    /// * `leaf_index` - The index of the leaf (0-based, before padding)
    ///
    /// # Returns
    /// * `Some(MerkleProof)` if the index is valid
    /// * `None` if the index is out of bounds
    ///
    /// # Example
    /// ```
    /// let proof = tree.generate_proof(0).unwrap();
    /// ```
    pub fn generate_proof(&self, leaf_index: usize) -> Option<MerkleProof> {
        if leaf_index >= self.leaves.len() {
            return None;
        }

        let mut siblings = Vec::new();
        let mut directions = Vec::new();
        let mut current_index = leaf_index;

        // Traverse from leaf to root, collecting siblings and directions
        for level in 0..self.depth {
            // Determine if current node is left (0) or right (1)
            let is_right = current_index % 2 == 1;
            let direction = if is_right { Fp::one() } else { Fp::zero() };
            directions.push(direction);

            // Get the sibling index
            let sibling_index = if is_right {
                current_index - 1
            } else {
                current_index + 1
            };

            // Get the sibling value
            let sibling = self.levels[level][sibling_index];
            siblings.push(sibling);

            // Move to parent index for next level
            current_index /= 2;
        }

        Some(MerkleProof {
            leaf: self.leaves[leaf_index],
            siblings,
            directions,
            root: self.root(),
        })
    }

    /// Returns the depth of the tree (number of levels from leaf to root, not including leaf level).
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the number of leaves in the tree (including zero-padding).
    pub fn num_leaves(&self) -> usize {
        self.leaves.len()
    }

    /// Returns a reference to the leaves (including zero-padding).
    pub fn leaves(&self) -> &[Fp] {
        &self.leaves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_creation_power_of_2() {
        // Create a tree with 4 leaves (power of 2)
        let tree = MerkleTree::new(vec![10u64, 20, 30, 40]);

        assert_eq!(tree.num_leaves(), 4);
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    fn test_tree_creation_non_power_of_2() {
        // Create a tree with 3 leaves (not power of 2, should pad to 4)
        let tree = MerkleTree::new(vec![10u64, 20, 30]);

        assert_eq!(tree.num_leaves(), 4); // Padded to 4
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.leaves()[3], Fp::zero()); // Last leaf should be zero
    }

    #[test]
    fn test_add_leaf() {
        let mut tree = MerkleTree::new(vec![10u64, 20]);
        assert_eq!(tree.num_leaves(), 2);

        tree.add(30u64);
        assert_eq!(tree.num_leaves(), 4); // Should pad to 4
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    fn test_generate_proof() {
        let tree = MerkleTree::new(vec![10u64, 20, 30, 40]);

        let proof = tree.generate_proof(0).unwrap();

        assert_eq!(proof.leaf, Fp::from(10u64));
        assert_eq!(proof.siblings.len(), 2); // Depth is 2
        assert_eq!(proof.directions.len(), 2);
        assert_eq!(proof.root, tree.root());
    }

    #[test]
    fn test_generate_proof_invalid_index() {
        let tree = MerkleTree::new(vec![10u64, 20]);

        let proof = tree.generate_proof(10);
        assert!(proof.is_none());
    }

    #[test]
    fn test_mixed_leaf_types() {
        // Test using both unhashed and pre-hashed values
        let hashed_val = Fp::from(15u64);
        let tree = MerkleTree::new(vec![
            LeafValue::Unhashed(10),
            LeafValue::Hashed(hashed_val),
        ]);

        assert_eq!(tree.num_leaves(), 2);
        assert_eq!(tree.leaves()[0], Fp::from(10u64));
        assert_eq!(tree.leaves()[1], hashed_val);
    }

    #[test]
    fn test_root_changes_after_add() {
        let mut tree = MerkleTree::new(vec![10u64, 20]);
        let root1 = tree.root();

        tree.add(30u64);
        let root2 = tree.root();

        // Root should change after adding a leaf
        assert_ne!(root1, root2);
    }

    #[test]
    fn test_proof_verification() {
        // Create a simple tree and verify the proof manually
        let tree = MerkleTree::new(vec![10u64, 20, 30, 40]);
        let proof = tree.generate_proof(0).unwrap();

        // Manually compute the path: hash(hash(10, 20), hash(30, 40))
        let leaf = Fp::from(10u64);
        let sibling1 = Fp::from(20u64);

        let h1 = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([leaf, sibling1]);

        // The sibling at level 1 should be hash(30, 40)
        let h2_sibling = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([Fp::from(30u64), Fp::from(40u64)]);

        let root = PoseidonHash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init()
            .hash([h1, h2_sibling]);

        assert_eq!(proof.root, root);
        assert_eq!(proof.root, tree.root());
    }
}

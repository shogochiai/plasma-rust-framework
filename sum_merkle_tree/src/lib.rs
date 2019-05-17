extern crate crypto;

use self::crypto::sha3::Sha3;
use byteorder::{LittleEndian, WriteBytesExt};
use crypto::digest::Digest;

#[derive(Debug)]
pub enum Error {
    VerifyError,
}

fn hash_leaf(value: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3::keccak256();
    let mut result = vec![0u8; hasher.output_bits() / 8];
    hasher.reset();
    hasher.input(value);
    hasher.result(result.as_mut_slice());
    result
}

fn empty_hash() -> Vec<u8> {
    hash_leaf(&[0u8]).to_vec()
}

trait Hashable {
    fn hash(&self) -> Vec<u8>;
}

/// SumMerkleNode is a node in merkle tree
///
///```text
///  full tree
///
///           root
///        /        \
///      Node       Node
///     /   \      /   \
///   Leaf  Leaf Leaf  Leaf
///
///  branch and proof
///
///           root
///        /        \
///      Node     ProofNode
///     /   \      
///   Leaf  Leaf
///```
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SumMerkleNode {
    Leaf {
        end: u64,
        data: Vec<u8>,
    },

    Node {
        end: u64,
        left: Box<SumMerkleNode>,
        right: Box<SumMerkleNode>,
    },

    ProofNode {
        end: u64,
        data: Vec<u8>,
    },
}

/// Caluculate hash of a node
fn compute_node(end: u64, data: &[u8]) -> Vec<u8> {
    let mut end_writer = vec![];
    end_writer.write_u64::<LittleEndian>(end).unwrap();
    end_writer.append(&mut Vec::from(data));
    hash_leaf(&end_writer).to_vec()
}

impl Hashable for SumMerkleNode {
    fn hash(&self) -> Vec<u8> {
        match self {
            SumMerkleNode::Leaf { data, .. } => hash_leaf(data),
            // H(H(left.end + left.data) + H(right.end + right.data))
            SumMerkleNode::Node { left, right, .. } => {
                let mut buf = compute_node(left.get_end(), &left.hash());
                buf.append(&mut compute_node(right.get_end(), &right.hash()));
                hash_leaf(&buf)
            }
            SumMerkleNode::ProofNode { data, .. } => data.clone(),
        }
    }
}

impl SumMerkleNode {
    pub fn create_proof_node(node: &SumMerkleNode) -> SumMerkleNode {
        SumMerkleNode::ProofNode {
            end: node.get_end(),
            data: node.hash().to_vec(),
        }
    }

    pub fn create_empty() -> Self {
        SumMerkleNode::Leaf {
            end: u64::max_value(),
            data: empty_hash(),
        }
    }

    pub fn create_empty_leaf(end: u64) -> Self {
        SumMerkleNode::Leaf {
            end,
            data: empty_hash(),
        }
    }

    pub fn create_leaf(end: u64, data: &[u8]) -> Self {
        SumMerkleNode::Leaf {
            end,
            data: data.to_vec(),
        }
    }

    pub fn create_node(end: u64, left: &SumMerkleNode, right: &SumMerkleNode) -> Self {
        SumMerkleNode::Node {
            end,
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
        }
    }

    pub fn compute_parent(left: &SumMerkleNode, right: &SumMerkleNode) -> SumMerkleNode {
        SumMerkleNode::create_node(right.get_end(), left, right)
    }

    fn get_end(&self) -> u64 {
        match self {
            SumMerkleNode::Leaf { end, .. } => *end,
            SumMerkleNode::Node { end, .. } => *end,
            SumMerkleNode::ProofNode { end, .. } => *end,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImplicitBounds {
    implicit_start: u64,
    implicit_end: u64,
}

impl ImplicitBounds {
    pub fn new(implicit_start: u64, implicit_end: u64) -> Self {
        ImplicitBounds {
            implicit_start,
            implicit_end,
        }
    }
}

#[derive(Debug)]
pub struct SumMerkleTree {
    tree: SumMerkleNode,
}

impl SumMerkleTree {
    /// generate sum merkle tree
    pub fn generate(leaves: &[SumMerkleNode]) -> SumMerkleTree {
        if leaves.len() <= 1 {
            return SumMerkleTree {
                tree: leaves[0].clone(),
            };
        }
        let mut parents = vec![];
        for chunk in leaves.chunks(2) {
            let v = chunk.to_vec();
            if chunk.len() == 1 {
                parents.push(SumMerkleNode::compute_parent(
                    &v[0],
                    &SumMerkleNode::create_empty(),
                ))
            } else {
                parents.push(SumMerkleNode::compute_parent(&v[0].clone(), &v[1].clone()))
            }
        }
        SumMerkleTree::generate(&parents)
    }

    /// Calculate merkle root
    pub fn get_root(&self) -> Vec<u8> {
        self.tree.hash()
    }

    /// Returns inclusion proof for a leaf
    pub fn get_inclusion_proof(&self, idx: usize, count: usize) -> Vec<SumMerkleNode> {
        SumMerkleTree::get_inclusion_proof_of_tree(&self.tree, idx, count)
    }

    fn get_inclusion_proof_of_tree(
        tree: &SumMerkleNode,
        idx: usize,
        count: usize,
    ) -> Vec<SumMerkleNode> {
        match tree {
            SumMerkleNode::Leaf { .. } => vec![],
            SumMerkleNode::Node { left, right, .. } => {
                let left_count = count.next_power_of_two() / 2;
                if idx < left_count {
                    let mut proofs = Self::get_inclusion_proof_of_tree(left, idx, left_count);
                    proofs.push(SumMerkleNode::create_proof_node(&right));
                    proofs
                } else {
                    let mut proofs = Self::get_inclusion_proof_of_tree(
                        right,
                        idx - left_count,
                        count - left_count,
                    );
                    proofs.push(SumMerkleNode::create_proof_node(&left));
                    proofs
                }
            }
            SumMerkleNode::ProofNode { .. } => vec![],
        }
    }

    /// get_path
    /// get_path converts index of leaf to binary.
    /// ex) 1 -> 0b0001 -(revert)> [true, false, false, false]
    /// It means right, left, left, left
    ///
    /// Another example.
    /// 3 -> 0b11 -(revert)> [true, true]
    /// It means right, right
    ///
    ///```text
    ///        root
    ///       /    \
    ///     /  \  /  \
    ///     0  1  2  3
    /// ```
    ///
    fn get_path(idx: usize, depth: usize, path: &mut Vec<bool>) {
        if depth == 0 {
            return;
        }
        path.push((idx & 0x01) != 0);
        Self::get_path(idx.rotate_right(1), depth - 1, path)
    }

    fn verify_and_get_parent(
        left: &SumMerkleNode,
        right: &SumMerkleNode,
        _first_left_end: u64,
    ) -> Result<SumMerkleNode, Error> {
        /*
        if left.get_end() > first_left_end {
            return Err(Error::VerifyError);
        }
        */
        if left.get_end() > right.get_end() {
            return Err(Error::VerifyError);
        }
        Ok(SumMerkleNode::compute_parent(left, right))
    }

    /// Verify whether leaf is included or not
    pub fn verify(
        leaf: &SumMerkleNode,
        idx: usize,
        inclusion_proof: Vec<SumMerkleNode>,
        root: &[u8],
    ) -> Result<ImplicitBounds, Error> {
        let mut path: Vec<bool> = vec![];
        Self::get_path(idx, inclusion_proof.len(), path.as_mut());
        println!("{:?}, {:?}", path, inclusion_proof);
        let first_left_end = path
            .iter()
            .position(|&p| p)
            .map(|pos| inclusion_proof[pos].clone())
            .map_or(0, |n| n.get_end());
        let mut computed = leaf.clone();
        for (i, item) in inclusion_proof.iter().enumerate() {
            if path[i] {
                // leaf is in right
                computed = Self::verify_and_get_parent(item, &computed, first_left_end)?
            } else {
                // leaf is in left
                computed = Self::verify_and_get_parent(&computed, item, first_left_end)?
            }
        }
        let is_last_leaf = 2u64.pow(inclusion_proof.len() as u32) - 1 == (idx as u64);
        if computed.hash() == root {
            Ok(ImplicitBounds::new(
                first_left_end,
                if is_last_leaf {
                    u64::max_value()
                } else {
                    leaf.get_end()
                },
            ))
        } else {
            Err(Error::VerifyError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SumMerkleNode;
    use super::SumMerkleTree;

    #[test]
    fn test_compute_parent() {
        let hash_message1 = Vec::from(&b"message"[..]);
        let leaf1 = SumMerkleNode::Leaf {
            end: 100,
            data: hash_message1,
        };
        let hash_message2 = Vec::from(&b"message"[..]);
        let leaf2 = SumMerkleNode::Leaf {
            end: 200,
            data: hash_message2,
        };
        let parent = SumMerkleNode::compute_parent(&leaf1, &leaf2);
        assert_eq!(parent.get_end(), 200);
    }

    #[test]
    fn test_generate_tree() {
        let hash_message1 = Vec::from(&b"message"[..]);
        let leaf1 = SumMerkleNode::Leaf {
            end: 100,
            data: hash_message1,
        };
        let hash_message2 = Vec::from(&b"message"[..]);
        let leaf2 = SumMerkleNode::Leaf {
            end: 200,
            data: hash_message2,
        };
        let tree = SumMerkleTree::generate(&[leaf1, leaf2]);
        assert_eq!(tree.get_root().len(), 32);
    }

    #[test]
    fn test_proof() {
        let hash_message1 = Vec::from(&b"message"[..]);
        let leaf1 = SumMerkleNode::Leaf {
            end: 100,
            data: hash_message1,
        };
        let hash_message2 = Vec::from(&b"message"[..]);
        let leaf2 = SumMerkleNode::Leaf {
            end: 200,
            data: hash_message2,
        };
        let tree = SumMerkleTree::generate(&[leaf1.clone(), leaf2]);
        let inclusion_proof = tree.get_inclusion_proof(0, 2);
        assert_eq!(inclusion_proof.len(), 1);
        assert_eq!(
            SumMerkleTree::verify(&leaf1.clone(), 0, inclusion_proof, &tree.get_root()).is_ok(),
            true
        );
    }

    #[test]
    fn test_large_leaves() {
        let mut leaves = vec![];
        for i in 0..100 {
            leaves.push(SumMerkleNode::Leaf {
                end: i * 100 + 100,
                data: Vec::from(&b"message"[..]),
            })
        }
        let tree = SumMerkleTree::generate(&leaves);
        let inclusion_proof = tree.get_inclusion_proof(5, 100);
        assert_eq!(inclusion_proof.len(), 7);
        assert_eq!(
            SumMerkleTree::verify(&leaves[5].clone(), 5, inclusion_proof, &tree.get_root()).is_ok(),
            true
        );
    }

    #[test]
    fn test_failed_to_verify() {
        let mut leaves = vec![];
        for i in 0..100 {
            leaves.push(SumMerkleNode::Leaf {
                end: i * 100 + 100,
                data: Vec::from(&b"message"[..]),
            })
        }
        let tree = SumMerkleTree::generate(&leaves);
        let inclusion_proof = tree.get_inclusion_proof(5, 100);
        assert_eq!(inclusion_proof.len(), 7);
        assert_eq!(
            SumMerkleTree::verify(&leaves[5].clone(), 7, inclusion_proof, &tree.get_root()).is_ok(),
            false
        );
    }

}

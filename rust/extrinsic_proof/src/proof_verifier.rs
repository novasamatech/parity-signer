use crate::types::{MetadataProof, Hash, Type};
use array_bytes::Hex;
use codec::Encode;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
enum MetadataDigest {
	Disabled,
	V1 {
		types_tree_root: Hash,
		extrinsic_metadata_hash: Hash,
		spec_version: u32,
		spec_name: String,
		base58_prefix: u16,
		decimals: u8,
		token_symbol: String,
	},
}

impl MetadataDigest {
	/// Returns the hash of this digest.
	pub fn hash(&self) -> Hash {
		blake3::hash(&self.encode()).into()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NodeIndex(usize);

impl NodeIndex {
	/// Returns if this is the root node index.
	fn is_root(self) -> bool {
		self.0 == 0
	}

	/// Returns the index of the parent.
	fn parent(self) -> Self {
		if self.is_root() {
			Self(0)
		} else {
			Self((self.0 - 1) / 2)
		}
	}

	/// Returns `true` if this this is a left child?
	fn is_left_child(self) -> bool {
		self.0 % 2 == 1
	}

	/// Returns the level of this index.
	fn level(self) -> usize {
		(self.0 + 1).ilog2() as _
	}

	//// Return the index of the right child.
	fn right_child(self) -> Self {
		Self(self.0 * 2 + 2)
	}

	//// Return the index of the left child.
	fn left_child(self) -> Self {
		Self(self.0 * 2 + 1)
	}

	/// Returns `true` if `other` is a descendent.
	fn is_descendent(self, other: Self) -> bool {
		// If the index is `0`, it is the root
		if self.is_root() {
			return true;
		}

		// If the index is greater, it can not be a descendent
		if self.0 > other.0 {
			return false;
		}

		let level0 = self.level();
		let level1 = other.level();

		// Check if applying X times the parent function leads to
		// the expected `index`. X is the level difference
		self.0 + 1 == (other.0 + 1) >> (level1 - level0)
	}
}

fn get_hash(
  leaf_indices: &mut &[u32],
  leaves: &mut &[Type],
  nodes: &mut &[Hash],
  node_index: NodeIndex
) -> Hash {
  let is_descendent = if leaf_indices.is_empty() {
    false
  } else {
    let current_leaf = NodeIndex(leaf_indices[0] as usize);

    if node_index == current_leaf {
      let hash = blake3::hash(&leaves[0].encode());

      *leaves = &leaves[1..];
      *leaf_indices = &leaf_indices[1..];
      return hash.into();
    }

    node_index.is_descendent(current_leaf)
  };

  if !is_descendent {
    let res = nodes[0];
    *nodes = &nodes[1..];
    return res;
  }

  let left_child = node_index.left_child();
  let left = get_hash(leaf_indices, leaves, nodes, left_child);

  let right_child = node_index.right_child();
  let right = get_hash(leaf_indices, leaves, nodes, right_child);

  blake3::hash(&(left, right).encode()).into()
}

pub fn verify_metadata_proof(metadata_proof: &MetadataProof, expected_hash: Hash) -> Result<(), String> {
  let proof_hash = get_hash(
    &mut &metadata_proof.proof.leaf_indices[..], 
    &mut &metadata_proof.proof.leaves[..], 
    &mut &metadata_proof.proof.nodes[..],
    NodeIndex(0)
  );

  let extrinsic_metadata_hash = metadata_proof.extrinsic.hash();

  let metadata_hash = MetadataDigest::V1 { 
    types_tree_root: proof_hash, 
    extrinsic_metadata_hash: extrinsic_metadata_hash,
    spec_version: metadata_proof.extra_info.spec_version,
    spec_name: metadata_proof.extra_info.spec_name.clone(),
    base58_prefix: metadata_proof.extra_info.base58_prefix,
    decimals: metadata_proof.extra_info.decimals,
    token_symbol: metadata_proof.extra_info.token_symbol.clone() 
  }.hash();

  println!("Calculated hash: {:?}", metadata_hash.hex(""));
  println!("Expected hash: {:?}", expected_hash.hex(""));

  if metadata_hash == expected_hash {
    Ok(())
  } else {
    Err("Different metadata hashes".to_string())
  }
}
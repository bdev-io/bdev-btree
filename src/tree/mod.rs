//! # Tree Data Structure
//!
//! B-Tree Data Structure is a generic data structure.
use super::BTreeGeneralTypeTrait;
use node::BNode;

#[derive(Clone)]
pub struct BTree<K,V> where K: BTreeGeneralTypeTrait + Ord + Clone, V: BTreeGeneralTypeTrait + Clone {
  root: BNode<K, V>,
  // DOC : === B-Tree Properties ===

    // DOC: AUTO_CALCULATED AREA
    header_size: usize,
    node_size: usize,
    is_initialized: bool,
    // DOC : =======================

  // TYPE : B-Tree DEGREE ( B-TREE 차수)
  btree_degree: usize,

  root_path: String,
  // TYPE : B-Tree KEY SIZE ( B-TREE KEY SIZE )
  sizeof_key: usize,
  // TYPE : B-Tree DATA SIZE ( B-TREE DATA SIZE )
  sizeof_data: usize,


  // TYPE : B-Tree Key & Value Type ( B-TREE KEY & VALUE TYPE )
  key_type: std::marker::PhantomData<K>,
  value_type: std::marker::PhantomData<V>,
  // DOC : === B-Tree Properties ===
}

pub mod func;
pub(crate) mod node;


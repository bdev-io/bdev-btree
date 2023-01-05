//! # Tree Data Structure
//!
//! B-Tree Data Structure is a generic data structure.
use super::BTreeGeneralTypeTrait;

pub struct BTree<K,V> where K: BTreeGeneralTypeTrait + Ord, V: BTreeGeneralTypeTrait {
  _k: std::marker::PhantomData<K>,
  _d: std::marker::PhantomData<V>,

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

impl<K,V> BTree<K, V> where K: BTreeGeneralTypeTrait + Ord, V: BTreeGeneralTypeTrait
{
  pub fn new(btree_degree: usize, sizeof_key: usize, sizeof_data: usize) -> Self {
    let mut header_size = 0;
    let mut node_size = 0;
    let mut is_initialized: bool = false;

    // TODO : 1. 헤더 사이즈 계산
    // TODO : 2. 노드 사이즈 계산
    // TODO : 3. 초기화 여부 계산 ()


    Self {
      _k: std::marker::PhantomData,
      _d: std::marker::PhantomData,

        // DOC: AUTO_CALCULATED AREA
        header_size,
        node_size,
        is_initialized,
        // DOC : =======================

      root_path: String::new(),

      btree_degree,
      sizeof_key,
      sizeof_data,

      key_type: std::marker::PhantomData::<K>,
      value_type: std::marker::PhantomData::<V>,
    }
  }
}

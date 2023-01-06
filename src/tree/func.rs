use super::{ BTree, BTreeGeneralTypeTrait};
use crate::constant::DISK_BLOCK_SIZE;

impl<K,V> BTree<K, V> where K: BTreeGeneralTypeTrait + Ord, V: BTreeGeneralTypeTrait
{
  pub fn new(btree_degree: usize, sizeof_key: usize, sizeof_data: usize) -> Self {
    // let header_size = {
    //   // DOC : B-Tree Header Size ( B-Tree 헤더 사이즈 )
    // };
    let header_size = DISK_BLOCK_SIZE;
    let node_size = 0;
    let is_initialized: bool = false;

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


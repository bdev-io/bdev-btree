use std::marker::PhantomData;
use super::{ BTreeGeneralTypeTrait, BNode };


impl<K, V> BNode<K, V> where K: BTreeGeneralTypeTrait + Ord, V: BTreeGeneralTypeTrait {
  pub fn new(degree: usize) -> Self {
    let keys = {
      let mut keys = Vec::with_capacity(degree);
      for _ in 0..degree {
        keys.push(K::default());
      }
      keys
    };
    let data_offsets = vec![u64::default(); degree];
    let child_offsets = vec![u64::default(); degree + 1];
    // HACK : KEYS & DATA_OFFSETS & CHILD_OFFSETS will be filled with default values.
    Self {
      is_leaf: true,
      key_count: 0,
      keys: keys.into_boxed_slice(),
      data_offsets: data_offsets.into_boxed_slice(),
      child_offsets: child_offsets.into_boxed_slice(),
      offset: None,
      _key: PhantomData,
      _value: PhantomData,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_bnode_new_degree3() {
    let bnode = BNode::<u64, u64>::new(3);
    assert!(bnode.is_leaf);
    assert!(bnode.keys.len() == 3);
    println!("{:?}", bnode.keys);
  }

  #[tokio::test]
  async fn test_bnode_new_degree5() {
    let bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.is_leaf);
    assert!(bnode.keys.len() == 5);
    println!("{:?}", bnode.keys);
  }
}

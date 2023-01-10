use std::marker::PhantomData;
use super::{ BTreeGeneralTypeTrait, BNode };
use std::io::Result;

impl<K, V> BNode<K, V> where K: BTreeGeneralTypeTrait + Ord + Clone, V: BTreeGeneralTypeTrait + Clone {
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

  pub fn node_print(&self) {
    println!("\n[DATA_OFFSETS: {:?}]\n", self.data_offsets);
    println!("- IS_LEAF: {}", self.is_leaf);
    println!("- KEY_COUNT: {}", self.key_count);
    println!("- KEYS : ");
    for i in 0..self.key_count {
      println!("- KEYS[{}]: {:?}", i, self.keys[i]);
    }
    println!("- CHILD_OFFSETS: {:?}", self.child_offsets);
    println!("- OFFSET: {:?}", self.offset);
  }


  // DOC : 여기서는 각 NODE 들에서 시작되는 method 들에 대해 기록한다.

  // TODO : 1. insert() 에서 호출되는 method 들을 작성한다.
  // TODO : 2. delete() 에서 호출되는 method 들을 작성한다. -- pass
  // TODO : 3. search() 에서 호출되는 method 들을 작성한다.
  // TODO : 4. update() 에서 호출되는 method 들을 작성한다.
  // TODO : 5. traverse() 에서 호출되는 method 들을 작성한다.
  // TODO : 6. node_print() 에서 호출되는 method 들을 작성한다.

  // TODO : search_best_leaf ( leaf node to insert )
  // TODO : search_key_position ( key position to insert )
  // TODO : split_tree ( split tree )


  pub async fn search_best_leaf(&mut self, key: K) -> Result<Self> {
    // let mut ex_stack: Vec<(BNode<K, V>, usize)> = vec![]; // TYPE : 순환 탐색을 위한 스택, (Node, index)
    // let mut cur_node = self.clone();
    // ex_stack.push((cur_node, 0));
    if self.is_leaf {
      return Ok(self.clone());
    }
    todo!("Not Implemented Yet");
  }

  pub async fn store_data(&mut self, data: V) -> Result<u64> {
    // TODO : Store Data SomeWhere.
    // todo!("Not Implemented Yet");
    Ok(0)
  }

  pub async fn insert_key_data(&mut self, key: K, data: V) -> Result<()> {
    let mut found_idx: i64 = -1;
    for idx in 0..self.key_count {
      if key < self.keys[idx] {
        found_idx = idx as i64;
        break;
      }
    }
    if found_idx > -1 {
      let found_idx: usize = found_idx as usize;
      for ridx in found_idx..self.key_count {
        self.keys[ridx + 1] = self.keys[ridx].clone();
        self.data_offsets[ridx + 1] = self.data_offsets[ridx];
      }
      self.keys[found_idx] = key;
      self.data_offsets[found_idx] = self.store_data(data).await?;
    } else {
      self.keys[self.key_count] = key;
      self.data_offsets[self.key_count] = self.store_data(data).await?;
    }

    self.key_count += 1;

    if self.key_count >= self.keys.len() {
      todo!("SPLIT 해야해~");
    }
    Ok(())
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
    bnode.node_print();
  }

  #[tokio::test]
  async fn test_bnode_new_degree5() {
    let bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.is_leaf);
    assert!(bnode.keys.len() == 5);
    bnode.node_print();
  }

  #[tokio::test]
  async fn test_bnode_search_best_leaf() {
    let mut bnode = BNode::<i64, i64>::new(3);
    assert!(bnode.is_leaf);
    assert!(bnode.keys.len() == 3);
    let bnode_search_result = bnode.search_best_leaf(3).await.unwrap();
    assert!(bnode_search_result.is_leaf == bnode.is_leaf);
    bnode.node_print();
  }

  #[tokio::test]
  async fn test_bnode_insert_key_data() {
    let mut bnode = BNode::<i64, i64>::new(3);
    assert!(bnode.insert_key_data(3, 3).await.is_ok());
    bnode.node_print();

    assert!(bnode.insert_key_data(3, 3).await.is_ok());
    bnode.node_print();

    assert!(bnode.insert_key_data(3, 3).await.is_ok());
    bnode.node_print();

    assert!(bnode.insert_key_data(3, 3).await.is_ok());
    bnode.node_print();
  }
}

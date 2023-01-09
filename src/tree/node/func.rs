use std::clone;
use std::marker::PhantomData;
use std::pin::Pin;
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


  // DOC : 여기서는 각 NODE 들에서 시작되는 method 들에 대해 기록한다.

  // TODO : 1. insert() 에서 호출되는 method 들을 작성한다.
  // TODO : 2. delete() 에서 호출되는 method 들을 작성한다. -- pass
  // TODO : 3. search() 에서 호출되는 method 들을 작성한다.
  // TODO : 4. update() 에서 호출되는 method 들을 작성한다.
  // TODO : 5. traverse() 에서 호출되는 method 들을 작성한다.
  // TODO : 6. print() 에서 호출되는 method 들을 작성한다.


  pub fn is_full(&self) -> bool {
    self.key_count == self.keys.len()
  }

  pub fn is_half_full(&self) -> bool {
    self.key_count == self.keys.len() / 2
  }


  pub fn is_leaf(&self) -> bool {
    self.is_leaf
  }

  pub fn is_not_leaf(&self) -> bool {
    !self.is_leaf
  }

  pub fn set_leaf(&mut self) {
    self.is_leaf = true;
  }

  pub fn set_not_leaf(&mut self) {
    self.is_leaf = false;
  }

  pub async fn insert(&mut self, key: K, value: V) -> Result<()> {
    Ok(())
  }

  pub async fn delete(&mut self, key: K, value: V) -> Result<()> {
    Ok(())
  }

  pub async fn search_leaf(&mut self, key: K) -> Result<()> {
    let mut ex_stack: Vec<(BNode<K, V>, usize)> = vec![]; // TYPE : 순환 탐색을 위한 스택, (Node, index)
    let mut cur_node = self.clone();
    ex_stack.push((cur_node, 0));



    Ok(())
  }

  pub async fn search(&mut self, key: K) -> Result<()> {
    Ok(())
  }

  pub async fn update(&mut self, key: K, value: V) -> Result<()> {
    Ok(())
  }

  pub async fn traverse(&mut self, key: K, value: V) -> Result<()> {
    Ok(())
  }

  pub async fn print(&mut self, key: K, value: V) -> Result<()> {
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
    println!("{:?}", bnode.keys);
  }

  #[tokio::test]
  async fn test_bnode_new_degree5() {
    let bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.is_leaf);
    assert!(bnode.keys.len() == 5);
    println!("{:?}", bnode.keys);
  }

  #[tokio::test]
  async fn test_bnode_insert() {
    let mut bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.insert(1, 1).await.is_ok());
  }

  #[tokio::test]
  async fn test_bnode_delete() {
    let mut bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.delete(1, 1).await.is_ok());
  }

  #[tokio::test]
  async fn test_bnode_search() {
    let mut bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.search(1).await.is_ok());
  }

  #[tokio::test]
  async fn test_bnode_update() {
    let mut bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.update(1, 1).await.is_ok());
  }

  #[tokio::test]
  async fn test_bnode_traverse() {
    let mut bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.traverse(1, 1).await.is_ok());
  }

  #[tokio::test]
  async fn test_bnode_print() {
    let mut bnode = BNode::<u64, u64>::new(5);
    assert!(bnode.print(1, 1).await.is_ok());
  }
}

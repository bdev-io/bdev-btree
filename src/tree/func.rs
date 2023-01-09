use super::{ BTree, node::BNode, BTreeGeneralTypeTrait};
use crate::constant::DISK_BLOCK_SIZE;

impl<K,V> BTree<K, V> where K: BTreeGeneralTypeTrait + Ord + Clone, V: BTreeGeneralTypeTrait + Clone
{
  pub fn new(btree_degree: usize, sizeof_key: usize, sizeof_data: usize) -> Self {
    // let header_size = {
    //   // DOC : B-Tree Header Size ( B-Tree 헤더 사이즈 )
    // };
    let header_size = DISK_BLOCK_SIZE;
    let node_size = 0;
    let is_initialized: bool = false;
    let root: BNode<K, V> = BNode::<K, V>::new(btree_degree);

    // TODO : 1. 헤더 사이즈 계산
    // TODO : 2. 노드 사이즈 계산
    // TODO : 3. 초기화 여부 계산 ()


    Self {
      root,

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

  // DOC : 여기서는 ROOT NODE 로 부터 시작되는 method 들에 대해 기록한다.
  // TODO : 1. insert() 에서 호출되는 method 들을 작성한다.
  // TODO : 2. delete() 에서 호출되는 method 들을 작성한다. -- pass
  // TODO : 3. search() 에서 호출되는 method 들을 작성한다.
  // TODO : 4. update() 에서 호출되는 method 들을 작성한다.
  // TODO : 5. traverse() 에서 호출되는 method 들을 작성한다.
  // TODO : 6. print() 에서 호출되는 method 들을 작성한다.

  pub async fn insert(&mut self, key: K, data: V) {
    println!("BTree::insert() is called.");
    // TODO : 1. leaf node to insert
    
    // TODO : 2. find best position (key index) to insert
    // TODO : 3. insert key & data
    // TODO : 4. check have to split?
    // TODO : 5. split!
  }

  pub async fn delete(&mut self, key: K) {
    println!("BTree::delete() is called.");
  }

  pub async fn search(&mut self, key: K) {
    println!("BTree::search() is called.");
  }

  pub async fn update(&mut self, key: K, data: V) {
    println!("BTree::update() is called.");
  }

  pub async fn traverse(&mut self) {
    println!("BTree::traverse() is called.");
  }

  pub async fn print(&mut self) {
    println!("BTree::print() is called.");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_btree_new() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.insert(1, 1).await;
  }

  #[tokio::test]
  async fn test_btree_insert() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.insert(1, 1).await;
  }

  #[tokio::test]
  async fn test_btree_delete() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.delete(1).await;
  }

  #[tokio::test]
  async fn test_btree_search() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.search(1).await;
  }

  #[tokio::test]
  async fn test_btree_update() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.update(1, 1).await;
  }

  #[tokio::test]
  async fn test_btree_traverse() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.traverse().await;
  }

  #[tokio::test]
  async fn test_btree_print() {
    let mut btree = BTree::<u64, u64>::new(3, 8, 8);
    btree.print().await;
  }

}


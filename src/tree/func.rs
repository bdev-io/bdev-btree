use super::{ BTree, node::BNode, BTreeGeneralTypeTrait};
use crate::constant::DISK_BLOCK_SIZE;
use std::io::Result;
use std::path::Path;

impl<K,V> BTree<K, V> where K: BTreeGeneralTypeTrait + Ord + Clone, V: BTreeGeneralTypeTrait + Clone
{
  pub fn new(tree_name: &str, btree_degree: usize, sizeof_key: usize, sizeof_data: usize) -> Self {
    let header_size = DISK_BLOCK_SIZE;
    let node_size = 0;
    let is_initialized: bool = false;
    let mut root: BNode<K, V> = BNode::<K, V>::new(btree_degree);
    root.parent_offset = Option::Some(0);

    let root_path = Path::new("stores").join({
      if cfg!(debug_assertions) {
        if cfg!(test) {
          "test"
        } else {
          "debug"
        }
      } else {
        "release"
      }
    }).join(tree_name).to_str().unwrap().to_string();

    Self {
      root,

        // DOC: AUTO_CALCULATED AREA
        header_size,
        node_size,
        is_initialized,
        // DOC : =======================

      root_path,

      btree_degree,
      sizeof_key,
      sizeof_data,

      key_type: std::marker::PhantomData::<K>,
      value_type: std::marker::PhantomData::<V>,
    }
  }

  // DOC : 여기서는 ROOT NODE 로 부터 시작되는 method 들에 대해 기록한다.

  pub async fn insert(&mut self, key: K, data: V) -> Result<()> {
    let x = self.root.search_best_leaf(key).await?;
    println!("{:?}", x.offset);
    Ok(())
  }

  pub async fn print_onetime(&mut self) -> Result<()> {
    println!("HEADER_SIZE: {}, NODE_SIZE: {}", self.header_size, self.node_size);
    println!("INITALIZED: {}", self.is_initialized);
    println!("ROOT_PATH: {:?}", self.root_path);
    println!("B-TREE DEGREE: {}", self.btree_degree);
    println!("SIZE OF [KEY : {}, DATA: {}]", self.sizeof_key, self.sizeof_data);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_btree_new() {
    let btree = BTree::<u64, u64>::new("TEST_TREE", 3, 8, 8);
    assert!(!btree.is_initialized);
  }

  #[tokio::test]
  async fn test_btree_insert() {
    let mut btree = BTree::<u64, u64>::new("TEST_TREE2",3, 8, 8);
    btree.insert(1, 1).await.unwrap();
    btree.print_onetime().await.unwrap();
    assert!(btree.insert(1, 1).await.is_ok());
  }

  #[tokio::test]
  async fn test_btree_print() {
    let mut btree = BTree::<usize, u64>::new("TEST_TREE3", 3, 8, 8);
    btree.insert(1, 1).await.unwrap();
    assert!(btree.print_onetime().await.is_ok());
  }

}


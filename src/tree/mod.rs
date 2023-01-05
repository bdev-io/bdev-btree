//! # Tree Data Structure
//!
//! B-Tree Data Structure is a generic data structure.
use super::BTreeGeneralTypeTrait;
pub struct BTree<K: BTreeGeneralTypeTrait + Ord, T: BTreeGeneralTypeTrait> {
  _k: std::marker::PhantomData<K>,
  _d: std::marker::PhantomData<T>,
  root_path: String,
  degree: usize,
  data_size: usize,
  key_size: usize,
}

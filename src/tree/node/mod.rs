use std::marker::PhantomData;
use super::BTreeGeneralTypeTrait;

#[derive(Clone)]
pub struct BNode<K, V> where K: BTreeGeneralTypeTrait + Ord + Clone, V: BTreeGeneralTypeTrait + Clone {
  pub(crate) is_leaf: bool,
  pub(crate) key_count: usize,
  pub(crate) keys: Box<[K]>, // NOTE : DEGREE
  pub(crate) data_offsets: Box<[u64]>, // NOTE : DEGREE
  pub(crate) child_offsets: Box<[u64]>, // NOTE : DEGREE + 1

  pub(crate) offset: Option<u64>, // TYPE : 해당 노드의 offset
  pub(crate) _key: PhantomData<K>,
  pub(crate) _value: PhantomData<V>,
}


pub mod func;

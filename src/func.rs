use crate::traits::external::BTreeGeneralTypeTrait;
use super::tree::BTree;
// NOTE : Hackable Degree

/// ## Functions for B-Tree
/// 
/// ### `init::<Type: Sized + ()`
/// 
/// Initialize the B-Tree
///
/// * `degree` : The degree of the B-Tree ( usize )
/// * `return` : None
///
/// # Example
///
/// ```
/// use btree::init;
/// let tree_name = "test";
/// init::<i32, u64>("i32tree", 3);
/// // OR 
/// init::<usize, i64>("tree2", 3);
/// // OR 
/// init::<u64, u64>("u64tree", 3);
/// // OR 
/// init::<usize, u64>("tree_test", 3);
/// ```
///
/// ## More Examples
/// // TODO : Add More Examples
///
pub fn init<K: BTreeGeneralTypeTrait + Ord, V: BTreeGeneralTypeTrait>(tree_name: &str, degree: usize) -> BTree<K, V> {
  let key = K::default();
  let data = V::default();

  let sizeof_key: usize = key.get_byte_size();
  let sizeof_data: usize = data.get_byte_size();
  if sizeof_key == 0 || sizeof_data == 0 {
    panic!("Data OR Key Size is 0");
  }

  // let is_initialized = GLOBAL_DEGREE_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst);
  // if is_initialized {
  //   panic!("B-Tree is already initialized.\nB-Tree가 이미 초기화 된 상태입니다.");
  // } else if degree < 2 || degree % 2 == 0 {
  //   panic!("Degree must be greater or equal than 2 and odd Number\n차수는 2 이상이여야 하며 홀수여야 합니다.");
  // }

  BTree::<K,V>::new(tree_name, degree, sizeof_key, sizeof_data)
}

// INFO : TEST

#[cfg(test)]
mod tests {
  use super::*;

  /// DOC: MUST ONLY ONE TEST FOR init
  #[ignore]
  #[tokio::test]
  async fn test_just_one_time() {
    // DOC : This Init Test will execute latest
    init::<usize, usize>("test_just_one_time", 3);
  }
}


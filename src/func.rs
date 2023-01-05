use crate::traits::external::BTreeGeneralTypeTrait;
use super::global::{ GLOBAL_DEGREE, GLOBAL_DEGREE_INITIALIZED, GLOBAL_DATA_SIZE  };
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
/// btree<u64>::init(3);
/// // OR ~
/// btree<u32>::init(3);
/// // OR ~
/// btree<usize>::init(3);
/// // OR ~
/// btree<i32>::init(3);
/// ```
///
pub fn init<K: BTreeGeneralTypeTrait + Ord, T: BTreeGeneralTypeTrait>(degree: usize) -> BTree<K, T> {
  let key = K::default();
  let data = T::default();

  let key_size: usize = key.get_byte_size();
  let data_size: usize = data.get_byte_size();
  if data_size == 0 || key_size == 0 {
    panic!("Data OR Key Size is 0");
  }

  debug!("Data Size : {}", data_size);

  let is_initialized = GLOBAL_DEGREE_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst);
  if is_initialized {
    panic!("B-Tree is already initialized.\nB-Tree가 이미 초기화 된 상태입니다.");
  } else if degree < 2 || degree % 2 == 0 {
    panic!("Degree must be greater or equal than 2 and odd Number\n차수는 2 이상이여야 하며 홀수여야 합니다.");
  }

  BTree::new::<K, T>(degree);
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
    init::<usize, usize>(3);
    assert_eq!(GLOBAL_DEGREE.load(std::sync::atomic::Ordering::SeqCst), 3);

    let result = std::panic::catch_unwind(|| init::<usize, u64>(4));
    assert!(result.is_err());

    let result = std::panic::catch_unwind(|| init::<usize, u64>(3));
    assert!(result.is_err());

    GLOBAL_DEGREE_INITIALIZED.store(false, std::sync::atomic::Ordering::SeqCst);
    GLOBAL_DATA_SIZE.store(0, std::sync::atomic::Ordering::SeqCst);
    GLOBAL_DEGREE.store(0, std::sync::atomic::Ordering::SeqCst);
  }
}


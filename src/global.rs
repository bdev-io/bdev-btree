use std::sync::atomic::{AtomicUsize, AtomicBool};

/// ## Global Variable
/// 
/// `GLOBAL_DEGREE` : The degree of the B-Tree
/// TYPE : GLOBAL DEGREE OF B-Tree, initialize : using `init()` function in `func.rs`
/// TYPE : B-Tree 의 차수, 전역 변수 <기본: 3> 초기화는 `func.rs` 의 `init()` 에서 진행됨.
///
pub static GLOBAL_DEGREE: AtomicUsize = AtomicUsize::new(3);
pub static GLOBAL_DEGREE_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub static GLOBAL_DATA_SIZE: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub fn reset_global() {
  GLOBAL_DEGREE.store(3, std::sync::atomic::Ordering::SeqCst);
  GLOBAL_DEGREE_INITIALIZED.store(false, std::sync::atomic::Ordering::SeqCst);
  GLOBAL_DATA_SIZE.store(0, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::Ordering;

  #[tokio::test(flavor = "current_thread")]
  async fn test_global_degree_load_first() {
    assert!(GLOBAL_DEGREE.load(Ordering::Relaxed) == 3);
  }

  #[tokio::test(flavor = "current_thread")]
  async fn test_global_degree_store() {
    // HACK : This is a hack to make sure the test is run after the previous test
    reset_global();
    GLOBAL_DEGREE.store(1, Ordering::SeqCst);
    GLOBAL_DEGREE_INITIALIZED.store(true, Ordering::SeqCst);
    assert!(GLOBAL_DEGREE.load(Ordering::SeqCst) == 1);
    assert!(GLOBAL_DEGREE_INITIALIZED.load(Ordering::SeqCst));
    reset_global();
  }

  #[tokio::test(flavor = "current_thread")]
  async fn test_global_degree_is_set() {
    // HACK : This is a hack to make sure the test is run after the previous test (degree_store is false setted)
    assert!(!GLOBAL_DEGREE_INITIALIZED.load(Ordering::Relaxed));
    reset_global();
  }

  struct TestStruct {
    _a: u64,
    _b: u64,
    _pin: std::marker::PhantomData<u64>,
  }

  #[tokio::test(flavor = "current_thread")]
  async fn test_data_size_calculation() {
    let usize_size: usize = std::mem::size_of::<usize>();
    let u64_size: usize = std::mem::size_of::<u64>();
    let u32_size: usize = std::mem::size_of::<u32>();
    let custom_size: usize = std::mem::size_of::<TestStruct>();

    assert!(usize_size > 0);
    assert!(u64_size > 0);
    assert!(u32_size > 0);
    assert!(custom_size > 0);
    reset_global();
  }
}

use std::sync::atomic::{AtomicUsize, AtomicBool};

/// ## Global Variable
/// 
/// `GLOBAL_DEGREE` : The degree of the B-Tree
/// TYPE : GLOBAL DEGREE OF B-Tree, initialize : using `init()` function in `func.rs`
/// TYPE : B-Tree 의 차수, 전역 변수 <기본: 3> 초기화는 `func.rs` 의 `init()` 에서 진행됨.
///
pub static GLOBAL_DEGREE: AtomicUsize = AtomicUsize::new(3);
pub static GLOBAL_DEGREE_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::Ordering;

  #[tokio::test]
  async fn test_global_degree_load_first() {
    assert!(GLOBAL_DEGREE.load(Ordering::Relaxed) == 3);
  }

  #[tokio::test]
  async fn test_global_degree_store() {
    tokio::time::sleep_until(tokio::time::Instant::now() + std::time::Duration::from_secs(1)).await;
    // HACK : This is a hack to make sure the test is run after the previous test
    GLOBAL_DEGREE.store(1, Ordering::SeqCst);
    GLOBAL_DEGREE_INITIALIZED.store(true, Ordering::SeqCst);
    assert!(GLOBAL_DEGREE.load(Ordering::Relaxed) == 1);
  }

  #[tokio::test]
  async fn test_global_degree_is_set() {
    tokio::time::sleep_until(tokio::time::Instant::now() + std::time::Duration::from_secs(2)).await;
    // HACK : This is a hack to make sure the test is run after the previous test (degree_store)
    assert!(GLOBAL_DEGREE_INITIALIZED.load(Ordering::Relaxed));
  }
}

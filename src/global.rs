use std::sync::atomic::AtomicUsize;

pub static GLOBAL_DEGREE: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::Ordering;

  #[tokio::test]
  async fn test_global_degree_load_first() {
    assert!(GLOBAL_DEGREE.load(Ordering::Relaxed) == 0);
  }

  #[tokio::test]
  async fn test_global_degree_store() {
    tokio::time::sleep_until(tokio::time::Instant::now() + std::time::Duration::from_secs(1)).await;
    // HACK : This is a hack to make sure the test is run after the previous test
    GLOBAL_DEGREE.store(1, Ordering::SeqCst);
    assert!(GLOBAL_DEGREE.load(Ordering::Relaxed) == 1);
  }
}

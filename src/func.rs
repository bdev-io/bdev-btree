use super::global::{ GLOBAL_DEGREE, GLOBAL_DEGREE_INITIALIZED };
// NOTE : Hackable Degree

/// ## Functions for B-Tree
/// 
/// ### `init()`
/// 
/// Initialize the B-Tree
///
/// * `degree` : The degree of the B-Tree ( usize )
/// * `return` : None
///
/// # Example
///
/// ```
/// btree::init(3);
/// ```
///
pub fn init(degree: usize) {
  let is_initialized = GLOBAL_DEGREE_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst);
  if is_initialized {
    panic!("B-Tree is already initialized.\nB-Tree가 이미 초기화 된 상태입니다.");
  } else if degree < 2 || degree % 2 == 0 {
    panic!("Degree must be greater or equal than 2 and odd Number\n차수는 2 이상이여야 하며 홀수여야 합니다.");
  }

  GLOBAL_DEGREE.store(degree, std::sync::atomic::Ordering::SeqCst);
}

// INFO : TEST

#[cfg(test)]
mod tests {
  use super::*;
  #[tokio::test]
  async fn test_init() {
    // DOC : This Init Test will execute latest
    tokio::time::sleep_until(tokio::time::Instant::now() + std::time::Duration::from_millis(1000)).await;
    init(3);
    assert_eq!(GLOBAL_DEGREE.load(std::sync::atomic::Ordering::SeqCst), 3);
  }

  #[tokio::test]
  async fn test_re_init() {
    // DOC : Re init Must Be Panic
    tokio::time::sleep_until(tokio::time::Instant::now() + std::time::Duration::from_millis(2000)).await;
    let result = std::panic::catch_unwind(|| init(4));
    assert!(result.is_err());
  }
}


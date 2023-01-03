use super::global::GLOBAL_DEGREE;

pub static B_DEGREE: &std::sync::atomic::AtomicUsize = &GLOBAL_DEGREE;

#[cfg(test)]
mod tests {
  use super::*;
  #[tokio::test]
  async fn test_degree() {
    assert!(B_DEGREE.load(std::sync::atomic::Ordering::Relaxed) > 0);
  }
}


use super::global::GLOBAL_DEGREE;

const BUFFER_FACTOR: usize = 512;

pub static B_DEGREE: &std::sync::atomic::AtomicUsize = &GLOBAL_DEGREE;
pub const B_HEADER_SIZE: usize = BUFFER_FACTOR * 2;

#[cfg(test)]
mod tests {
  use super::*;

  /// DOC : MUST READ TEST ONLY ON OTHER MODULE ( GLOBAL )
  #[tokio::test(flavor = "current_thread")]
  async fn test_degree() {
    assert!(B_DEGREE.load(std::sync::atomic::Ordering::Relaxed) > 0);
  }
}


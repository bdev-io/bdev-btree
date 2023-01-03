use super::global::GLOBAL_DEGREE;

const BUFFER_FACTOR: usize = 512;

pub static B_DEGREE: &std::sync::atomic::AtomicUsize = &GLOBAL_DEGREE;
pub const B_HEADER_SIZE: usize = BUFFER_FACTOR * 2;

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test(flavor = "current_thread")]
  async fn test_degree() {
    B_DEGREE.store(4, std::sync::atomic::Ordering::Relaxed);
    assert!(B_DEGREE.load(std::sync::atomic::Ordering::Relaxed) > 0);
    B_DEGREE.store(3, std::sync::atomic::Ordering::Relaxed);
  }
}


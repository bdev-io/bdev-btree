use super::global::{ GLOBAL_DEGREE, GLOBAL_DATA_SIZE };

const BUFFER_FACTOR: usize = 512;

/// ## B-Tree DEGREE ( B-TREE 차수)
pub static B_DEGREE: &std::sync::atomic::AtomicUsize = &GLOBAL_DEGREE;
/// ## B-Tree File Header Size ( B-TREE 파일 헤더 크기 )
pub const B_HEADER_SIZE: usize = BUFFER_FACTOR * 2;
/// ## B-Tree Node Raw Byte Size ( B-TREE 노드의 바이트 크기 )
pub static B_NODE_SIZE: &std::sync::atomic::AtomicUsize = &GLOBAL_DATA_SIZE;

#[cfg(test)]
mod tests {
  use super::*;

  /// DOC : MUST READ TEST ONLY ON OTHER MODULE ( GLOBAL )
  #[tokio::test(flavor = "current_thread")]
  async fn test_degree() {
    assert!(B_DEGREE.load(std::sync::atomic::Ordering::Relaxed) > 0);
  }
}


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
mod tests {
  struct TestStruct {
    _a: u64,
    _b: u64,
    _pin: std::marker::PhantomData<u64>,
  }
  #[tokio::test]
  async fn test_data_size_calculation() {
    let usize_size: usize = std::mem::size_of::<usize>();
    let u64_size: usize = std::mem::size_of::<u64>();
    let u32_size: usize = std::mem::size_of::<u32>();
    let custom_size: usize = std::mem::size_of::<TestStruct>();

    assert!(usize_size > 0);
    assert!(u64_size > 0);
    assert!(u32_size > 0);
    assert!(custom_size > 0);
  }
}

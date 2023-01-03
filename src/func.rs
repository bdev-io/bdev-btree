use super::global::{ GLOBAL_DEGREE };
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
/// ```
/// btree::init(3);
/// ```
pub fn init(degree: usize) {
  GLOBAL_DEGREE.store(degree, std::sync::atomic::Ordering::SeqCst);
}


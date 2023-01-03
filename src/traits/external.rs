//! # B-Tree Data Structure Trait
//!
//! This crate provides a trait for B-Tree data structure.
//! This trait provide a common interface for all B-Tree data structure.
//!
//! ## Example for Other Structure
//! ```rust
//! struct MyBTreeData {
//!   x: u32,
//!   y: u32,
//! } // INFO : This will be DATA STRUCTURE
//!
//! use btree::traits::BTreeDataTrait;
//! impl BTreeDataTrait for MyBTreeData {
//!   fn get_byte_size(&self) -> usize {
//!     std::mem::size_of::<u32>() * 2
//!   }
//!   fn to_bytes(&self) -> Vec<u8> {
//!     let mut bytes = Vec::new();
//!     bytes.extend_from_slice(&self.x.to_be_bytes());
//!     bytes.extend_from_slice(&self.y.to_be_bytes());
//!     bytes
//!   }
//!   fn from_bytes(bytes: &[u8]) -> Self {
//!     let x = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
//!     let y = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
//!     Self { x, y }
//!   }
//! }
//! ```
//!
pub trait BTreeDataTrait {
  /// DOC : This Function Will Called, When B-Tree Need Data Size
  fn get_byte_size(&self) -> usize;
  /// DOC : This Function Will Called, When B-Tree Need Data Raw Bytes
  fn to_bytes(&self) -> Vec<u8>;
  /// DOC : This Function Will Called, When B-Tree Get Node From Raw Bytes
  fn from_raw_bytes(raw: &[u8]) -> Self;
}

/// DOC : Generic B-Tree Implementation for B-Tree Data Structure

macro_rules! int_trait_impl {
  ($name:ident for $($t:ty)*) => ($(
    impl $name for $t {
      #[inline]
      fn get_byte_size(&self) -> usize {
        std::mem::size_of::<$t>()
      }
      #[inline]
      fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
      }
      #[inline]
      fn from_raw_bytes(raw: &[u8]) -> Self {
        Self::from_be_bytes(raw.try_into().unwrap())
      }
    }
  )*)
}

int_trait_impl!(BTreeDataTrait for u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_int_trait_impl() {
    let x = 10u32;
    assert_eq!(x.get_byte_size(), 4);
    assert_eq!(x.to_bytes(), vec![0, 0, 0, 10]);
    assert_eq!(u32::from_raw_bytes(&x.to_bytes()), 10);
  }
}

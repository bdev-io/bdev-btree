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
//! use btree::traits::BTreeGeneralTypeTrait;
//! impl BTreeGeneralTypeTrait for MyBTreeData {
//!   fn get_byte_size(&self) -> usize {
//!     self.to_le_bytes().len()
//!   }
//!   fn to_raw_bytes(&self) -> Vec<u8> {
//!     let mut bytes = Vec::new();
//!     bytes.extend_from_slice(&self.x.to_le_bytes());
//!     bytes.extend_from_slice(&self.y.to_le_bytes());
//!     bytes
//!   }
//!   fn from_bytes(bytes: &[u8]) -> Self {
//!     let x = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
//!     let y = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
//!     Self { x, y }
//!   }
//! }
//! impl Default for MyBTreeData {
//!   fn default() -> Self {
//!     Self { x: 0, y: 0 }
//!   }
//! }
//! ```
//!
pub trait BTreeGeneralTypeTrait where Self: Default {
  /// DOC : This Function Will Called, When B-Tree Need Data Size
  fn get_byte_size(&self) -> usize;
  /// DOC : This Function Will Called, When B-Tree Need Data Raw Bytes
  fn to_raw_bytes(&self) -> Vec<u8>;
  /// DOC : This Function Will Called, When B-Tree Get Node From Raw Bytes
  fn from_raw_bytes(raw: &[u8]) -> Self;
}

/// DOC : Generic B-Tree Implementation for B-Tree Data Structure
macro_rules! int_trait_impl {
  ($name:ident for $($t:ty)*) => ($(
    impl $name for $t {
      #[inline]
      fn get_byte_size(&self) -> usize {
        self.to_le_bytes().len()
      }
      #[inline]
      fn to_raw_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
      }
      #[inline]
      fn from_raw_bytes(raw: &[u8]) -> Self {
        Self::from_le_bytes(raw.try_into().unwrap())
      }
    }
  )*)
}

int_trait_impl!(BTreeGeneralTypeTrait for u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize);

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_int_trait_impl() {
    let x = 10u32;
    assert_eq!(x.get_byte_size(), 4);
    assert_eq!(x.to_raw_bytes(), vec![0, 0, 0, 10]);
    assert_eq!(u32::from_raw_bytes(&x.to_raw_bytes()), 10);
  }

  #[tokio::test]
  async fn test_trait_implement_my_custom_struct() {
    #[derive(Default)]
    struct MyBTreeData {
      x: u32,
      y: u32,
    }

    impl BTreeGeneralTypeTrait for MyBTreeData {
      fn get_byte_size(&self) -> usize {
        (u32::default().to_le_bytes().len()) * 2
      }
      fn to_raw_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.x.to_le_bytes());
        bytes.extend_from_slice(&self.y.to_le_bytes());
        bytes
      }
      fn from_raw_bytes(raw: &[u8]) -> Self {
        let x = u32::from_le_bytes(raw[0..4].try_into().unwrap());
        let y = u32::from_le_bytes(raw[4..8].try_into().unwrap());
        Self { x, y }
      }
    }

    let origin_data = MyBTreeData { x: 10, y: 20 };
    let origin_byte_size = origin_data.get_byte_size();
    let origin_raw_bytes = origin_data.to_raw_bytes();
    println!("origin_data_bytes : {:?}", origin_raw_bytes);
    let restore_data = MyBTreeData::from_raw_bytes(&origin_raw_bytes);
    assert!(origin_data.x == restore_data.x);
    assert!(origin_data.y == restore_data.y);
    assert!(origin_byte_size == restore_data.get_byte_size());
    assert!(origin_raw_bytes == restore_data.to_raw_bytes());

  }
}

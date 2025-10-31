use std::fmt::Debug;

use iterator::ArrayIterator;

use crate::builder::ArrayBuilder;

pub trait Array: Sized + 'static {
  type Builder: ArrayBuilder<Array = Self>;

  type Item: Debug;

  type RefItem<'a>: Copy + Debug;

  fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

  fn len(&self) -> usize;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn iter(&self) -> ArrayIterator<Self>;
}

mod iterator;

mod primitive_array;
pub use primitive_array::I32Array;
pub use primitive_array::PrimitiveArray;

mod string_array;
pub use string_array::StringArray;

#[cfg(test)]
mod tests {
  use crate::array::primitive_array::I32Array;

  use super::*;

  fn build_array_from_vec<A: Array>(items: &[Option<A::RefItem<'_>>]) -> A {
    let mut builder = A::Builder::with_capacity(items.len());
    for item in items {
      builder.push(*item);
    }
    builder.finish()
  }

  fn check_array_eq<'a, A: Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>])
  where
    A::RefItem<'a>: PartialEq,
  {
    for (a, b) in array.iter().zip(vec.iter()) {
      assert_eq!(&a, b);
    }
  }

  #[test]
  fn test_build_int32_array() {
    let data = vec![Some(1), Some(2), Some(3), None, Some(5)];
    let array = build_array_from_vec::<I32Array>(&data[..]);
    check_array_eq(&array, &data[..]);
  }

  #[test]
  fn test_build_string_array() {
    let data = vec![Some("1"), Some("2"), Some("3"), None, Some("5"), Some("")];
    let array = build_array_from_vec::<StringArray>(&data[..]);
    check_array_eq(&array, &data[..]);
  }
}

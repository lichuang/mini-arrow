use iterator::ArrayIterator;

use crate::builder::ArrayBuilder;
use crate::scalar::Scalar;
use crate::scalar::ScalarRef;

pub trait Array: Sized + 'static + TryFrom<ArrayImpl> + Into<ArrayImpl>
where
  for<'a> Self::Item: Scalar<RefType<'a> = Self::RefItem<'a>>,
{
  type Builder: ArrayBuilder<Array = Self>;

  type Item: Scalar<ArrayType = Self>;

  type RefItem<'a>: ScalarRef<'a, ScalarType = Self::Item, ArrayType = Self>;

  fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

  fn len(&self) -> usize;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn iter(&self) -> ArrayIterator<Self>;

  fn from_slice(data: &[Option<Self::RefItem<'_>>]) -> Self {
    let mut builder = Self::Builder::with_capacity(data.len());
    for item in data {
      builder.push(*item);
    }
    builder.finish()
  }
}

pub enum ArrayImpl {
  Int32(I32Array),
  String(StringArray),
}

/* saved by impl macros
impl TryFrom<ArrayImpl> for I32Array {
  type Error = ();

  fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Int32(array) => Ok(array),
      _ => Err(()),
    }
  }
}

impl From<I32Array> for ArrayImpl {
  fn from(array: I32Array) -> Self {
    ArrayImpl::Int32(array)
  }
}

impl TryFrom<ArrayImpl> for StringArray {
  type Error = ();

  fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::String(array) => Ok(array),
      _ => Err(()),
    }
  }
}

impl From<StringArray> for ArrayImpl {
  fn from(array: StringArray) -> Self {
    ArrayImpl::String(array)
  }
}
*/

mod impls;
mod iterator;

mod primitive_array;
pub use primitive_array::I32Array;
pub use primitive_array::PrimitiveArray;

mod string_array;
pub use string_array::StringArray;

#[cfg(test)]
mod tests {
  use crate::{TypeMismatch, array::primitive_array::I32Array, builder::I32ArrayBuilder};

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

  fn add_i32(i1: i32, i2: i32) -> i32 {
    i1 + i2
  }

  fn add_i32_vec(i1: I32Array, i2: I32Array) -> I32Array {
    let mut builder = I32ArrayBuilder::with_capacity(i1.len());
    for (a, b) in i1.iter().zip(i2.iter()) {
      builder.push(a.and_then(|a| b.map(|b| add_i32(a, b))));
    }
    builder.finish()
  }

  fn add_i32_wrapper(i1: ArrayImpl, i2: ArrayImpl) -> Result<ArrayImpl, TypeMismatch> {
    Ok(add_i32_vec(i1.try_into()?, i2.try_into()?).into())
  }

  #[test]
  fn test_add_array() {
    check_array_eq::<I32Array>(
      &add_i32_wrapper(
        I32Array::from_slice(&[Some(1), Some(2), Some(3), None]).into(),
        I32Array::from_slice(&[Some(1), Some(2), None, Some(4)]).into(),
      )
      .unwrap()
      .try_into()
      .unwrap(),
      &[Some(2), Some(4), None, None],
    );

    let result = add_i32_wrapper(
      StringArray::from_slice(&[Some("1"), Some("2"), Some("3"), None]).into(),
      I32Array::from_slice(&[Some(1), Some(2), None, Some(4)]).into(),
    );
    assert!(result.is_err());
  }
}

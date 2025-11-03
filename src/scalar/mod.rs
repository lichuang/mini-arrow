use std::fmt::Debug;

use crate::array::Array;

pub trait Scalar: 'static + Clone + Debug + TryFrom<ScalarImpl> + Into<ScalarImpl> {
  type ArrayType: Array<Item = Self>;

  type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;

  fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

pub trait ScalarRef<'a>:
  'a + Clone + Copy + Debug + TryFrom<ScalarRefImpl<'a>> + Into<ScalarRefImpl<'a>>
{
  type ArrayType: Array<RefItem<'a> = Self>;

  type ScalarType: Scalar<RefType<'a> = Self>;

  fn as_scalar(&self) -> Self::ScalarType;
}

pub enum ScalarImpl {
  Int32(i32),
  Int64(i64),
  Bool(bool),
  String(String),
}

impl ScalarImpl {
  pub fn identifier(&self) -> &'static str {
    match self {
      Self::Int32(_) => stringify!(Int32),
      Self::Int64(_) => stringify!(Int64),
      Self::Bool(_) => stringify!(Bool),
      Self::String(_) => stringify!(String),
    }
  }
}

pub enum ScalarRefImpl<'a> {
  Int32(i32),
  Int64(i64),
  Bool(bool),
  String(&'a str),
}

impl<'a> ScalarRefImpl<'a> {
  pub fn identifier(&self) -> &'static str {
    match self {
      Self::Int32(_) => stringify!(Int32),
      Self::Int64(_) => stringify!(Int64),
      Self::Bool(_) => stringify!(Bool),
      Self::String(_) => stringify!(String),
    }
  }
}

//mod impls;
mod primitive_scalar;
mod string_scalar;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::array::{Array, I32Array, StringArray};
  use crate::builder::ArrayBuilder;

  fn build_array_repeated<A: Array>(item: A::RefItem<'_>, len: usize) -> A {
    let mut builder = A::Builder::with_capacity(len);
    for _ in 0..len {
      builder.push(Some(item));
    }
    builder.finish()
  }

  fn build_array_repeated_item<A: Array>(item: A::Item, len: usize) -> A {
    let mut builder = A::Builder::with_capacity(len);
    for _ in 0..len {
      builder.push(Some(item.as_scalar_ref()));
    }
    builder.finish()
  }

  fn check_array_eq<'a, A: Array>(array: &'a A, item: A::RefItem<'a>)
  where
    A::RefItem<'a>: PartialEq,
  {
    for a in array.iter() {
      assert_eq!(a, Some(item));
    }
  }

  #[test]
  fn test_build_int32_repeat_array() {
    let array = build_array_repeated::<I32Array>(1, 233);
    check_array_eq(&array, 1);
    let array = build_array_repeated_item::<I32Array>(1, 233);
    check_array_eq(&array, 1);
  }

  #[test]
  fn test_build_string_repeat_array() {
    let array = build_array_repeated::<StringArray>("233", 5);
    check_array_eq(&array, "233");
    let array = build_array_repeated_item::<StringArray>("233".to_string(), 5);
    check_array_eq(&array, "233");
  }

  #[test]
  fn test_try_from_into() {
    {
      let i: i32 = 2333;
      let j: ScalarImpl = i.into();
      let k: ScalarRefImpl = i.into();
      let i1: i32 = j.try_into().unwrap();
      let i2: i32 = k.try_into().unwrap();
      assert_eq!(i1, i);
      assert_eq!(i2, i);
    }
    {
      let hello_str = "hello";
      let hello_string = "hello".to_string();
      let j: ScalarImpl = hello_string.clone().into();
      let k: ScalarRefImpl = hello_str.into();
      let i1: String = j.try_into().unwrap();
      let i2: &str = k.try_into().unwrap();
      assert_eq!(i1, hello_string);
      assert_eq!(i2, hello_str);
    }
  }
}

use crate::array::I32Array;

use super::{Scalar, ScalarRef};

impl Scalar for i32 {
  type ArrayType = I32Array;
  type RefType<'a> = i32;

  fn as_scalar_ref(&self) -> i32 {
    *self
  }
}

impl<'a> ScalarRef<'a> for i32 {
  type ArrayType = I32Array;
  type ScalarType = i32;

  fn as_scalar(&self) -> i32 {
    *self
  }
}

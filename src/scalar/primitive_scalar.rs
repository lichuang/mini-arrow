use crate::{TypeMismatch, array::I32Array};

use super::{Scalar, ScalarImpl, ScalarRef, ScalarRefImpl};

impl Scalar for i32 {
  type ArrayType = I32Array;
  type RefType<'a> = i32;

  fn as_scalar_ref(&self) -> i32 {
    *self
  }
}

#[doc = concat!(
                "Implement [`ScalarImpl`] -> [`", stringify!(i32), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarImpl> for i32 {
  type Error = TypeMismatch;
  fn try_from(that: ScalarImpl) -> Result<Self, Self::Error> {
    match that {
      ScalarImpl::Int32(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(Int32), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(i32), "`] -> [`ScalarImpl`]")]
impl From<i32> for ScalarImpl {
  fn from(that: i32) -> Self {
    ScalarImpl::Int32(that)
  }
}

impl<'a> ScalarRef<'a> for i32 {
  type ArrayType = I32Array;
  type ScalarType = i32;

  fn as_scalar(&self) -> i32 {
    *self
  }
}

#[doc = concat!(
                "Implement [`ScalarRefImpl`] -> [`", stringify!(i32), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarRefImpl<'a>> for i32 {
  type Error = TypeMismatch;
  fn try_from(that: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
    match that {
      ScalarRefImpl::Int32(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(Int32), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(i32), "`] -> [`ScalarRefImpl`]")]
impl<'a> From<i32> for ScalarRefImpl<'a> {
  fn from(that: i32) -> Self {
    ScalarRefImpl::Int32(that)
  }
}

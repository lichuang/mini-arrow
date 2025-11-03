use crate::{
  TypeMismatch,
  array::{BoolArray, I32Array, I64Array},
};

use super::{Scalar, ScalarImpl, ScalarRef, ScalarRefImpl};

// Scalar for i32
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

// Scalar for bool
impl Scalar for bool {
  type ArrayType = BoolArray;
  type RefType<'a> = bool;

  fn as_scalar_ref(&self) -> bool {
    *self
  }
}

#[doc = concat!(
                "Implement [`ScalarImpl`] -> [`", stringify!(bool), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarImpl> for bool {
  type Error = TypeMismatch;
  fn try_from(that: ScalarImpl) -> Result<Self, Self::Error> {
    match that {
      ScalarImpl::Bool(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(Bool), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(bool), "`] -> [`ScalarImpl`]")]
impl From<bool> for ScalarImpl {
  fn from(that: bool) -> Self {
    ScalarImpl::Bool(that)
  }
}

impl<'a> ScalarRef<'a> for bool {
  type ArrayType = BoolArray;
  type ScalarType = bool;

  fn as_scalar(&self) -> bool {
    *self
  }
}

#[doc = concat!(
                "Implement [`ScalarRefImpl`] -> [`", stringify!(bool), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarRefImpl<'a>> for bool {
  type Error = TypeMismatch;
  fn try_from(that: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
    match that {
      ScalarRefImpl::Bool(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(Bool), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(i32), "`] -> [`ScalarRefImpl`]")]
impl<'a> From<bool> for ScalarRefImpl<'a> {
  fn from(that: bool) -> Self {
    ScalarRefImpl::Bool(that)
  }
}

// Scalar for i64
impl Scalar for i64 {
  type ArrayType = I64Array;
  type RefType<'a> = i64;

  fn as_scalar_ref(&self) -> i64 {
    *self
  }
}

#[doc = concat!(
                "Implement [`ScalarImpl`] -> [`", stringify!(i64), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarImpl> for i64 {
  type Error = TypeMismatch;
  fn try_from(that: ScalarImpl) -> Result<Self, Self::Error> {
    match that {
      ScalarImpl::Int64(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(Int64), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(i64), "`] -> [`ScalarImpl`]")]
impl From<i64> for ScalarImpl {
  fn from(that: i64) -> Self {
    ScalarImpl::Int64(that)
  }
}

impl<'a> ScalarRef<'a> for i64 {
  type ArrayType = I64Array;
  type ScalarType = i64;

  fn as_scalar(&self) -> i64 {
    *self
  }
}

#[doc = concat!(
                "Implement [`ScalarRefImpl`] -> [`", stringify!(i64), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarRefImpl<'a>> for i64 {
  type Error = TypeMismatch;
  fn try_from(that: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
    match that {
      ScalarRefImpl::Int64(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(Bool), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(i64), "`] -> [`ScalarRefImpl`]")]
impl<'a> From<i64> for ScalarRefImpl<'a> {
  fn from(that: i64) -> Self {
    ScalarRefImpl::Int64(that)
  }
}

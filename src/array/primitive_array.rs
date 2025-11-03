use bitvec::vec::BitVec;

use crate::{
  PrimitiveType, TypeMismatch,
  builder::PrimitiveArrayBuilder,
  scalar::{Scalar, ScalarRef},
};

use super::{Array, ArrayImpl, iterator::ArrayIterator};

pub struct PrimitiveArray<T: PrimitiveType> {
  /// The actual data of this array.
  data: Vec<T>,

  /// The null bitmap of this array.
  bitmap: BitVec,
}

pub type I32Array = PrimitiveArray<i32>;
pub type I64Array = PrimitiveArray<i64>;
pub type BoolArray = PrimitiveArray<bool>;

impl PrimitiveType for i32 {}
impl PrimitiveType for i64 {}
impl PrimitiveType for bool {}

impl<T: PrimitiveType> Array for PrimitiveArray<T>
where
  T: PrimitiveType,
  T: Scalar<ArrayType = Self>,
  for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = Self>,
  for<'a> T: Scalar<RefType<'a> = T>,
  Self: Into<ArrayImpl>,
  Self: TryFrom<ArrayImpl>,
{
  type Builder = PrimitiveArrayBuilder<T>;

  type Item = T;

  type RefItem<'a> = T;

  fn get(&self, idx: usize) -> Option<T> {
    if self.bitmap[idx] {
      Some(self.data[idx])
    } else {
      None
    }
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn iter(&self) -> ArrayIterator<Self> {
    ArrayIterator::new(self)
  }
}

impl<T: PrimitiveType> PrimitiveArray<T> {
  pub fn new(data: Vec<T>, bitmap: BitVec) -> Self {
    Self { data, bitmap }
  }
}

impl TryFrom<ArrayImpl> for I32Array {
  type Error = TypeMismatch;

  fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Int32(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(Int32), other.identifier())),
    }
  }
}

impl<'a> TryFrom<&'a ArrayImpl> for &'a I32Array {
  type Error = TypeMismatch;

  fn try_from(array: &'a ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Int32(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(Int32), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(I32Array), "`] -> [`ArrayImpl`]")]
impl From<I32Array> for ArrayImpl {
  fn from(array: I32Array) -> Self {
    ArrayImpl::Int32(array)
  }
}

impl TryFrom<ArrayImpl> for BoolArray {
  type Error = TypeMismatch;

  fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Bool(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(Bool), other.identifier())),
    }
  }
}

impl<'a> TryFrom<&'a ArrayImpl> for &'a BoolArray {
  type Error = TypeMismatch;

  fn try_from(array: &'a ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Bool(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(Bool), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(BoolArray), "`] -> [`ArrayImpl`]")]
impl From<BoolArray> for ArrayImpl {
  fn from(array: BoolArray) -> Self {
    ArrayImpl::Bool(array)
  }
}

impl TryFrom<ArrayImpl> for I64Array {
  type Error = TypeMismatch;

  fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Int64(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(Int64), other.identifier())),
    }
  }
}

impl<'a> TryFrom<&'a ArrayImpl> for &'a I64Array {
  type Error = TypeMismatch;

  fn try_from(array: &'a ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::Int64(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(Int64), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(I64Array), "`] -> [`ArrayImpl`]")]
impl From<I64Array> for ArrayImpl {
  fn from(array: I64Array) -> Self {
    ArrayImpl::Int64(array)
  }
}

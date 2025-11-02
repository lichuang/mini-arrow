use bitvec::vec::BitVec;

use crate::{
  PrimitiveType, TypeMismatch,
  array::{ArrayImpl, PrimitiveArray},
  scalar::{Scalar, ScalarRef},
};

use super::{ArrayBuilder, ArrayBuilderImpl};

pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
  /// The actual data of this array.
  data: Vec<T>,

  /// The null bitmap of this array.
  bitmap: BitVec,
}

impl<T: PrimitiveType> ArrayBuilder for PrimitiveArrayBuilder<T>
where
  T: PrimitiveType,
  T: Scalar<ArrayType = PrimitiveArray<T>>,
  for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = PrimitiveArray<T>>,
  for<'a> T: Scalar<RefType<'a> = T>,
  PrimitiveArray<T>: Into<ArrayImpl>,
  PrimitiveArray<T>: TryFrom<ArrayImpl>,
{
  type Array = PrimitiveArray<T>;

  fn with_capacity(capacity: usize) -> Self {
    Self {
      data: Vec::with_capacity(capacity),
      bitmap: BitVec::with_capacity(capacity),
    }
  }

  fn push(&mut self, value: Option<T>) {
    match value {
      Some(v) => {
        self.data.push(v);
        self.bitmap.push(true);
      }
      None => {
        self.data.push(T::default());
        self.bitmap.push(false);
      }
    }
  }

  fn finish(self) -> Self::Array {
    PrimitiveArray::new(self.data, self.bitmap)
  }
}

pub type I32ArrayBuilder = PrimitiveArrayBuilder<i32>;

#[doc = concat!("Implement [`ArrayBuilderImpl`] -> [`", stringify!(I32ArrayBuilder), "`]")]
impl TryFrom<ArrayBuilderImpl> for I32ArrayBuilder {
  type Error = TypeMismatch;

  fn try_from(builder: ArrayBuilderImpl) -> Result<Self, Self::Error> {
    match builder {
      ArrayBuilderImpl::Int32(builder) => Ok(builder),
      other => Err(TypeMismatch(stringify!(Int32), other.identifier())),
    }
  }
}

#[doc = concat!("Implement mut ref of [`ArrayBuilderImpl`] -> [`", stringify!(I32ArrayBuilder), "`]")]
impl<'a> TryFrom<&'a mut ArrayBuilderImpl> for &'a mut I32ArrayBuilder {
  type Error = TypeMismatch;

  fn try_from(builder: &'a mut ArrayBuilderImpl) -> Result<Self, Self::Error> {
    match builder {
      ArrayBuilderImpl::Int32(builder) => Ok(builder),
      other => Err(TypeMismatch(stringify!(Int32), other.identifier())),
    }
  }
}

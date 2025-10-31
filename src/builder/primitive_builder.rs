use bitvec::vec::BitVec;

use crate::{
  PrimitiveType,
  array::PrimitiveArray,
  scalar::{Scalar, ScalarRef},
};

use super::ArrayBuilder;

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

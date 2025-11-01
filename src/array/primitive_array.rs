use bitvec::vec::BitVec;

use crate::{
  PrimitiveType,
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

impl PrimitiveType for i32 {}

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

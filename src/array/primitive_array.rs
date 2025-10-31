use bitvec::vec::BitVec;

use crate::{PrimitiveType, builder::PrimitiveArrayBuilder};

use super::{Array, iterator::ArrayIterator};

pub struct PrimitiveArray<T: PrimitiveType> {
  /// The actual data of this array.
  data: Vec<T>,

  /// The null bitmap of this array.
  bitmap: BitVec,
}

pub type I32Array = PrimitiveArray<i32>;

impl PrimitiveType for i32 {}
impl PrimitiveType for f32 {}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
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

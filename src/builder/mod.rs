use crate::array::Array;

pub trait ArrayBuilder {
  type Array: Array<Builder = Self>;

  fn with_capacity(capacity: usize) -> Self;

  fn push(&mut self, value: <Self::Array as Array>::RefItem<'_>);

  fn finish(self) -> Self::Array;
}

use iterator::ArrayIterator;

use crate::builder::ArrayBuilder;

pub trait Array: Sized {
  type Builder: ArrayBuilder;

  type Item;

  type RefItem<'a>: Copy
  where
    Self: 'a;

  fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

  fn len(&self) -> usize;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn iter(&self) -> ArrayIterator<Self>;
}

mod iterator;
mod primitive_array;

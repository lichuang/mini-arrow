use crate::array::Array;

pub trait ArrayBuilder {
  type Array: Array<Builder = Self>;

  fn with_capacity(capacity: usize) -> Self;

  fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

  fn finish(self) -> Self::Array;
}

mod primitive_builder;
pub use primitive_builder::PrimitiveArrayBuilder;

mod string_builder;
pub use string_builder::StringArrayBuilder;

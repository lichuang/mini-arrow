use crate::array::Array;

pub trait ArrayBuilder {
  type Array: Array<Builder = Self>;

  fn with_capacity(capacity: usize) -> Self;

  fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

  fn finish(self) -> Self::Array;
}

pub enum ArrayBuilderImpl {
  Int32(I32ArrayBuilder),
  Int64(I64ArrayBuilder),
  Bool(BoolArrayBuilder),
  String(StringArrayBuilder),
}

mod builder_impl;
//mod impls;

mod primitive_builder;
pub use primitive_builder::BoolArrayBuilder;
pub use primitive_builder::I32ArrayBuilder;
pub use primitive_builder::I64ArrayBuilder;
pub use primitive_builder::PrimitiveArrayBuilder;

mod string_builder;
pub use string_builder::StringArrayBuilder;

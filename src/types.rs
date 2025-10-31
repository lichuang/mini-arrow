use std::fmt::Debug;

/// A type that is primitive, such as `i32` and `i64`.
pub trait PrimitiveType: Default + Copy + Debug + 'static {}

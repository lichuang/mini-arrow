pub mod array;
pub mod builder;
//mod macros;
pub mod scalar;
pub mod types;

pub use types::PrimitiveType;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Type mismatch on conversion: expected {0}, get {1}")]
pub struct TypeMismatch(&'static str, &'static str);

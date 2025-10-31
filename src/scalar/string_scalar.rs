use crate::array::StringArray;

use super::{Scalar, ScalarRef};

impl Scalar for String {
  type ArrayType = StringArray;
  type RefType<'a> = &'a str;

  fn as_scalar_ref(&self) -> &str {
    self.as_str()
  }
}

impl<'a> ScalarRef<'a> for &'a str {
  type ArrayType = StringArray;
  type ScalarType = String;

  fn as_scalar(&self) -> String {
    self.to_string()
  }
}

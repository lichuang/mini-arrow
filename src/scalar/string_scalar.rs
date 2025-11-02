use crate::{TypeMismatch, array::StringArray};

use super::{Scalar, ScalarImpl, ScalarRef, ScalarRefImpl};

impl Scalar for String {
  type ArrayType = StringArray;
  type RefType<'a> = &'a str;

  fn as_scalar_ref(&self) -> &str {
    self.as_str()
  }
}

#[doc = concat!(
                "Implement [`ScalarImpl`] -> [`", stringify!(String), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarImpl> for String {
  type Error = ();
  fn try_from(that: ScalarImpl) -> Result<Self, Self::Error> {
    match that {
      ScalarImpl::String(v) => Ok(v),
      _ => Err(()),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(String), "`] -> [`ScalarImpl`]")]
impl From<String> for ScalarImpl {
  fn from(that: String) -> Self {
    ScalarImpl::String(that)
  }
}

impl<'a> ScalarRef<'a> for &'a str {
  type ArrayType = StringArray;
  type ScalarType = String;

  fn as_scalar(&self) -> String {
    self.to_string()
  }
}

#[doc = concat!(
                "Implement [`ScalarRefImpl`] -> [`", stringify!(str), "`], return [`TypeMismatch`] error if mismatch")]
impl<'a> TryFrom<ScalarRefImpl<'a>> for &'a str {
  type Error = TypeMismatch;
  fn try_from(that: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
    match that {
      ScalarRefImpl::String(v) => Ok(v),
      other => Err(TypeMismatch(stringify!(String), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(str), "`] -> [`ScalarRefImpl`]")]
impl<'a> From<&'a str> for ScalarRefImpl<'a> {
  fn from(that: &'a str) -> Self {
    ScalarRefImpl::String(that)
  }
}

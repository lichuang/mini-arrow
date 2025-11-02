use crate::scalar::ScalarRefImpl;

use super::{Array, ArrayImpl};

impl ArrayImpl {
  pub fn get(&self, idx: usize) -> Option<ScalarRefImpl<'_>> {
    match self {
      Self::Int32(array) => array.get(idx).map(ScalarRefImpl::Int32),
      Self::String(array) => array.get(idx).map(ScalarRefImpl::String),
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Self::Int32(array) => array.len(),
      Self::String(array) => array.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Self::Int32(array) => array.is_empty(),
      Self::String(array) => array.is_empty(),
    }
  }

  pub fn identifier(&self) -> &'static str {
    match self {
      Self::Int32(_) => stringify!(Int32),
      Self::String(_) => stringify!(String),
    }
  }
}

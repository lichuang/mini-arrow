use crate::scalar::ScalarRefImpl;

use super::{Array, ArrayImpl};

impl ArrayImpl {
  pub fn get(&self, idx: usize) -> Option<ScalarRefImpl<'_>> {
    match self {
      Self::Int32(array) => array.get(idx).map(ScalarRefImpl::Int32),
      Self::Int64(array) => array.get(idx).map(ScalarRefImpl::Int64),
      Self::Bool(array) => array.get(idx).map(ScalarRefImpl::Bool),
      Self::String(array) => array.get(idx).map(ScalarRefImpl::String),
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Self::Int32(array) => array.len(),
      Self::Int64(array) => array.len(),
      Self::Bool(array) => array.len(),
      Self::String(array) => array.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Self::Int32(array) => array.is_empty(),
      Self::Int64(array) => array.is_empty(),
      Self::Bool(array) => array.is_empty(),
      Self::String(array) => array.is_empty(),
    }
  }

  pub fn identifier(&self) -> &'static str {
    match self {
      Self::Int32(_) => stringify!(Int32),
      Self::Int64(_) => stringify!(Int64),
      Self::Bool(_) => stringify!(Bool),
      Self::String(_) => stringify!(String),
    }
  }
}

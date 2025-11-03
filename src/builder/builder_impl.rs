use super::{ArrayBuilder, ArrayBuilderImpl};
use crate::{TypeMismatch, array::ArrayImpl, scalar::ScalarRefImpl};

impl ArrayBuilderImpl {
  pub fn push(&mut self, v: Option<ScalarRefImpl<'_>>) {
    match (self, v) {
      // int32 array
      (Self::Int32(a), Some(ScalarRefImpl::Int32(v))) => a.push(Some(v)),
      (Self::Int32(a), None) => a.push(None),

      // int64 array
      (Self::Int64(a), Some(ScalarRefImpl::Int64(v))) => a.push(Some(v)),
      (Self::Int64(a), None) => a.push(None),

      // bool array
      (Self::Bool(a), Some(ScalarRefImpl::Bool(v))) => a.push(Some(v)),
      (Self::Bool(a), None) => a.push(None),

      // string array
      (Self::String(a), Some(ScalarRefImpl::String(v))) => a.push(Some(v)),
      (Self::String(a), None) => a.push(None),
      // other cases, type mismatched
      (other, Some(v)) => Err(TypeMismatch(other.identifier(), v.identifier())).unwrap(),
    }
  }

  pub fn finish(self) -> ArrayImpl {
    match self {
      Self::Int32(a) => ArrayImpl::Int32(a.finish()),
      Self::Int64(a) => ArrayImpl::Int64(a.finish()),
      Self::Bool(a) => ArrayImpl::Bool(a.finish()),
      Self::String(a) => ArrayImpl::String(a.finish()),
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

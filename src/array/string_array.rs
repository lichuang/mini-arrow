use bitvec::vec::BitVec;

use crate::{TypeMismatch, builder::StringArrayBuilder};

use super::{Array, ArrayImpl, iterator::ArrayIterator};

pub struct StringArray {
  /// The flattened data of string.
  data: Vec<u8>,

  /// Offsets of each string in the data flat array.
  offsets: Vec<usize>,

  /// The null bitmap of this array.
  bitmap: BitVec,
}

impl Array for StringArray {
  type Builder = StringArrayBuilder;

  type Item = String;

  /// For [`StringArray`], we can only get an `&str` out of it with zero overhead.
  type RefItem<'a> = &'a str;

  fn get(&self, idx: usize) -> Option<&str> {
    if self.bitmap[idx] {
      let range = self.offsets[idx]..self.offsets[idx + 1];
      Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
    } else {
      None
    }
  }

  fn len(&self) -> usize {
    self.bitmap.len()
  }

  fn iter(&self) -> ArrayIterator<Self> {
    ArrayIterator::new(self)
  }
}

impl StringArray {
  pub fn new(data: Vec<u8>, offsets: Vec<usize>, bitmap: BitVec) -> Self {
    Self {
      data,
      offsets,
      bitmap,
    }
  }
}

impl TryFrom<ArrayImpl> for StringArray {
  type Error = TypeMismatch;

  fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::String(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(String), other.identifier())),
    }
  }
}

impl<'a> TryFrom<&'a ArrayImpl> for &'a StringArray {
  type Error = TypeMismatch;

  fn try_from(array: &'a ArrayImpl) -> Result<Self, Self::Error> {
    match array {
      ArrayImpl::String(array) => Ok(array),
      other => Err(TypeMismatch(stringify!(String), other.identifier())),
    }
  }
}

#[doc = concat!("Implement [`", stringify!(StringArray), "`] -> [`ArrayImpl`]")]
impl From<StringArray> for ArrayImpl {
  fn from(array: StringArray) -> Self {
    ArrayImpl::String(array)
  }
}

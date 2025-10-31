use bitvec::vec::BitVec;

use crate::array::StringArray;

use super::ArrayBuilder;

pub struct StringArrayBuilder {
  /// The flattened data of string.
  data: Vec<u8>,

  /// Offsets of each string in the data flat array.
  offsets: Vec<usize>,

  /// The null bitmap of this array.
  bitmap: BitVec,
}

impl ArrayBuilder for StringArrayBuilder {
  type Array = StringArray;

  fn with_capacity(capacity: usize) -> Self {
    let mut offsets = Vec::with_capacity(capacity + 1);
    offsets.push(0);
    Self {
      data: Vec::with_capacity(capacity),
      bitmap: BitVec::with_capacity(capacity),
      offsets,
    }
  }

  fn push(&mut self, value: Option<&str>) {
    match value {
      Some(v) => {
        self.data.extend(v.as_bytes());
        self.offsets.push(self.data.len());
        self.bitmap.push(true);
      }
      None => {
        self.offsets.push(self.data.len());
        self.bitmap.push(false);
      }
    }
  }

  fn finish(self) -> Self::Array {
    StringArray::new(self.data, self.offsets, self.bitmap)
  }
}

use std::marker::PhantomData;

use crate::builder::ArrayBuilder;
use crate::scalar::Scalar;
use crate::{
  TypeMismatch,
  array::{Array, ArrayImpl},
};
use anyhow::Result;
use anyhow::anyhow;

mod cmp;
mod string;

pub use cmp::ExprCmpGe;
pub use cmp::ExprCmpLe;
pub use string::ExprStrContains;

use super::Expression;

/// A trait over all binary scalar functions, which takes `I1` and `I2` as input parameter, and
/// outputs array of type `O`.
pub trait BinaryExprFunc<I1: Array, I2: Array, O: Array> {
  fn eval<'a>(&self, i1: I1::RefItem<'a>, i2: I2::RefItem<'a>) -> O::Item;
}

pub struct BinaryExpression<I1: Array, I2: Array, O: Array, F> {
  expr: F,
  _phantom: PhantomData<(I1, I2, O)>,
}

//impl<'a, I1: Array, I2: Array, O: Array, F> BinaryExpression<I1, I2, O, F>
impl<I1: Array, I2: Array, O: Array, F> BinaryExpression<I1, I2, O, F>
where
  /*
    &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    F: Fn(I1::RefItem<'a>, I2::RefItem<'a>) -> O::Item,
  */
  for<'a> &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
  for<'a> &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
  F: BinaryExprFunc<I1, I2, O>,
{
  pub fn new(expr: F) -> Self {
    Self {
      expr,
      _phantom: PhantomData,
    }
  }

  /*
    pub fn eval(&self, i1: &'a ArrayImpl, i2: &'a ArrayImpl) -> Result<ArrayImpl> {
      let i1a: &'a I1 = i1.try_into()?;
      let i2a: &'a I2 = i2.try_into()?;
      assert_eq!(i1.len(), i2.len(), "array length mismatch");
      let mut builder: O::Builder = O::Builder::with_capacity(i1.len());
      for (i1, i2) in i1a.iter().zip(i2a.iter()) {
        match (i1, i2) {
          (Some(i1), Some(i2)) => builder.push(Some((self.func)(i1, i2).as_scalar_ref())),
          _ => builder.push(None),
        }
      }
      Ok(builder.finish().into())
    }
  */
  pub fn eval_batch(&self, i1: &ArrayImpl, i2: &ArrayImpl) -> Result<ArrayImpl> {
    let i1a: &I1 = i1.try_into()?;
    let i2a: &I2 = i2.try_into()?;
    assert_eq!(i1.len(), i2.len(), "array length mismatch");
    let mut builder: O::Builder = O::Builder::with_capacity(i1.len());
    for (i1, i2) in i1a.iter().zip(i2a.iter()) {
      match (i1, i2) {
        (Some(i1), Some(i2)) => builder.push(Some(self.expr.eval(i1, i2).as_scalar_ref())),
        _ => builder.push(None),
      }
    }
    Ok(builder.finish().into())
  }
}

impl<I1: Array, I2: Array, O: Array, F> Expression for BinaryExpression<I1, I2, O, F>
where
  for<'a> &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
  for<'a> &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
  F: BinaryExprFunc<I1, I2, O>,
{
  fn eval_expr(&self, data: &[&ArrayImpl]) -> Result<ArrayImpl> {
    if data.len() != 2 {
      return Err(anyhow!("Expect two inputs for BinaryExpression"));
    }
    self.eval_batch(data[0], data[1])
  }
}

#[cfg(test)]
mod tests {
  use crate::array::BoolArray;
  use crate::array::I32Array;
  use crate::array::I64Array;
  use crate::array::StringArray;

  use super::cmp::*;
  use super::string::*;
  use super::*;

  /// Test if an array has the same content as a vector
  fn check_array_eq<'a, A: Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>])
  where
    A::RefItem<'a>: PartialEq,
  {
    for (a, b) in array.iter().zip(vec.iter()) {
      assert_eq!(&a, b);
    }
  }

  #[test]
  fn test_cmp_le() {
    // Compare two `i32` array. Cast them to `i64` before comparing.
    let expr = BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
      ExprCmpLe::<_, _, I64Array>(PhantomData),
    );
    let result = expr
      .eval_batch(
        &I32Array::from_slice(&[Some(0), Some(1), None]).into(),
        &I32Array::from_slice(&[Some(1), Some(0), None]).into(),
      )
      .unwrap();
    check_array_eq::<BoolArray>(
      (&result).try_into().unwrap(),
      &[Some(true), Some(false), None],
    );
  }

  #[test]
  fn test_cmp_ge_str() {
    let expr =
      BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(
        ExprCmpGe::<_, _, StringArray>(PhantomData),
      );
    let result = expr
      .eval_batch(
        &StringArray::from_slice(&[Some("0"), Some("1"), None]).into(),
        &StringArray::from_slice(&[Some("1"), Some("0"), None]).into(),
      )
      .unwrap();
    check_array_eq::<BoolArray>(
      (&result).try_into().unwrap(),
      &[Some(false), Some(true), None],
    );
  }

  #[test]
  fn test_str_contains() {
    let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(ExprStrContains);
    let result = expr
      .eval_batch(
        &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
        &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
      )
      .unwrap();
    check_array_eq::<BoolArray>(
      (&result).try_into().unwrap(),
      &[Some(true), Some(false), None],
    );
  }
}

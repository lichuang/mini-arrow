#![allow(dead_code)]

use std::{cmp::Ordering, marker::PhantomData};

use crate::array::{Array, BoolArray};

use super::BinaryExprFunc;

/// Return if `i1 < i2`. Note that `i1` and `i2` could be different types. This
/// function will automatically cast them into `C` type.
///
/// * `I1`: left input type.
/// * `I2`: right input type.
/// * `C`: cast type.
pub struct ExprCmpLe<I1: Array, I2: Array, C: Array>(pub PhantomData<(I1, I2, C)>);

impl<I1: Array, I2: Array, C: Array> BinaryExprFunc<I1, I2, BoolArray> for ExprCmpLe<I2, I2, C>
where
  for<'a> I1::RefItem<'a>: Into<C::RefItem<'a>>,
  for<'a> I2::RefItem<'a>: Into<C::RefItem<'a>>,
  for<'a> C::RefItem<'a>: PartialOrd,
{
  fn eval<'a>(&self, i1: I1::RefItem<'a>, i2: I2::RefItem<'a>) -> bool {
    i1.into().partial_cmp(&i2.into()).unwrap() == Ordering::Less
  }
}

/// Return if `i1 > i2`. Note that `i1` and `i2` could be different types. This
/// function will automatically cast them into `C` type.
///
/// * `I1`: left input type.
/// * `I2`: right input type.
/// * `C`: cast type.
pub struct ExprCmpGe<I1: Array, I2: Array, C: Array>(pub PhantomData<(I1, I2, C)>);

impl<I1: Array, I2: Array, C: Array> BinaryExprFunc<I1, I2, BoolArray> for ExprCmpGe<I2, I2, C>
where
  for<'a> I1::RefItem<'a>: Into<C::RefItem<'a>>,
  for<'a> I2::RefItem<'a>: Into<C::RefItem<'a>>,
  for<'a> C::RefItem<'a>: PartialOrd,
{
  fn eval<'a>(&self, i1: I1::RefItem<'a>, i2: I2::RefItem<'a>) -> bool {
    i1.into().partial_cmp(&i2.into()).unwrap() == Ordering::Greater
  }
}

#![allow(dead_code)]

use crate::array::{BoolArray, StringArray};

use super::BinaryExprFunc;

/// Checks if `i1.contains(i2)` for two string inputs.
pub struct ExprStrContains;

impl BinaryExprFunc<StringArray, StringArray, BoolArray> for ExprStrContains {
  fn eval(&self, i1: &str, i2: &str) -> bool {
    i1.contains(i2)
  }
}

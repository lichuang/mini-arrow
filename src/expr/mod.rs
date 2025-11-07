use std::marker::PhantomData;

use crate::array::ArrayImpl;
use anyhow::Result;

mod binary;

/// A trait over all expressions -- unary, binary, etc.
pub trait Expression {
  fn eval_expr(&self, data: &[&ArrayImpl]) -> Result<ArrayImpl>;
}

/// All supported expression functions
pub enum ExpressionFunc {
  CmpLe,
  CmpGe,
  StrContains,
}

/// Build expression with runtime information.
pub fn build_binary_expression(f: ExpressionFunc) -> Box<dyn Expression> {
  use ExpressionFunc::*;

  use crate::array::*;
  use crate::expr::binary::*;

  match f {
    CmpLe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
      ExprCmpLe::<_, _, I32Array>(PhantomData),
    )),
    CmpGe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
      ExprCmpGe::<_, _, I32Array>(PhantomData),
    )),
    StrContains => {
      Box::new(BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(ExprStrContains))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::array::{Array, StringArray};
  use crate::scalar::ScalarRefImpl;

  #[test]
  fn test_build_str_contains() {
    let expr = build_binary_expression(ExpressionFunc::StrContains);

    for _ in 0..10 {
      let result = expr
        .eval_expr(&[
          &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
          &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
        ])
        .unwrap();
      assert_eq!(result.get(0).unwrap(), ScalarRefImpl::Bool(true));
      assert_eq!(result.get(1).unwrap(), ScalarRefImpl::Bool(false));
      assert!(result.get(2).is_none());
    }
  }
}

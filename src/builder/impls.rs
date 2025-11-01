use crate::TypeMismatch;
use crate::array::*;
use crate::builder::ArrayBuilder;
use crate::builder::ArrayBuilderImpl;
use crate::macros::for_all_variants;
use crate::scalar::*;

/// Implements dispatch functions for [`ArrayBuilder`]
macro_rules! impl_array_builder_dispatch {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl ArrayBuilderImpl {
            /// Appends an element to the back of array.
            pub fn push(&mut self, v: Option<ScalarRefImpl<'_>>) {
                match (self, v) {
                    $(
                        (Self::$Abc(a), Some(ScalarRefImpl::$Abc(v))) => a.push(Some(v)),
                        (Self::$Abc(a), None) => a.push(None),
                    )*
                    (a, Some(b)) => Err(TypeMismatch(a.identifier(), b.identifier())).unwrap(),
                }
            }

            /// Finish build and return a new array.
            pub fn finish(self) -> ArrayImpl {
                match self {
                    $(
                        Self::$Abc(a) => ArrayImpl::$Abc(a.finish()),
                    )*
                }
            }

            /// Get identifier of the current array builder
            pub fn identifier(&self) -> &'static str {
                match self {
                    $(
                        Self::$Abc(_) => stringify!($Abc),
                    )*
                }
            }
        }
    }
}

for_all_variants! { impl_array_builder_dispatch }

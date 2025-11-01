//! Contains all macro-generated implementations of array methods

use crate::TypeMismatch;
use crate::array::*;
use crate::builder::ArrayBuilderImpl;
use crate::builder::I32ArrayBuilder;
use crate::builder::StringArrayBuilder;
use crate::macros::for_all_variants;
use crate::scalar::*;

/// Implements dispatch functions for [`Array`]
macro_rules! impl_array_dispatch {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl ArrayImpl {
            /// Get the value at the given index.
            pub fn get(&self, idx: usize) -> Option<ScalarRefImpl<'_>> {
                match self {
                    $(
                        Self::$Abc(array) => array.get(idx).map(ScalarRefImpl::$Abc),
                    )*
                }
            }

            /// Number of items of array.
            pub fn len(&self) -> usize {
                match self {
                    $(
                        Self::$Abc(a) => a.len(),
                    )*
                }
            }

            /// Number of items of array.
            pub fn is_empty(&self) -> bool {
                match self {
                    $(
                        Self::$Abc(a) => a.is_empty(),
                    )*
                }
            }

            /// Get identifier of the current array
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

for_all_variants! { impl_array_dispatch }

/// Implements `TryFrom` and `From` for [`Array`].
macro_rules! impl_array_conversion {
    ([], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        $(
            #[doc = concat!("Implement [`", stringify!($AbcArray), "`] -> [`ArrayImpl`]")]
            impl From<$AbcArray> for ArrayImpl {
                fn from(array: $AbcArray) -> Self {
                    Self::$Abc(array)
                }
            }

            #[doc = concat!("Implement [`ArrayImpl`] -> [`", stringify!($AbcArray), "`]")]
            impl TryFrom<ArrayImpl> for $AbcArray {
                type Error = TypeMismatch;

                fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
                    match array {
                        ArrayImpl::$Abc(array) => Ok(array),
                        other => Err(TypeMismatch(stringify!($Abc), other.identifier())),
                    }
                }
            }

            #[doc = concat!("Implement reference of [`ArrayImpl`] -> [`", stringify!($AbcArray), "`]")]
            impl<'a> TryFrom<&'a ArrayImpl> for &'a $AbcArray {
                type Error = TypeMismatch;

                fn try_from(array: &'a ArrayImpl) -> Result<Self, Self::Error> {
                    match array {
                        ArrayImpl::$Abc(array) => Ok(array),
                        other => Err(TypeMismatch(stringify!($Abc), other.identifier())),
                    }
                }
            }

            #[doc = concat!("Implement [`", stringify!($AbcArrayBuilder), "`] -> [`ArrayBuilderImpl`]")]
            impl From<$AbcArrayBuilder> for ArrayBuilderImpl {
                fn from(builder: $AbcArrayBuilder) -> Self {
                    Self::$Abc(builder)
                }
            }

            #[doc = concat!("Implement [`ArrayBuilderImpl`] -> [`", stringify!($AbcArrayBuilder), "`]")]
            impl TryFrom<ArrayBuilderImpl> for $AbcArrayBuilder {
                type Error = TypeMismatch;

                fn try_from(builder: ArrayBuilderImpl) -> Result<Self, Self::Error> {
                    match builder {
                        ArrayBuilderImpl::$Abc(builder) => Ok(builder),
                        other => Err(TypeMismatch(stringify!($Abc), other.identifier())),
                    }
                }
            }

            #[doc = concat!("Implement mut ref of [`ArrayBuilderImpl`] -> [`", stringify!($AbcArrayBuilder), "`]")]
            impl<'a> TryFrom<&'a mut ArrayBuilderImpl> for &'a mut $AbcArrayBuilder {
                type Error = TypeMismatch;

                fn try_from(builder: &'a mut ArrayBuilderImpl) -> Result<Self, Self::Error> {
                    match builder {
                        ArrayBuilderImpl::$Abc(builder) => Ok(builder),
                        other => Err(TypeMismatch(stringify!($Abc), other.identifier())),
                    }
                }
            }
        )*
    };
}

for_all_variants! { impl_array_conversion }

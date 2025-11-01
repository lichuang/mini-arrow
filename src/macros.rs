//! Necessary macros to cover variants of array types.

/// `for_all_variants` includes all variants of our array types. If you added a new array
/// type inside the project, be sure to add a variant here.
///
/// Every tuple has four elements, where
/// `{ enum variant name, function suffix name, array type, builder type, scalar type }`

macro_rules! for_all_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Int32, int32, I32Array, I32ArrayBuilder, i32, i32 },
            { String, string, StringArray, StringArrayBuilder, String, &'a str }
        }
    };
}

pub(crate) use for_all_variants;

macro_rules! for_all_primitive_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Int32, int32, I32Array, I32ArrayBuilder, i32, i32 }
        }
    };
}
pub(crate) use for_all_primitive_variants;

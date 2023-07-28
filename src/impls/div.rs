use crate::Div;

macro_rules! div_impl_integer {
    ($(($($t:ty)*) => $panic:expr),*) => ($($(
        /// This operation rounds towards zero, truncating any
        /// fractional part of the exact result.
        ///
        /// # Panics
        ///
        #[doc = $panic]
        impl const Div for $t {
            type Output = $t;

            #[inline]
            fn div(self, other: $t) -> $t { self / other }
        }
    )*)*)
}

div_impl_integer! {
    (usize u8 u16 u32 u64 u128) => "This operation will panic if `other == 0`.",
    (isize i8 i16 i32 i64 i128) => "This operation will panic if `other == 0` or the division results in overflow."
}

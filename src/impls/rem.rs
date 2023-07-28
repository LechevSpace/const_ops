use crate::Rem;

macro_rules! rem_impl_integer {
    ($(($($t:ty)*) => $panic:expr),*) => ($($(
        /// This operation satisfies `n % d == n - (n / d) * d`. The
        /// result has the same sign as the left operand.
        ///
        /// # Panics
        ///
        #[doc = $panic]
        impl const Rem for $t {
            type Output = $t;

            #[inline]
            fn rem(self, other: $t) -> $t { self % other }
        }
    )*)*)
}

rem_impl_integer! {
    (usize u8 u16 u32 u64 u128) => "This operation will panic if `other == 0`.",
    (isize i8 i16 i32 i64 i128) => "This operation will panic if `other == 0` or if `self / other` results in overflow."
}

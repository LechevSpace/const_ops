use crate::BitXor;

macro_rules! bitxor_impl {
    ($($t:ty)*) => ($(
        impl const BitXor for $t {
            type Output = $t;

            #[inline]
            fn bitxor(self, other: $t) -> $t { self ^ other }
        }
    )*)
}

bitxor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

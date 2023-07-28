use crate::BitAnd;

macro_rules! bitand_impl {
    ($($t:ty)*) => ($(
        impl const BitAnd for $t {
            type Output = $t;

            #[inline]
            fn bitand(self, rhs: $t) -> $t { self & rhs }
        }
    )*)
}

bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

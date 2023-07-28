use crate::BitOr;

macro_rules! bitor_impl {
    ($($t:ty)*) => ($(
        impl const BitOr for $t {
            type Output = $t;

            #[inline]
            fn bitor(self, rhs: $t) -> $t { self | rhs }
        }
    )*)
}

bitor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

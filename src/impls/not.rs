use crate::Not;

macro_rules! not_impl {
    ($($t:ty)*) => ($(
        impl const Not for $t {
            type Output = $t;

            #[inline]
            fn not(self) -> $t { !self }
        }
    )*)
}

not_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

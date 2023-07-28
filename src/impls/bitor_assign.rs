use crate::BitOrAssign;

macro_rules! bitor_assign_impl {
    ($($t:ty)+) => ($(
        impl const BitOrAssign for $t {
            #[inline]
            fn bitor_assign(&mut self, other: $t) { *self |= other }
        }
    )+)
}

bitor_assign_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

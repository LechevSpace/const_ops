use crate::BitXorAssign;

macro_rules! bitxor_assign_impl {
    ($($t:ty)+) => ($(
        impl const BitXorAssign for $t {
            #[inline]
            fn bitxor_assign(&mut self, other: $t) { *self ^= other }
        }
    )+)
}

bitxor_assign_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

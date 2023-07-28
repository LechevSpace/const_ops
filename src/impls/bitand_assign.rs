use crate::BitAndAssign;

macro_rules! bitand_assign_impl {
    ($($t:ty)+) => ($(
        impl const BitAndAssign for $t {
            #[inline]
            fn bitand_assign(&mut self, other: $t) { *self &= other }
        }

    )+)
}

bitand_assign_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

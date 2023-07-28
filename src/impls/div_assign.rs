use crate::DivAssign;

macro_rules! div_assign_impl {
    ($($t:ty)+) => ($(
        impl const DivAssign for $t {
            #[inline]
            fn div_assign(&mut self, other: $t) { *self /= other }
        }
    )+)
}

div_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

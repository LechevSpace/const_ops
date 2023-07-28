use crate::MulAssign;

macro_rules! mul_assign_impl {
    ($($t:ty)+) => ($(
        impl const MulAssign for $t {
            #[inline]
            fn mul_assign(&mut self, other: $t) { *self *= other }
        }
    )+)
}

mul_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

use crate::RemAssign;

macro_rules! rem_assign_impl {
    ($($t:ty)+) => ($(
        impl const RemAssign for $t {
            #[inline]
            fn rem_assign(&mut self, other: $t) { *self %= other }
        }
    )+)
}

rem_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

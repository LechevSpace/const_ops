use crate::SubAssign;

macro_rules! sub_assign_impl {
    ($($t:ty)+) => ($(
        impl const SubAssign for $t {
            #[inline]
            fn sub_assign(&mut self, other: $t) { *self -= other }
        }
    )+)
}

sub_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

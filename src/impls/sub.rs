use crate::Sub;

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl const Sub for $t {
            type Output = $t;

            #[inline]
            fn sub(self, other: $t) -> $t { self - other }
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

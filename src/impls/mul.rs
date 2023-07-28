use crate::Mul;

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl const Mul for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: $t) -> $t { self * other }
        }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

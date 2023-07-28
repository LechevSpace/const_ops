use crate::Neg;

macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl const Neg for $t {
            type Output = $t;

            #[inline]
            fn neg(self) -> $t { -self }
        }
    )*)
}

neg_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

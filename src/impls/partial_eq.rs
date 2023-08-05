use crate::PartialEq;

macro_rules! partialeq_impl {
    ($($t:ty)*) => ($(
        impl const PartialEq for $t {

            #[inline]
            fn eq(&self, rhs: &$t) -> bool { *self == *rhs }
        }
    )*)
}

partialeq_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

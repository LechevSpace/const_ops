use crate::ShrAssign;

macro_rules! shr_assign_impl {
    ($t:ty, $f:ty) => {
        impl const ShrAssign<$f> for $t {
            #[inline]
            fn shr_assign(&mut self, other: $f) {
                *self >>= other
            }
        }
    };
}

macro_rules! shr_assign_impl_all {
    ($($t:ty)*) => ($(
        shr_assign_impl! { $t, u8 }
        shr_assign_impl! { $t, u16 }
        shr_assign_impl! { $t, u32 }
        shr_assign_impl! { $t, u64 }
        shr_assign_impl! { $t, u128 }
        shr_assign_impl! { $t, usize }

        shr_assign_impl! { $t, i8 }
        shr_assign_impl! { $t, i16 }
        shr_assign_impl! { $t, i32 }
        shr_assign_impl! { $t, i64 }
        shr_assign_impl! { $t, i128 }
        shr_assign_impl! { $t, isize }
    )*)
}

shr_assign_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

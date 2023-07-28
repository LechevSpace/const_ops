use crate::ShlAssign;

macro_rules! shl_assign_impl {
    ($t:ty, $f:ty) => {
        impl const ShlAssign<$f> for $t {
            #[inline]
            fn shl_assign(&mut self, other: $f) {
                *self <<= other
            }
        }
    };
}

macro_rules! shl_assign_impl_all {
    ($($t:ty)*) => ($(
        shl_assign_impl! { $t, u8 }
        shl_assign_impl! { $t, u16 }
        shl_assign_impl! { $t, u32 }
        shl_assign_impl! { $t, u64 }
        shl_assign_impl! { $t, u128 }
        shl_assign_impl! { $t, usize }

        shl_assign_impl! { $t, i8 }
        shl_assign_impl! { $t, i16 }
        shl_assign_impl! { $t, i32 }
        shl_assign_impl! { $t, i64 }
        shl_assign_impl! { $t, i128 }
        shl_assign_impl! { $t, isize }
    )*)
}

shl_assign_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

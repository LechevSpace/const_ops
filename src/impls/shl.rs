use crate::Shl;

macro_rules! shl_impl {
    ($t:ty, $f:ty) => {
        impl const Shl<$f> for $t {
            type Output = $t;

            #[inline]
            fn shl(self, other: $f) -> $t {
                self << other
            }
        }
    };
}
macro_rules! shl_impl_all {
    ($($t:ty)*) => ($(
        shl_impl! { $t, u8 }
        shl_impl! { $t, u16 }
        shl_impl! { $t, u32 }
        shl_impl! { $t, u64 }
        shl_impl! { $t, u128 }
        shl_impl! { $t, usize }

        shl_impl! { $t, i8 }
        shl_impl! { $t, i16 }
        shl_impl! { $t, i32 }
        shl_impl! { $t, i64 }
        shl_impl! { $t, i128 }
        shl_impl! { $t, isize }
    )*)
}

shl_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 isize i128 }

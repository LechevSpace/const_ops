use crate::AddAssign;

macro_rules! add_assign_impl {
    ($($t:ty)+) => ($(
        impl const AddAssign for $t {
            #[inline]
            fn add_assign(&mut self, other: $t) { *self += other }
        }

        //forward_ref_op_assign! { impl AddAssign, add_assign for $t, $t }
    )+)
}

add_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

use crate::complex::Complex;

pub(crate) trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($type:ty, $value:expr) => {
        impl One for $type {
            fn one() -> Self {
                $value
            }
        }
    };
}

impl_one!(f64, 1.0);
impl_one!(f32, 1.0);
impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(i128, 1);
impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(u128, 1);
impl_one!(Complex, Complex::new(1.0, 0.0));

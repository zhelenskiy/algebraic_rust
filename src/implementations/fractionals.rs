use num::{Complex, Integer, One, Num};
use num::rational::Ratio;
use crate::implementations::helpers::multi::AssociativeCommutativeMulti;
use crate::structures::basics::Invertible;
use crate::structures::ring_like::Field;


macro_rules! impls {
    ($t:ty, $one:expr, $pt:tt, $($bound:tt)+) => {
impl<$pt: Clone + $($bound)+> Invertible<$t> for AssociativeCommutativeMulti<$t> {
    fn inverse(x: $t) -> $t { $one / x }
}
    };
    ($t:ty, $one:expr) => {
impl Invertible<$t> for AssociativeCommutativeMulti<$t> {
    fn inverse(x: $t) -> $t { $one / x }
}
    };
}

impls!(f32, 1.0);
impls!(f64, 1.0);
impls!(Ratio<T>, Ratio::<T>::one(), T, Integer);
impls!(Complex<T>, Complex::<T>::one(), T, Num + Field);

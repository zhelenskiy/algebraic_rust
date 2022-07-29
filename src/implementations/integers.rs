use num::{BigInt, BigUint, Complex, Integer, Num, One, Zero};
use num::rational::Ratio;
use crate::implementations::helpers::sum::AssociativeCommutativeSum;
use crate::implementations::helpers::multi::AssociativeCommutativeMulti;
use crate::structures::basics::WithIdentityElement;
use crate::structures::common_ring_like::Semiring;

macro_rules! impls {
    ($t:ty, $zero:expr, $one:expr, $pt:tt, $bound:tt) => {
impl<$pt: Clone + $bound> WithIdentityElement<$t> for AssociativeCommutativeSum<$t> {
    fn identity() -> $t { $zero }
}

impl<$pt: Clone + $bound> WithIdentityElement<$t> for AssociativeCommutativeMulti<$t> {
    fn identity() -> $t { $one }
}

impl<$pt: Clone + $bound> Semiring for $t {
    type Sum = AssociativeCommutativeSum<$t>;
    type Multi = AssociativeCommutativeMulti<$t>;
}
    };
    ($t:ty, $zero:expr, $one:expr) => {
impl WithIdentityElement<$t> for AssociativeCommutativeSum<$t> {
    fn identity() -> $t { $zero }
}

impl WithIdentityElement<$t> for AssociativeCommutativeMulti<$t> {
    fn identity() -> $t { $one }
}

impl Semiring for $t {
    type Sum = AssociativeCommutativeSum<$t>;
    type Multi = AssociativeCommutativeMulti<$t>;
}
    };
}

impls!(i8, 0, 1);
impls!(u8, 0, 1);
impls!(i16, 0, 1);
impls!(u16, 0, 1);
impls!(i32, 0, 1);
impls!(u32, 0, 1);
impls!(i64, 0, 1);
impls!(u64, 0, 1);
#[cfg(has_i128)]
impls!(i128, 0, 1);
#[cfg(has_i128)]
impls!(u128, 0, 1);
impls!(isize, 0, 1);
impls!(usize, 0, 1);
impls!(f32, 0.0, 1.0);
impls!(f64, 0.0, 1.0);
impls!(BigInt, BigInt::zero(), BigInt::one());
impls!(BigUint, BigUint::zero(), BigUint::one());
impls!(Complex<T>, Complex::<T>::zero(), Complex::<T>::one(), T, Num);
impls!(Ratio<T>, Ratio::<T>::zero(), Ratio::<T>::one(), T, Integer);


fn f() {
    BigInt::one();
    Complex::<i32>::one();
}

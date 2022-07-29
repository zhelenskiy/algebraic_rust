use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Mul, Neg, Sub};
pub use finite::FiniteMatrix;
use Matrix::Finite;
use crate::matrix::Matrix::InfiniteDiagonal;

pub mod finite;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Matrix<T> {
    InfiniteDiagonal(T),
    Finite(FiniteMatrix<T>),
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InfiniteDiagonal(value) => writeln!(f, "Diagonal {{{value}}}"),
            Finite(regular) => regular.fmt(f)
        }
    }
}

macro_rules! impl_standard {
    ($trait:tt, $function:tt, $operator:tt) => {

impl<T: $trait<T, Output=T> + Clone> $trait for Matrix<T> {
    type Output = Matrix<T>;

    fn $function(self, rhs: Self) -> Self::Output {
        match self {
            InfiniteDiagonal(d1) => match rhs {
                InfiniteDiagonal(d2) => InfiniteDiagonal(d1 $operator d2),
                Finite(m2) => {
                    let m = m2.map_with_indexes(|(x, y), item| if x == y { d1.clone() $operator item } else { item });
                    Finite(m)
                }
            }
            Finite(m1) => match rhs {
                InfiniteDiagonal(d2) => {
                    let m = m1.map_with_indexes(|(x, y), item| if x == y { item $operator d2.clone() } else { item });
                    Finite(m)
                }
                Finite(m2) => Finite(m1 $operator m2)
            }
        }
    }
}};
}

impl<T: Neg<Output=T>> Neg for Matrix<T> {
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        match self {
            InfiniteDiagonal(d) => InfiniteDiagonal(-d),
            Finite(m) => Finite(-m)
        }
    }
}

impl_standard!(Add, add, +);
impl_standard!(Sub, sub, -);

impl<T: Mul<Output=T> + Clone + Sum> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            InfiniteDiagonal(d1) => match rhs {
                InfiniteDiagonal(d2) => InfiniteDiagonal(d1 * d2),
                Finite(m2) => Finite(m2.map(|x| d1.clone() * x)),
            }
            Finite(m1) => match rhs {
                InfiniteDiagonal(d2) => Finite(m1.map(|x| x * d2.clone())),
                Finite(m2) => Finite(m1 * m2),
            }
        }
    }
}

// todo!("ring, etc")
// todo!("fibonacci")
// todo!("macroses")
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Index, Mul, Neg, Sub};
pub use finite::FiniteMatrix;
use Matrix::Finite;
use crate::matrix::Matrix::InfiniteDiagonal;
use crate::matrix::finite::Shape;
use crate::structures::ring_like::{Semiring, zero};

pub mod finite;


pub fn matrix<T>(height: usize, width: usize, flat_data: Vec<T>) -> Matrix<T> {
    Finite(FiniteMatrix::from_iter(Shape { height, width }, flat_data.into_iter()))
}

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

impl <T: Semiring + Clone> Matrix<T> {
    pub fn get(&self, index: (usize, usize)) -> T {
        match &self {
            &InfiniteDiagonal(d) => if index.0 == index.1 { d.to_owned() } else { zero() },
            &Finite(m) => m.index(index).to_owned()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::fibonacci::fibonacci;
    use crate::matrix::matrix;
    use crate::structures::ring_like::{minus, multi, plus, unary_minus};

    #[test]
    fn test_multiplication() {
        let matrix1 = matrix(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let matrix2 = matrix(2, 3, vec![7, 8, 9, 10, 11, 12]);
        let correct_data = vec![27, 30, 33, 61, 68, 75, 95, 106, 117];
        let correct_data = matrix(3, 3, correct_data);
        assert_eq!(matrix1.clone() * matrix2.clone(), correct_data);
        assert_eq!(multi(matrix1, matrix2), correct_data);
    }

    #[test]
    fn test_addition() {
        let matrix1 = matrix(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let matrix2 = matrix(3, 2, vec![7, 8, 9, 10, 11, 12]);
        let correct_data = vec![8, 10, 12, 14, 16, 18];
        let correct_data = matrix(3, 2, correct_data);
        assert_eq!(matrix1.clone() + matrix2.clone(), correct_data);
        assert_eq!(plus(matrix1, matrix2), correct_data);
    }

    #[test]
    fn test_negation() {
        let matrix1 = matrix(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let correct_data = vec![-1, -2, -3, -4, -5, -6];
        let correct_data = matrix(3, 2, correct_data);
        assert_eq!(-matrix1.clone(), correct_data);
        assert_eq!(unary_minus(matrix1), correct_data);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = matrix(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let matrix2 = matrix(3, 2, vec![7, 8, 9, 10, 11, 12]);
        let correct_data = vec![-6, -6, -6, -6, -6, -6];
        let correct_data = matrix(3, 2, correct_data);
        assert_eq!(matrix1.clone() - matrix2.clone(), correct_data);
        assert_eq!(minus(matrix1, matrix2), correct_data);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci::<i32>(0), 0);
        assert_eq!(fibonacci::<i32>(1), 1);
        assert_eq!(fibonacci::<i32>(2), 1);
        assert_eq!(fibonacci::<i32>(3), 2);
        assert_eq!(fibonacci::<i32>(4), 3);
        assert_eq!(fibonacci::<i32>(5), 5);
        assert_eq!(fibonacci::<i32>(6), 8);
    }
}

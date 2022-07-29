use std::fmt::Display;
use crate::matrix::{FiniteMatrix, Matrix};
use crate::matrix::finite::Shape;
use crate::matrix::Matrix::Finite;
use crate::power::unsigned::pow;
use crate::structures::ring_like::{multi, one, RingWithOne, zero};

pub fn fibonacci<T: RingWithOne + Clone + Display>(index: u64) -> T {
    let base: Matrix<T> = Finite(FiniteMatrix::from_iter(
        Shape { height: 1, width: 2 },
        vec![zero(), one()].into_iter()
    ));
    let step = Finite(FiniteMatrix::from_iter(
        Shape { height: 2, width: 2 },
        vec![zero(), one(), one(), one()].into_iter()
    ));
    let multiplier: Matrix<T> = pow(step, index);
    let matrix = multi(base, multiplier);
    matrix.get((0, 0)).to_owned()
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
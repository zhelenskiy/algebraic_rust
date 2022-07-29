use std::marker::PhantomData;
use crate::matrix::{FiniteMatrix, Matrix};
use crate::matrix::Matrix::{Finite, InfiniteDiagonal};
use crate::structures::basics::{Associative, Commutative, Invertible, Magma, WithIdentityElement};
use crate::structures::ring_like::{multi, one, plus, Semiring, unary_minus, zero};
use super::finite_matrix::{FiniteMatrixSum, FiniteMatrixMul};

pub struct MatrixSum<T: Semiring>(PhantomData<T>);

impl<T: Semiring> Commutative for MatrixSum<T> where <T as Semiring>::Sum: Commutative {}

impl<T: Semiring> Associative for MatrixSum<T> where <T as Semiring>::Sum: Associative {}

impl<T: Semiring> WithIdentityElement<Matrix<T>> for MatrixSum<T> {
    fn identity() -> Matrix<T> {
        InfiniteDiagonal(zero())
    }
}

impl<T: Semiring + Clone> Magma<Matrix<T>> for MatrixSum<T> {
    fn operation(operand1: Matrix<T>, operand2: Matrix<T>) -> Matrix<T> {
        match operand1 {
            InfiniteDiagonal(d1) => match operand2 {
                InfiniteDiagonal(d2) => InfiniteDiagonal(plus(d1, d2)),
                Finite(m2) => Finite(m2.map_with_indexes(|(x, y), item|
                    if x == y { plus(d1.clone(), item) } else { item }
                ))
            }
            Finite(m1) => match operand2 {
                InfiniteDiagonal(d2) => Finite(m1.map_with_indexes(|(x, y), item|
                    if x == y { plus(item, d2.clone()) } else { item }
                )),
                Finite(m2) => Finite(FiniteMatrixSum::operation(m1, m2))
            }
        }
    }
}

impl<T: Semiring> Invertible<Matrix<T>> for MatrixSum<T> where T::Sum: Invertible<T> {
    fn inverse(operand: Matrix<T>) -> Matrix<T> {
        match operand {
            InfiniteDiagonal(d) => InfiniteDiagonal(unary_minus(d)),
            Finite(m) => Finite(FiniteMatrixSum::inverse(m))
        }
    }
}


pub struct MatrixMul<T: Semiring>(PhantomData<T>);

impl<T: Semiring + Clone> Semiring for Matrix<T> {
    type Sum = MatrixSum<T>;
    type Multi = MatrixMul<T>;
}

impl<T: Semiring> WithIdentityElement<Matrix<T>> for MatrixMul<T> where T::Multi: WithIdentityElement<T> {
    fn identity() -> Matrix<T> {
        InfiniteDiagonal(one())
    }
}

impl<T: Semiring> Associative for MatrixMul<T> where <T as Semiring>::Multi: Associative {}

impl<T: Semiring + Clone> Magma<Matrix<T>> for MatrixMul<T> {
    fn operation(operand1: Matrix<T>, operand2: Matrix<T>) -> Matrix<T> {
        match operand1 {
            InfiniteDiagonal(d1) => match operand2 {
                InfiniteDiagonal(d2) => InfiniteDiagonal(multi(d1, d2)),
                Finite(m2) => Finite(m2.map(|x| multi(d1.clone(), x)))
            }
            Finite(m1) => match operand2 {
                InfiniteDiagonal(d2) => Finite(m1.map(|x| multi(x, d2.clone()))),
                Finite(m2) => Finite(FiniteMatrixMul::operation(m1, m2))
            }
        }
    }
}

use std::marker::PhantomData;
use crate::matrix::finite::Shape;
use crate::structures::basics::{Associative, Commutative, Invertible, Magma};
use crate::structures::ring_like::{multi, plus, Semiring, unary_minus};
pub use crate::matrix::FiniteMatrix;

pub struct FiniteMatrixSum<T: Semiring>(PhantomData<T>);

impl<T: Semiring> Commutative for FiniteMatrixSum<T> where <T as Semiring>::Sum: Commutative {}

impl<T: Semiring> Associative for FiniteMatrixSum<T> where <T as Semiring>::Sum: Associative {}

impl<T: Semiring> Magma<FiniteMatrix<T>> for FiniteMatrixSum<T> {
    fn operation(operand1: FiniteMatrix<T>, operand2: FiniteMatrix<T>) -> FiniteMatrix<T> {
        let shape = operand1.shape();
        assert_eq!(shape, operand2.shape());
        FiniteMatrix::from_iter(shape, operand1.into_iter().zip(operand2.into_iter()).map(|(x, y)| plus(x, y)))
    }
}

impl<T: Semiring> Invertible<FiniteMatrix<T>> for FiniteMatrixSum<T> where <T as Semiring>::Sum: Invertible<T> {
    fn inverse(operand: FiniteMatrix<T>) -> FiniteMatrix<T> {
        operand.map(unary_minus)
    }
}


pub struct FiniteMatrixMul<T: Semiring>(PhantomData<T>);

impl<T: Semiring> Associative for FiniteMatrixMul<T> where <T as Semiring>::Multi: Associative {}

impl<T: Semiring + Clone> Magma<FiniteMatrix<T>> for FiniteMatrixMul<T> {
    fn operation(operand1: FiniteMatrix<T>, operand2: FiniteMatrix<T>) -> FiniteMatrix<T> {
        let k_max = operand1.width();
        assert_eq!(k_max, operand2.height());
        let shape = Shape { height: operand1.height(), width: operand2.width() };
        FiniteMatrix::from_indexed_generator(shape, |x, y|
            (0..k_max).map(|k|
                multi(operand1[(x, k)].clone(), operand2[(k, y)].clone())
            ).reduce(plus).unwrap(),
        )
    }
}

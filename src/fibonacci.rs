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

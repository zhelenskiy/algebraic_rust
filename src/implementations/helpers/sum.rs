use std::marker::PhantomData;
use std::ops::{Add, Neg};
use crate::structures::basics::{Associative, Commutative, Invertible, Magma};

pub struct AssociativeCommutativeSum<T>(PhantomData<T>);

impl<T: Add<T, Output=T>> Magma<T> for AssociativeCommutativeSum<T> {
    fn operation(operand1: T, operand2: T) -> T { operand1 + operand2 }
}

impl<T: Add<Output=T>> Associative for AssociativeCommutativeSum<T> {}

impl<T: Add<Output=T>> Commutative for AssociativeCommutativeSum<T> {}

impl<T: Add<Output=T> + Neg<Output=T>> Invertible<T> for AssociativeCommutativeSum<T> {
    fn inverse(operand: T) -> T { -operand }
}



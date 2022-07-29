use std::marker::PhantomData;
use std::ops::Mul;
use crate::structures::basics::{Associative, Commutative, Magma};

pub struct AssociativeCommutativeMulti<T>(PhantomData<T>);

impl<T: Mul<T, Output=T>> Magma<T> for AssociativeCommutativeMulti<T> {
    fn operation(operand1: T, operand2: T) -> T { operand1 * operand2 }
}

impl<T: Mul<Output=T>> Associative for AssociativeCommutativeMulti<T> {}

impl<T: Mul<Output=T>> Commutative for AssociativeCommutativeMulti<T> {}

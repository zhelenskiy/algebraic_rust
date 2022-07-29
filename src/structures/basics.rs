
pub trait Magma<T> {
    fn operation(operand1: T, operand2: T) -> T;
}

pub trait Associative {}
pub trait Commutative {}

pub trait Invertible<T> {
    fn inverse(operand: T) -> T;
}

pub trait WithIdentityElement<T> {
    fn identity() -> T;
}
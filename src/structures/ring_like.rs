use super::group_like::*;

pub trait Semiring {
    type Sum: CommutativeMonoid<Self>;
    type Multi: Semigroup<Self>;
}

pub fn zero<T: Semiring>() -> T { T::Sum::identity() }

pub fn plus<T: Semiring>(operand1: T, operand2: T) -> T { T::Sum::operation(operand1, operand2) }

pub fn multi<T: Semiring>(operand1: T, operand2: T) -> T { T::Multi::operation(operand1, operand2) }

pub trait SemiringWithOne = Semiring<Multi: Monoid<Self>>;

pub trait CommutativeSemiring = Semiring<Multi: CommutativeSemigroup<Self>>;

pub trait CommutativeSemiringWithOne = Semiring<Multi: CommutativeMonoid<Self>>;


pub trait Ring = Semiring<Sum: AbelGroup<Self>>;

pub fn unary_minus<T: Ring>(operand: T) -> T { T::Sum::inverse(operand) }

pub fn minus<T: Ring>(operand1: T, operand2: T) -> T {
    plus(operand1, unary_minus(operand2))
}


pub trait RingWithOne = Ring<Multi: Monoid<Self>>;

pub fn one<T: RingWithOne>() -> T { T::Multi::identity() }

pub trait CommutativeRing = Ring<Multi: CommutativeSemigroup<Self>>;

pub trait CommutativeRingWithOne = Ring<Multi: CommutativeMonoid<Self>>;


// dividing by 0 is undefined and implementation defined
pub trait Field = CommutativeRingWithOne<Multi: AbelGroup<Self>>;

pub fn reciprocal<T: Field>(operand: T) -> T { T::Multi::inverse(operand) }

pub fn div<T: Field>(operand1: T, operand2: T) -> T {
    multi(operand1, reciprocal(operand2))
}

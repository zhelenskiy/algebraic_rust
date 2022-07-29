use crate::structures::basics::{Associative, Commutative, Invertible, Magma, WithIdentityElement};


pub trait CommutativeMagma<T> = Magma<T> + Commutative;

pub trait Semigroup<T> = Magma<T> + Associative;

pub trait Quasigroup<T> = Magma<T> + Invertible<T>;

pub trait UnitaryMagma<T> = Magma<T> + WithIdentityElement<T>;


pub trait CommutativeSemigroup<T> = Magma<T> + Associative + Commutative;

pub trait CommutativeQuasigroup<T> = Magma<T> + Invertible<T> + Commutative;

pub trait CommutativeUnitaryMagma<T> = Magma<T> + WithIdentityElement<T> + Commutative;

pub trait Loop<T> = Magma<T> + Invertible<T> + WithIdentityElement<T>;

pub trait InverseSemigroup<T> = Magma<T> + Invertible<T> + Associative;

pub trait Monoid<T> = Magma<T> + WithIdentityElement<T> + Associative;


pub trait CommutativeLoop<T> = Magma<T> + Invertible<T> + WithIdentityElement<T> + Commutative;

pub trait CommutativeInverseSemigroup<T> = Magma<T> + Invertible<T> + Associative + Commutative;

pub trait CommutativeMonoid<T> = Magma<T> + WithIdentityElement<T> + Associative + Commutative;

pub trait Group<T> = Magma<T> + WithIdentityElement<T> + Invertible<T> + Associative;


pub trait CommutativeGroup<T> = Magma<T> + WithIdentityElement<T> + Invertible<T> + Associative + Commutative;

pub trait AbelGroup<T> = CommutativeGroup<T>;

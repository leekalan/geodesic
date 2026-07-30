use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

mod float;

use crate::vector_space::VectorSpace;

pub trait Field:
    Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + DivAssign
    + MulAssign
{
    /// Additive identity.
    fn zero() -> Self;

    /// Multiplicative identity.
    fn one() -> Self;

    /// Additive inverse.
    fn neg(self) -> Self {
        -self
    }

    /// Multiplicative inverse.
    fn inv(self) -> Self {
        Self::one() / self
    }

    /// Scale a vector by a scalar.
    fn scale<V: VectorSpace<Scalar = Self>>(self, vector: V) -> V {
        vector * self
    }
}

pub trait Rootable: Field {
    fn sqrt(self) -> Self;
}

pub trait Summable: Sum {}

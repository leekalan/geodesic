use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::field::Field;

pub trait VectorSpace:
    Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<Self::Scalar, Output = Self>
    + Div<Self::Scalar, Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign<Self::Scalar>
    + DivAssign<Self::Scalar>
{
    type Scalar: Field;

    /// Additive identity.
    fn zero() -> Self;

    /// Multiplicative identity.
    fn one() -> Self::Scalar {
        Self::Scalar::one()
    }

    /// Additive inverse.
    fn neg(self) -> Self {
        -self
    }

    /// Multiplicative inverse.
    fn inv(scalar: Self::Scalar) -> Self::Scalar {
        scalar.inv()
    }
}

pub trait FiniteVectorSpace: VectorSpace {
    fn dim() -> usize;
}

pub trait IterableVectorSpace: VectorSpace {
    fn iterate<'a>(&'a self) -> impl Iterator<Item = &'a Self::Scalar> + 'a
    where
        Self::Scalar: 'a;
}

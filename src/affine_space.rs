use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{field::Field, vector_space::VectorSpace};

pub trait AffineSpace:
    Clone
    + Add<Self::Vector, Output = Self>
    + Sub<Self::Vector, Output = Self>
    + Sub<Output = Self::Vector>
    + AddAssign<Self::Vector>
    + SubAssign<Self::Vector>
{
    type Vector: VectorSpace<Scalar = Self::Scalar>;
    type Scalar: Field;

    /// The mechanical origin (not mathematically pure)
    fn origin() -> Self;
}

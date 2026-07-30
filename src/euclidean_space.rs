use crate::{affine_space::AffineSpace, field::Rootable, inner_product::InnerProductSpace};

pub trait EuclidianSpace: AffineSpace<Vector = Self::InnerProductSpace> {
    type InnerProductSpace: InnerProductSpace<Scalar = Self::Scalar>;

    fn distance_squared(lhs: Self, rhs: Self) -> Self::Scalar {
        Self::Vector::norm_squared(&(lhs - rhs))
    }

    fn distance(lhs: Self, rhs: Self) -> Self::Scalar
    where
        Self::Scalar: Rootable,
    {
        Self::Vector::norm(&(lhs - rhs))
    }
}

impl<A: AffineSpace> EuclidianSpace for A
where
    A::Vector: InnerProductSpace<Scalar = A::Scalar>,
{
    type InnerProductSpace = A::Vector;
}

fn __compiler_test_1<E: EuclidianSpace>(input: E::Vector) -> E::InnerProductSpace {
    input
}
fn __compiler_test_2<E: EuclidianSpace>(input: E::InnerProductSpace) -> E::Vector {
    input
}

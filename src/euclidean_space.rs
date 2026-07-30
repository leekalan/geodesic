use crate::{
    affine_space::AffineSpace,
    field::{Field, Rootable},
    inner_product::InnerProductSpace,
    vector_space::VectorSpace,
};

mod space;

pub use space::{Space, Space1, Space2, Space3, Space4};

pub trait EuclidianSpace<IP: InnerProductSpace<Self::Scalar, Vector = Self::Vector>> {
    type Point: AffineSpace<Vector = Self::Vector, Scalar = Self::Scalar>;
    type Vector: VectorSpace<Scalar = Self::Scalar>;
    type Scalar: Field;

    fn distance_squared(lhs: Self::Point, rhs: Self::Point) -> Self::Scalar {
        IP::norm_squared(&(lhs - rhs))
    }

    fn distance(lhs: Self::Point, rhs: Self::Point) -> Self::Scalar
    where
        Self::Scalar: Rootable,
    {
        IP::norm(&(lhs - rhs))
    }
}

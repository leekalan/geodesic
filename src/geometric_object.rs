use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    affine_space::AffineSpace, euclidean_space::EuclidianSpace, field::Field,
    inner_product::InnerProductSpace, vector_space::VectorSpace,
};

mod geometry;
pub mod lines;

pub use geometry::{G1, G2, G3, G4, Geometry};

pub trait EuclideanGeometry: Clone {
    type Space: EuclidianSpace<
            Self::InnerProduct,
            Point = Self::Point,
            Vector = Self::Vector,
            Scalar = Self::Scalar,
        >;
    type InnerProduct: InnerProductSpace<Self::Scalar, Vector = Self::Vector>;
    type Point: AffineSpace<Vector = Self::Vector, Scalar = Self::Scalar>;
    type Vector: VectorSpace<Scalar = Self::Scalar>;
    type Scalar: Field;
}

pub trait GeometricObject:
    Clone
    + Add<<Self::Geometry as EuclideanGeometry>::Vector>
    + Sub<<Self::Geometry as EuclideanGeometry>::Vector>
    + AddAssign<<Self::Geometry as EuclideanGeometry>::Vector>
    + SubAssign<<Self::Geometry as EuclideanGeometry>::Vector>
{
    type Geometry: EuclideanGeometry;
}

use crate::{
    field::{Field, Summable},
    geometric_object::EuclideanGeometry,
};

mod dot_product;
mod point;
mod space;
mod vector;

pub use dot_product::DotProduct;
pub use point::{Point, Point1, Point2, Point3, Point4};
pub use space::{Space, Space1, Space2, Space3, Space4};
pub use vector::{Vector, Vector1, Vector2, Vector3, Vector4};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Geometry<const N: usize, F: Field> {
    __field: std::marker::PhantomData<F>,
}

impl<const N: usize, F: Field> Geometry<N, F> {
    pub const fn new() -> Self {
        Self {
            __field: std::marker::PhantomData,
        }
    }
}

impl<const N: usize, F: Field + Summable> EuclideanGeometry for Geometry<N, F> {
    type Space = Space<N, F>;
    type InnerProduct = DotProduct<N>;
    type Point = Point<N, F>;
    type Vector = Vector<N, F>;
    type Scalar = F;
}

pub type G1<F> = Geometry<1, F>;
pub type G2<F> = Geometry<2, F>;
pub type G3<F> = Geometry<3, F>;
pub type G4<F> = Geometry<4, F>;

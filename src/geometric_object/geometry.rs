use crate::{
    affine_space::Point,
    euclidean_space::Space,
    field::{Field, Summable},
    geometric_object::EuclideanGeometry,
    inner_product::DotProduct,
    vector_space::Vector,
};

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

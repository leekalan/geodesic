use crate::{
    affine_space::Point, field::Field, inner_product::InnerProductSpace, vector_space::Vector,
};

use super::EuclidianSpace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Space<const N: usize, F: Field> {
    __field: std::marker::PhantomData<F>,
}

impl<const N: usize, F: Field> Space<N, F> {
    pub const fn new() -> Self {
        Self {
            __field: std::marker::PhantomData,
        }
    }
}

impl<const N: usize, F: Field, IP: InnerProductSpace<F, Vector = Vector<N, F>>> EuclidianSpace<IP>
    for Space<N, F>
{
    type Point = Point<N, F>;
    type Vector = Vector<N, F>;
    type Scalar = F;
}

pub type Space1<F> = Space<1, F>;
pub type Space2<F> = Space<2, F>;
pub type Space3<F> = Space<3, F>;
pub type Space4<F> = Space<4, F>;

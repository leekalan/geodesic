use super::Vector;
use crate::{
    field::{Field, Summable},
    inner_product::InnerProductSpace,
};

pub struct DotProduct<const N: usize> {}

impl<const N: usize, F: Field + Summable> InnerProductSpace<F> for DotProduct<N> {
    type Vector = Vector<N, F>;

    fn inner_product(lhs: &Self::Vector, rhs: &Self::Vector) -> F {
        lhs.raw_ref()
            .iter()
            .zip(rhs.raw_ref())
            .map(|(a, b)| a.clone() * b.clone())
            .sum()
    }
}

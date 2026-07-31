use crate::{
    field::{Absolutable, Field, Rootable},
    vector_space::VectorSpace,
};

/// An inner product space is a vector space equipped with an inner product.
///
/// # Invariants
/// - `inner_product(a, b + λ * c) == inner_product(a, b) + λ * inner_product(a, c)`
/// - `inner_product(a, b) == inner_product(b, a)`
/// - `inner_product(a, a) > 0` if `a != 0`
/// - `inner_product(a, a) == 0` if `a == 0`
pub trait InnerProductSpace<F: Field> {
    type Vector: VectorSpace<Scalar = F>;

    fn inner_product(lhs: &Self::Vector, rhs: &Self::Vector) -> F;

    fn norm_squared(vector: &Self::Vector) -> F {
        Self::inner_product(vector, vector)
    }

    fn norm(vector: &Self::Vector) -> F
    where
        F: Rootable,
    {
        Self::norm_squared(vector).sqrt()
    }

    fn normalise(mut vector: Self::Vector) -> Self::Vector
    where
        F: Rootable,
    {
        let norm = Self::norm(&vector);
        vector /= norm;
        vector
    }

    fn orthogonal(lhs: &Self::Vector, rhs: &Self::Vector) -> bool {
        Self::inner_product(lhs, rhs) == F::zero()
    }

    fn parrallel(lhs: &Self::Vector, rhs: &Self::Vector) -> bool
    where
        F: Absolutable,
    {
        Self::inner_product(lhs, rhs).abs() == Self::norm_squared(lhs) * Self::norm_squared(rhs)
    }
}

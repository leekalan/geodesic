use crate::{
    field::{Field, Rootable},
    vector_space::VectorSpace,
};

/// An inner product space is a vector space equipped with an inner product.
///
/// # Invariants
/// - `inner_product(a, b + λ * c) == inner_product(a, b) + λ * inner_product(a, c)`
/// - `inner_product(a, b) == inner_product(b, a)`
/// - `inner_product(a, a) > 0` if `a != 0`
/// - `inner_product(a, a) == 0` if `a == 0`
pub trait InnerProductSpace: VectorSpace {
    fn inner_product(lhs: &Self, rhs: &Self) -> Self::Scalar;

    fn norm_squared(vector: &Self) -> Self::Scalar {
        Self::inner_product(vector, vector)
    }

    fn norm(vector: &Self) -> Self::Scalar
    where
        Self::Scalar: Rootable,
    {
        Self::norm_squared(vector).sqrt()
    }

    fn normalize(mut vector: Self) -> Self
    where
        Self::Scalar: Rootable,
    {
        let norm = Self::norm(&vector);
        vector /= norm;
        vector
    }

    fn orthogonal(lhs: &Self, rhs: &Self) -> bool {
        Self::inner_product(lhs, rhs) == Self::Scalar::zero()
    }
}

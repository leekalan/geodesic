use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{affine_space::AffineSpace, euclidean_space::EuclidianSpace};

mod line;

pub use line::Line;

pub trait Geometry:
    Clone
    + Add<<Self::Space as AffineSpace>::Vector, Output = Self>
    + Sub<<Self::Space as AffineSpace>::Vector, Output = Self>
    + AddAssign<<Self::Space as AffineSpace>::Vector>
    + SubAssign<<Self::Space as AffineSpace>::Vector>
{
    type Space: EuclidianSpace;
}

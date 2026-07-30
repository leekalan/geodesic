use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::geometric_object::{EuclideanGeometry, GeometricObject};

#[derive(Debug, Clone, Copy)]
pub struct Line<G: EuclideanGeometry> {
    a: G::Point,
    b: G::Point,
}

impl<G: EuclideanGeometry> Line<G> {
    pub fn between(a: G::Point, b: G::Point) -> Self {
        Self { a, b }
    }
}

impl<G: EuclideanGeometry> Add<G::Vector> for Line<G> {
    type Output = Self;

    fn add(mut self, rhs: G::Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl<G: EuclideanGeometry> Sub<G::Vector> for Line<G> {
    type Output = Self;

    fn sub(mut self, rhs: G::Vector) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<G: EuclideanGeometry> AddAssign<G::Vector> for Line<G> {
    fn add_assign(&mut self, rhs: G::Vector) {
        self.a += rhs.clone();
        self.b += rhs;
    }
}

impl<G: EuclideanGeometry> SubAssign<G::Vector> for Line<G> {
    fn sub_assign(&mut self, rhs: G::Vector) {
        self.a -= rhs.clone();
        self.b -= rhs;
    }
}

impl<G: EuclideanGeometry> GeometricObject for Line<G> {
    type Geometry = G;
}

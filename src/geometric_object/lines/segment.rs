use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::geometric_object::{EuclideanGeometry, GeometricObject};

use super::Line;

#[derive(Debug, Clone, Copy)]
pub struct Segment<G: EuclideanGeometry> {
    a: G::Point,
    b: G::Point,
}

impl<G: EuclideanGeometry> Segment<G> {
    pub fn between(a: G::Point, b: G::Point) -> Self {
        Self { a, b }
    }

    pub fn point_direction(point: G::Point, direction: G::Vector) -> Self {
        Self {
            a: point.clone(),
            b: point + direction,
        }
    }
    
    /// The endpoints have no intrinsic ordering.
    pub fn end_points(self) -> (G::Point, G::Point) {
        (self.a, self.b)
    }

    pub fn line(self) -> Line<G> {
        Line::through(self.a, self.b)
    }
}

impl<G: EuclideanGeometry> Add<G::Vector> for Segment<G> {
    type Output = Self;

    fn add(mut self, rhs: G::Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl<G: EuclideanGeometry> Sub<G::Vector> for Segment<G> {
    type Output = Self;

    fn sub(mut self, rhs: G::Vector) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<G: EuclideanGeometry> AddAssign<G::Vector> for Segment<G> {
    fn add_assign(&mut self, rhs: G::Vector) {
        self.a += rhs.clone();
        self.b += rhs;
    }
}

impl<G: EuclideanGeometry> SubAssign<G::Vector> for Segment<G> {
    fn sub_assign(&mut self, rhs: G::Vector) {
        self.a -= rhs.clone();
        self.b -= rhs;
    }
}

impl<G: EuclideanGeometry> GeometricObject for Segment<G> {
    type Geometry = G;
}

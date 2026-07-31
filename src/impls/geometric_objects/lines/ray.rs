use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::geometric_object::{EuclideanGeometry, GeometricObject};

use super::Line;

#[derive(Debug, Clone, Copy)]
pub struct Ray<G: EuclideanGeometry> {
    origin: G::Point,
    direction: G::Vector,
}

impl<G: EuclideanGeometry> Ray<G> {
    pub fn point_through(origin: G::Point, point: G::Point) -> Self {
        Self {
            direction: point - origin.clone(),
            origin,
        }
    }

    pub fn point_direction(origin: G::Point, direction: G::Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(self) -> G::Point {
        self.origin
    }

    pub fn direction(self) -> G::Vector {
        self.direction
    }

    pub fn line(self) -> Line<G> {
        Line::point_direction(self.origin, self.direction)
    }
}

impl<G: EuclideanGeometry> Add<G::Vector> for Ray<G> {
    type Output = Self;

    fn add(mut self, rhs: G::Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl<G: EuclideanGeometry> Sub<G::Vector> for Ray<G> {
    type Output = Self;

    fn sub(mut self, rhs: G::Vector) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<G: EuclideanGeometry> AddAssign<G::Vector> for Ray<G> {
    fn add_assign(&mut self, rhs: G::Vector) {
        self.origin += rhs;
    }
}

impl<G: EuclideanGeometry> SubAssign<G::Vector> for Ray<G> {
    fn sub_assign(&mut self, rhs: G::Vector) {
        self.origin -= rhs;
    }
}

impl<G: EuclideanGeometry> GeometricObject for Ray<G> {
    type Geometry = G;
}

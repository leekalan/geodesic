use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::geometric_object::{EuclideanGeometry, GeometricObject};

#[derive(Debug, Clone, Copy)]
pub struct Line<G: EuclideanGeometry> {
    point: G::Point,
    direction: G::Vector,
}

impl<G: EuclideanGeometry> Line<G> {
    pub fn through(a: G::Point, b: G::Point) -> Self {
        Self { point: a.clone(), direction: b - a }
    }

    pub fn point_direction(point: G::Point, direction: G::Vector) -> Self {
        Self {
            point,
            direction,
        }
    }

    pub fn parallel_through(&self, point: G::Point) -> Self {
        Self::point_direction(point, self.direction.clone())
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
        self.point += rhs;
    }
}

impl<G: EuclideanGeometry> SubAssign<G::Vector> for Line<G> {
    fn sub_assign(&mut self, rhs: G::Vector) {
        self.point -= rhs;
    }
}

impl<G: EuclideanGeometry> GeometricObject for Line<G> {
    type Geometry = G;
}

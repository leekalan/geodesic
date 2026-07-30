use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::geometric_object::{EuclideanGeometry, GeometricObject};

use super::{Line, Ray, Segment};

#[derive(Debug, Clone, Copy)]
pub struct DirectedSegment<G: EuclideanGeometry> {
    origin: G::Point,
    end_point: G::Point,
}

impl<G: EuclideanGeometry> DirectedSegment<G> {
    pub fn between(origin: G::Point, end_point: G::Point) -> Self {
        Self {
            origin,
            end_point,
        }
    }

    pub fn point_direction(origin: G::Point, direction: G::Vector) -> Self {
        Self {
            end_point: origin.clone() + direction,
            origin,
        }
    }

    pub fn origin(self) -> G::Point {
        self.origin
    }

    pub fn end_point(self) -> G::Point {
        self.end_point
    }
    
    pub fn direction(self) -> G::Vector {
        self.end_point - self.origin
    }

    pub fn line(self) -> Line<G> {
        Line::through(self.origin, self.end_point)
    }

    pub fn ray(self) -> Ray<G> {
        Ray::point_through(self.origin, self.end_point)
    }

    pub fn segment(self) -> Segment<G> {
        Segment::between(self.origin, self.end_point)
    }
}

impl<G: EuclideanGeometry> Add<G::Vector> for DirectedSegment<G> {
    type Output = Self;

    fn add(mut self, rhs: G::Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl<G: EuclideanGeometry> Sub<G::Vector> for DirectedSegment<G> {
    type Output = Self;

    fn sub(mut self, rhs: G::Vector) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<G: EuclideanGeometry> AddAssign<G::Vector> for DirectedSegment<G> {
    fn add_assign(&mut self, rhs: G::Vector) {
        self.origin += rhs.clone();
        self.end_point += rhs;
    }
}

impl<G: EuclideanGeometry> SubAssign<G::Vector> for DirectedSegment<G> {
    fn sub_assign(&mut self, rhs: G::Vector) {
        self.origin -= rhs.clone();
        self.end_point -= rhs;
    }
}

impl<G: EuclideanGeometry> GeometricObject for DirectedSegment<G> {
    type Geometry = G;
}

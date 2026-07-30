use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{euclidean_space::EuclidianSpace, geometry::Geometry};

#[derive(Debug, Clone, Copy)]
pub struct Line<S: EuclidianSpace> {
    a: S,
    b: S,
}

impl<S: EuclidianSpace> Line<S> {
    pub fn between(a: S, b: S) -> Self {
        Self { a, b }
    }
}

impl<S: EuclidianSpace> Add<S::Vector> for Line<S> {
    type Output = Self;

    fn add(mut self, rhs: S::Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl<S: EuclidianSpace> Sub<S::Vector> for Line<S> {
    type Output = Self;

    fn sub(mut self, rhs: S::Vector) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<S: EuclidianSpace> AddAssign<S::Vector> for Line<S> {
    fn add_assign(&mut self, rhs: S::Vector) {
        self.a += rhs.clone();
        self.b += rhs;
    }
}

impl<S: EuclidianSpace> SubAssign<S::Vector> for Line<S> {
    fn sub_assign(&mut self, rhs: S::Vector) {
        self.a -= rhs.clone();
        self.b -= rhs;
    }
}

impl<S: EuclidianSpace> Geometry for Line<S> {
    type Space = S;
}

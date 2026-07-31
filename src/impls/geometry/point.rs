use std::array;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::Vector;
use crate::{affine_space::AffineSpace, field::Field};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<const N: usize, F: Field> {
    data: [F; N],
}

impl<const N: usize, F: Field> Point<N, F> {
    pub fn new(data: [F; N]) -> Self {
        Self { data }
    }

    pub fn raw(self) -> [F; N] {
        self.data
    }
}

impl<const N: usize, F: Field> Add<Vector<N, F>> for Point<N, F> {
    type Output = Self;

    fn add(mut self, rhs: Vector<N, F>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize, F: Field> Sub<Vector<N, F>> for Point<N, F> {
    type Output = Self;

    fn sub(mut self, rhs: Vector<N, F>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize, F: Field> Sub for Point<N, F> {
    type Output = Vector<N, F>;

    fn sub(mut self, rhs: Point<N, F>) -> Self::Output {
        for (a, b) in self.data.iter_mut().zip(rhs.raw()) {
            *a -= b;
        }
        Vector::new(self.data)
    }
}

impl<const N: usize, F: Field> AddAssign<Vector<N, F>> for Point<N, F> {
    fn add_assign(&mut self, rhs: Vector<N, F>) {
        for (a, b) in self.data.iter_mut().zip(rhs.raw()) {
            *a += b;
        }
    }
}

impl<const N: usize, F: Field> SubAssign<Vector<N, F>> for Point<N, F> {
    fn sub_assign(&mut self, rhs: Vector<N, F>) {
        for (a, b) in self.data.iter_mut().zip(rhs.raw()) {
            *a -= b;
        }
    }
}

impl<const N: usize, F: Field> AffineSpace for Point<N, F> {
    type Vector = Vector<N, F>;
    type Scalar = F;

    fn origin() -> Self {
        Self {
            data: array::from_fn(|_| F::zero()),
        }
    }
}

pub type Point1<F> = Point<1, F>;
pub type Point2<F> = Point<2, F>;
pub type Point3<F> = Point<3, F>;
pub type Point4<F> = Point<4, F>;

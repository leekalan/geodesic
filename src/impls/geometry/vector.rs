use std::array;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::field::Field;
use crate::vector_space::VectorSpace;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize, F: Field> {
    data: [F; N],
}

impl<const N: usize, F: Field> Vector<N, F> {
    pub fn new(data: [F; N]) -> Self {
        Self { data }
    }

    pub fn raw(self) -> [F; N] {
        self.data
    }

    pub fn raw_ref(&self) -> &[F; N] {
        &self.data
    }
}

impl<const N: usize, F: Field> Add<Vector<N, F>> for Vector<N, F> {
    type Output = Self;

    fn add(mut self, rhs: Vector<N, F>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize, F: Field> Neg for Vector<N, F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            data: self.data.map(|a| -a),
        }
    }
}

impl<const N: usize, F: Field> Sub<Vector<N, F>> for Vector<N, F> {
    type Output = Self;

    fn sub(mut self, rhs: Vector<N, F>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize, F: Field> Mul<F> for Vector<N, F> {
    type Output = Self;

    fn mul(mut self, rhs: F) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const N: usize, F: Field> Div<F> for Vector<N, F> {
    type Output = Self;

    fn div(mut self, rhs: F) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const N: usize, F: Field> AddAssign<Vector<N, F>> for Vector<N, F> {
    fn add_assign(&mut self, rhs: Vector<N, F>) {
        for (a, b) in self.data.iter_mut().zip(rhs.data) {
            *a += b;
        }
    }
}

impl<const N: usize, F: Field> SubAssign<Vector<N, F>> for Vector<N, F> {
    fn sub_assign(&mut self, rhs: Vector<N, F>) {
        for (a, b) in self.data.iter_mut().zip(rhs.data) {
            *a -= b;
        }
    }
}

impl<const N: usize, F: Field> MulAssign<F> for Vector<N, F> {
    fn mul_assign(&mut self, rhs: F) {
        for a in self.data.iter_mut() {
            *a *= rhs.clone();
        }
    }
}

impl<const N: usize, F: Field> DivAssign<F> for Vector<N, F> {
    fn div_assign(&mut self, rhs: F) {
        for a in self.data.iter_mut() {
            *a /= rhs.clone();
        }
    }
}

impl<const N: usize, F: Field> VectorSpace for Vector<N, F> {
    type Scalar = F;

    fn zero() -> Self {
        Self {
            data: array::from_fn(|_| F::zero()),
        }
    }
}

pub type Vector1<F> = Vector<1, F>;
pub type Vector2<F> = Vector<2, F>;
pub type Vector3<F> = Vector<3, F>;
pub type Vector4<F> = Vector<4, F>;

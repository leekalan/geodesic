pub mod affine_space;
pub mod euclidean_space;
pub mod field;
pub mod geometric_object;
pub mod inner_product;
pub mod vector_space;

mod impls;

pub mod prelude {
    use super::impls::*;

    // pub use fields::*;
    pub use geometric_objects::*;
    pub use geometry::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn line_test() {
        let mut a = Point2::new([1., 0.]);
        let mut b = Point2::new([0., 1.]);
        let c = Point2::new([0., 0.]);

        let vec = Vector2::new([1., 1.]);

        a += vec * 2.;
        b -= vec / 2.;

        let mut line = Line::<G2<_>>::through(a, b);

        line += vec * 2.;

        let line2 = line.parallel_through(c);

        print!("{:?}\n{:?}\n", line, line2);
    }
}

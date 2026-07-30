pub mod affine_space;
pub mod euclidean_space;
pub mod field;
pub mod geometric_object;
pub mod inner_product;
pub mod vector_space;

#[cfg(test)]
mod tests {
    use super::*;

    use affine_space::Point2;
    use geometric_object::{G2, lines::Line};
    use vector_space::Vector2;

    #[test]
    fn line_test() {
        let mut a = Point2::new([1., 0.]);
        let mut b = Point2::new([0., 1.]);

        let vec = Vector2::new([1., 1.]);

        a += vec * 2.;
        b -= vec / 2.;

        let mut line = Line::<G2<_>>::through(a, b);

        line += vec * 2.;

        print!("{:?}", line);
    }
}

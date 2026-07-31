use crate::field::{Field, Summable};

impl Field for f32 {
    fn zero() -> Self {
        0f32
    }

    fn one() -> Self {
        1f32
    }
}
impl Summable for f32 {}

impl Field for f64 {
    fn zero() -> Self {
        0f64
    }

    fn one() -> Self {
        0f64
    }
}
impl Summable for f64 {}

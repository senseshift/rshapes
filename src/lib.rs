mod data;
pub use data::*;

mod shapes;
pub use shapes::*;

pub mod traits;
pub use traits::distance;
pub use traits::distance_squared;

use nalgebra::Scalar;
pub use nalgebra::{Point2, Vector2};
use num::traits::NumOps;

pub trait FloatMath: Scalar + NumOps + PartialOrd + Copy + Into<f64> {}

impl FloatMath for u8 {}
impl FloatMath for u16 {}
impl FloatMath for u32 {}
impl FloatMath for f32 {}
impl FloatMath for f64 {}

#[cfg(test)]
pub mod testing;

#[cfg(feature = "proptest-support")]
pub mod proptest;

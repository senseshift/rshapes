use crate::Rectangle;
use nalgebra::Scalar;

pub trait BoundingBox<T: Scalar> {
  fn bbox(&self) -> Rectangle<T>;
}

impl<T: Scalar, U: BoundingBox<T>> BoundingBox<T> for &U {
  fn bbox(&self) -> Rectangle<T> {
    U::bbox(*self)
  }
}

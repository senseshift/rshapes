use crate::*;

pub trait Centroid<T: Scalar> {
  fn centroid(&self) -> Point2<T>;
}

impl<T: Scalar, U: Centroid<T>> Centroid<T> for &U {
  fn centroid(&self) -> Point2<T> {
    U::centroid(*self)
  }
}

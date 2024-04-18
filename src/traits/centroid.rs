use crate::*;

pub trait Centroid<T: Scalar> {
  fn centroid(&self) -> Point2<T>;
}

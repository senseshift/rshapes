use crate::*;

pub trait PointsInside<T: Scalar> {
  fn points_inside(&self) -> Vec<Point2<T>>;
}

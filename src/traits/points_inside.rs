use crate::*;

pub trait PointsInside<T: Scalar> {
  fn points_inside(&self) -> Vec<Point2<T>>;
}

impl<T: Scalar, U: PointsInside<T>> PointsInside<T> for &U {
  fn points_inside(&self) -> Vec<Point2<T>> {
    U::points_inside(*self)
  }
}

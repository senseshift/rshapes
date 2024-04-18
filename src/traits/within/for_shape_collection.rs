use super::Within;
use crate::*;

use num::Unsigned;

impl<T, U> Within<&Point2<T>> for ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
  Shape<T, U>: for<'a> Within<&'a Point2<T>, Result = bool>,
{
  type Result = bool;

  fn within(&self, other: &Point2<T>) -> Self::Result {
    for geometry in &self.shapes {
      if geometry.within(other) {
        return true;
      }
    }
    false
  }
}
impl<T, U> Within<Point2<T>> for ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
  Shape<T, U>: for<'a> Within<&'a Point2<T>, Result = bool>,
{
  type Result = bool;

  fn within(&self, other: Point2<T>) -> Self::Result {
    self.within(&other)
  }
}

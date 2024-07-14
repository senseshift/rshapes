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

#[cfg(test)]
mod tests {

  use crate::{traits::Within, Circle, Point2, Rectangle, Shape, ShapeCollection};

  #[test]
  fn shape_collection_within_u8() {
    let collection = ShapeCollection::new(vec![
      Shape::Rectangle(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10]))),
      Shape::Circle(Circle::new(Point2::from([10, 10]), 5)),
    ]);

    assert!(collection.within(Point2::from([0, 0]))); // top-left of rectangle
    assert!(collection.within(Point2::from([5, 5]))); // center of rectangle
    assert!(collection.within(Point2::from([12, 12]))); // inside circle

    assert!(!collection.within(Point2::from([20, 20]))); // outside both
  }
}

use super::Within;
use crate::*;

impl<T> Within<&Point2<T>> for Rectangle<T>
where
  T: Scalar + PartialOrd,
{
  type Result = bool;

  fn within(&self, other: &Point2<T>) -> Self::Result {
    self.min().x <= other.x
      && other.x <= self.max().x
      && self.min().y <= other.y
      && other.y <= self.max().y
  }
}
impl<T> Within<Point2<T>> for Rectangle<T>
where
  T: Scalar + PartialOrd,
{
  type Result = bool;

  fn within(&self, other: Point2<T>) -> Self::Result {
    self.within(&other)
  }
}

#[cfg(test)]
mod tests {
  use crate::testing::*;
  use crate::{traits::Within, Point2, Rectangle};
  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([0, 0]) => true; "top-left")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([10, 10]) => true; "bottom-right")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([5, 5]) => true; "center")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([0, 5]) => true; "left")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([5, 0]) => true; "top")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([10, 5]) => true; "right")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([5, 10]) => true; "bottom")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([11, 5]) => false; "outside right")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([5, 11]) => false; "outside top")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([11, 11]) => false; "outside top-right")]
  #[test_case(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10])), Point2::from([255, 255]) => false; "outside max")]
  #[test_case(Rectangle::new(Point2::from([5, 5]), Point2::from([10, 10])), Point2::from([6, 6]) => true; "non-centered inside")]
  #[test_case(Rectangle::new(Point2::from([5, 5]), Point2::from([10, 10])), Point2::from([11, 11]) => false; "non-centered outside")]
  fn rectangle_within_u8(rectangle: Rectangle<u8>, point: Point2<u8>) -> bool {
    rectangle.within(&point)
  }

  #[proptest]
  fn rectangle_within_u8_fuzz(rectangle: Rectangle<u8>, point: PointView2<u8>) {
    let _out = rectangle.within(&point.into());
  }
}

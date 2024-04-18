use super::Within;
use crate::*;

impl Within<&Point2<u8>> for Circle<u8, u8> {
  type Result = bool;

  fn within(&self, other: &Point2<u8>) -> Self::Result {
    if self.radius == 0 {
      return other == &self.center;
    }

    crate::distance_squared(&self.center, other) <= (self.radius as f64).powi(2)
  }
}

impl Within<Point2<u8>> for Circle<u8, u8> {
  type Result = bool;

  fn within(&self, other: Point2<u8>) -> Self::Result {
    self.within(&other)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::testing::PointView;
  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([0, 0]) => true; "center")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([1, 0]) => true; "inside right")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([0, 1]) => true; "inside top")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([10, 0]) => true; "inside edge right")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([0, 10]) => true; "inside edge top")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([10, 10]) => false; "outside edge top-right")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([11, 0]) => false; "outside right")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([0, 11]) => false; "outside top")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([11, 11]) => false; "outside top-right")]
  #[test_case(Circle::new(Point2::from([0, 0]), 10), Point2::from([255, 255]) => false; "outside max")]
  fn circle_within_u8(circle: Circle<u8, u8>, point: Point2<u8>) -> bool {
    circle.within(&point)
  }

  #[proptest]
  fn circle_within_u8_fuzz(circle: Circle<u8, u8>, point: PointView<u8, 2>) {
    let _out = circle.within(&point.into());
  }
}

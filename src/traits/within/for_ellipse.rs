use super::Within;
use crate::*;

impl Within<&Point2<u8>> for Ellipse<u8, u8> {
  type Result = bool;

  fn within(&self, other: &Point2<u8>) -> Self::Result {
    if *self.width() == 0 || *self.height() == 0 {
      return other == &self.center;
    }

    if !self.bbox().within(other) {
      return false;
    }

    let px = ((other.x as f64) - (self.center.x as f64));
    let px2 = px.powi(2);

    let py = ((other.y as f64) - (self.center.y as f64));
    let py2 = py.powi(2);

    let rx2 = (*self.width() as f64).powi(2);
    let ry2 = (*self.height() as f64).powi(2);

    let dst = px2 / rx2 + py2 / ry2;

    dst <= 1.0
  }
}

impl Within<Point2<u8>> for Ellipse<u8, u8> {
  type Result = bool;

  fn within(&self, other: Point2<u8>) -> Self::Result {
    self.within(&other)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::*;

  use crate::testing::PointView;
  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Ellipse::new(Point2::from([0, 0]), (4, 5)), Point2::from([0, 0]) => true; "center")]
  #[test_case(Ellipse::new(Point2::from([0, 0]), (4, 5)), Point2::from([2, 2]) => true; "inside")]
  #[test_case(Ellipse::new(Point2::from([0, 0]), (5, 5)), Point2::from([0, 5]) => true; "edge")]
  #[test_case(Ellipse::new(Point2::from([0, 0]), (4, 5)), Point2::from([10, 11]) => false; "outside")]
  #[test_case(Ellipse::new(Point2::from([0, 0]), (4, 5)), Point2::from([255, 255]) => false; "outside max")]
  #[test_case(Ellipse::new(Point2::from([5, 5]), (4, 3)), Point2::from([6, 6]) => true; "non-centered inside")]
  #[test_case(Ellipse::new(Point2::from([5, 5]), (4, 3)), Point2::from([10, 10]) => false; "non-centered outside")]
  fn ellipse_within_u8(ellipse: Ellipse<u8, u8>, point: Point2<u8>) -> bool {
    ellipse.within(&point)
  }

  #[proptest]
  fn ellipse_within_u8_fuzz(ellipse: Ellipse<u8, u8>, point: PointView<u8, 2>) {
    let _out = ellipse.within(&point.into());
  }
}

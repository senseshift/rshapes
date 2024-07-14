use super::Within;
use crate::traits::bbox::*;
use crate::*;

impl Within<&Point2<u8>> for Triangle<u8> {
  type Result = bool;

  fn within(&self, other: &Point2<u8>) -> Self::Result {
    if !self.bbox().within(other) {
      return false;
    }

    let a = self.0.map(|x| x as f64);
    let b = self.1.map(|x| x as f64);
    let c = self.2.map(|x| x as f64);

    let p = other.map(|x| x as f64);

    let area = |a: &Point2<f64>, b: &Point2<f64>, c: &Point2<f64>| {
      let x1 = a.x - c.x;
      let y1 = a.y - c.y;
      let x2 = b.x - c.x;
      let y2 = b.y - c.y;
      let area = x1 * y2 - x2 * y1;
      area.abs()
    };

    let area_abc = area(&a, &b, &c);
    let area_pbc = area(&p, &b, &c);
    let area_apc = area(&a, &p, &c);
    let area_abp = area(&a, &b, &p);

    area_abc == area_pbc + area_apc + area_abp
  }
}
impl Within<Point2<u8>> for Triangle<u8> {
  type Result = bool;

  fn within(&self, other: Point2<u8>) -> Self::Result {
    self.within(&other)
  }
}

#[cfg(test)]
mod tests {
  use crate::testing::*;
  use crate::{traits::Within, Point2, Triangle};
  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Triangle::new(Point2::from([0, 0]), Point2::from([10, 0]), Point2::from([0, 10])), Point2::from([0, 0]) => true; "top-left")]
  #[test_case(Triangle::new(Point2::from([0, 0]), Point2::from([10, 0]), Point2::from([0, 10])), Point2::from([20, 20]) => false; "outside bbox")]
  #[test_case(Triangle::new(Point2::from([0, 0]), Point2::from([10, 0]), Point2::from([0, 10])), Point2::from([9, 9]) => false; "inside bbox")]
  fn triangle_within_u8(triangle: Triangle<u8>, point: Point2<u8>) -> bool {
    triangle.within(point)
  }

  #[proptest]
  fn triangle_within_u8_fuzz(triangle: Triangle<u8>, point: PointView2<u8>) {
    let _out = triangle.within(&point.into());
  }
}

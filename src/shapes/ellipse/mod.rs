use derivative::Derivative;
use getset::Getters;

use crate::*;

#[cfg_attr(
  feature = "serde-serialize",
  derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Getters, Derivative)]
#[derivative(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Ellipse<T: Scalar, R: Scalar> {
  #[getset(get = "pub")]
  pub(crate) center: Point2<T>,
  #[getset(get = "pub")]
  pub(crate) radius: (R, R),
}

impl<T, R> Ellipse<T, R>
where
  T: Scalar,
  R: Scalar,
{
  #[inline]
  pub fn new(center: Point2<T>, radius: (R, R)) -> Self {
    Self { center, radius }
  }

  #[inline]
  pub fn width(&self) -> &R {
    &self.radius.0
  }

  #[inline]
  pub fn height(&self) -> &R {
    &self.radius.1
  }
}

impl Ellipse<u8, u8> {
  pub fn point_intersection(&self, point: &Point2<u8>, max_iterations: usize) -> Point2<f64> {
    let a = self.radius.0 as f64;
    let b = self.radius.1 as f64;

    let epsilon = 0.1 / a.max(b);

    let dx = point.x as f64 - self.center.x as f64;
    let dy = point.y as f64 - self.center.y as f64;
    let p1 = Point2::new(dx, dy);

    // Intersection of straight line from origin to p with ellipse as the first approximation:
    let mut phi = (a * p1.y).atan2(b * p1.x);

    // Newton iteration to find solution of
    // f(θ) := (a^2 − b^2) cos(phi) sin(phi) − x a sin(phi) + y b cos(phi) = 0:
    for _ in 0..max_iterations {
      let sin_phi = phi.sin();
      let cos_phi = phi.cos();

      let f = (a * a - b * b) * cos_phi * sin_phi - dx * a * sin_phi + dy * b * cos_phi;
      let f1 = (a * a - b * b) * (cos_phi * cos_phi - sin_phi * sin_phi)
        - p1.x * a * cos_phi
        - p1.y * b * sin_phi;

      let delta = f / f1;
      phi -= delta;
      if delta.abs() < epsilon {
        break;
      }
    }

    let x = a * phi.cos() + self.center.x as f64;
    let y = b * phi.sin() + self.center.y as f64;

    Point2::new(x, y)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use float_cmp::assert_approx_eq;
  use nalgebra::Point2;
  use test_case::test_case;

  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(130, 120), Point2::new(130.0, 120.0); "point_at_0_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(127, 127), Point2::new(125.91230017765763, 124.03251492896399); "point_at_45_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(120, 125), Point2::new(120.0, 125.0); "point_at_90_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(113, 127), Point2::new(114.08769982234237, 124.03251492896399); "point_at_135_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(110, 120), Point2::new(110.0, 120.0); "point_at_180_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(113, 113), Point2::new(114.08769982234237, 115.96748507103601); "point_at_225_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(120, 115), Point2::new(120.0, 115.0); "point_at_270_deg")]
  #[test_case(Ellipse::new(Point2::new(120, 120), (10, 5)), Point2::new(127, 113), Point2::new(125.91230017765763, 115.96748507103601); "point_at_315_deg")]
  fn test_ellipse_point_intersection_u8(
    ellipse: Ellipse<u8, u8>,
    point: Point2<u8>,
    expected: Point2<f64>,
  ) {
    let intersection = ellipse.point_intersection(&point, 1000);

    println!("Intersection Point: {:?}", intersection);

    assert_approx_eq!(f64, intersection.x, expected.x, ulps = 2);
    assert_approx_eq!(f64, intersection.y, expected.y, ulps = 2);
  }
}

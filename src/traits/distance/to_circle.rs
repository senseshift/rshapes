use crate::{
  traits::{distance, Distance},
  Circle,
};

impl Distance<&Circle<u8, u8>> for Circle<u8, u8> {
  type Result = f64;

  fn distance(&self, other: &Circle<u8, u8>) -> Self::Result {
    let centers_distance = distance(&self.center, &other.center);
    let radius_sum = self.radius as f64 + other.radius as f64;

    if centers_distance > radius_sum {
      centers_distance - radius_sum
    } else {
      0.0
    }
  }
}

impl Distance<Circle<u8, u8>> for Circle<u8, u8> {
  type Result = f64;

  fn distance(&self, other: Circle<u8, u8>) -> Self::Result {
    self.distance(&other)
  }
}

#[cfg(test)]
mod tests {
  use crate::{traits::Distance, Circle};
  use float_cmp::assert_approx_eq;
  use test_case::test_case;
  use test_strategy::proptest;

  #[proptest]
  fn circle_distance_u8_fuzz(circle1: Circle<u8, u8>, circle2: Circle<u8, u8>) {
    let _out = circle1.distance(&circle2);
  }

  #[test_case(Circle::new([60, 60].into(), 10), Circle::new([60, 120].into(), 10), 40.0)]
  #[test_case(Circle::new([60, 60].into(), 10), Circle::new([60, 120].into(), 5), 45.0)]
  #[test_case(Circle::new([60, 60].into(), 10), Circle::new([120, 60].into(), 10), 40.0)]
  #[test_case(Circle::new([60, 60].into(), 10), Circle::new([120, 60].into(), 5), 45.0)]
  #[test_case(Circle::new([60, 60].into(), 10), Circle::new([120, 120].into(), 10), 64.8528137423857)]
  #[test_case(Circle::new([60, 60].into(), 10), Circle::new([120, 120].into(), 5), 69.8528137423857)]
  fn circle_distance_u8(circle1: Circle<u8, u8>, circle2: Circle<u8, u8>, expected: f64) {
    let out = circle1.distance(&circle2);
    assert_approx_eq!(f64, out, expected, ulps = 2);
  }
}

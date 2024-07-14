use derivative::Derivative;
use nalgebra::*;

#[cfg_attr(
  feature = "serde-serialize",
  derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Derivative)]
#[derivative(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Line<T>
where
  T: Scalar,
{
  pub start: Point2<T>,
  pub end: Point2<T>,
}

impl<T> Line<T>
where
  T: Scalar,
{
  #[inline]
  pub fn new_unchecked(start: Point2<T>, end: Point2<T>) -> Self {
    Self { start, end }
  }
}

impl<T> Line<T>
where
  T: Scalar + PartialOrd,
{
  pub fn new(a: Point2<T>, b: Point2<T>) -> Self {
    if a < b {
      Self::new_unchecked(a, b)
    } else {
      Self::new_unchecked(b, a)
    }
  }
}

#[cfg(test)]
mod test {
  use test_case::test_case;

  use crate::{Line, Point2};

  #[test_case(
    Line::new(Point2::new(0, 0), Point2::new(10, 10)),
    Point2::new(0, 0),
    Point2::new(10, 10)
  )]
  #[test_case(
    Line::new(Point2::new(10, 10), Point2::new(0, 0)),
    Point2::new(0, 0),
    Point2::new(10, 10)
  )]
  fn test_new_normalizes(line: Line<u8>, start: Point2<u8>, end: Point2<u8>) {
    assert_eq!(line.start, start);
    assert_eq!(line.end, end);
  }
}

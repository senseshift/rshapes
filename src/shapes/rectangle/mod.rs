use derivative::Derivative;
use num::Num;

use crate::*;

#[cfg_attr(
  feature = "serde-serialize",
  derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Derivative)]
#[derivative(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rectangle<T: Scalar>(Point2<T>, Point2<T>);

impl<T> Rectangle<T>
where
  T: Scalar,
{
  #[inline]
  pub fn new_unchecked(min: Point2<T>, max: Point2<T>) -> Self {
    Self(min, max)
  }

  #[inline]
  pub fn min(&self) -> &Point2<T> {
    &self.0
  }

  #[inline]
  pub fn max(&self) -> &Point2<T> {
    &self.1
  }
}

impl<T> Rectangle<T>
where
  T: Scalar + PartialOrd + Copy,
{
  pub fn new(a: Point2<T>, b: Point2<T>) -> Self {
    let x_max = if a.x > b.x { a.x } else { b.x };
    let x_min = if a.x < b.x { a.x } else { b.x };
    let y_max = if a.y > b.y { a.y } else { b.y };
    let y_min = if a.y < b.y { a.y } else { b.y };

    Self(Point2::new(x_min, y_min), Point2::new(x_max, y_max))
  }
}

impl<T> Rectangle<T>
where
  T: Scalar + Num,
{
  /// Returns the width of the rectangle.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Rectangle};
  ///
  /// let rectangle = Rectangle::new(Point2::new(0, 0), Point2::new(10, 10));
  /// assert_eq!(rectangle.width(), 10);
  ///
  /// ```
  #[inline]
  pub fn width(&self) -> T {
    self.max().x.clone() - self.min().x.clone()
  }

  /// Returns the height of the rectangle.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Rectangle};
  ///
  /// let rectangle = Rectangle::new(Point2::new(0, 0), Point2::new(10, 10));
  /// assert_eq!(rectangle.height(), 10);
  ///
  /// ```
  #[inline]
  pub fn height(&self) -> T {
    self.max().y.clone() - self.min().y.clone()
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn points_inside_are_within() {
    use crate::traits::*;
    use crate::*;

    let rectangle = Rectangle::new(Point2::new(0, 0), Point2::new(2, 2));
    let points = rectangle.points_inside();

    for point in points {
      assert!(
        rectangle.within(&point),
        "point {:?} is not within rectangle {:?}",
        point,
        rectangle
      );
    }
  }
}

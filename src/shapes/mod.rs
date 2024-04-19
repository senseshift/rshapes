mod circle;
mod ellipse;
mod rectangle;
mod shape_collection;
mod triangle;

pub use circle::*;
pub use ellipse::*;
pub use rectangle::*;
pub use shape_collection::*;
pub use triangle::*;

use derivative::Derivative;
use nalgebra::Scalar;
use num::Unsigned;

#[derive(Derivative)]
#[derivative(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Shape<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
{
  Rectangle(Rectangle<T>),
  Circle(Circle<T, U>),
  Ellipse(Ellipse<T, U>),
  Triangle(Triangle<T>),
  Collection(ShapeCollection<T, U>),
}

impl<T, R> From<Rectangle<T>> for Shape<T, R>
where
  T: Scalar + Clone,
  R: Scalar + Unsigned,
{
  fn from(rectangle: Rectangle<T>) -> Self {
    Self::Rectangle(rectangle)
  }
}

impl<T, R> From<Circle<T, R>> for Shape<T, R>
where
  T: Scalar + Clone,
  R: Scalar + Unsigned,
{
  fn from(circle: Circle<T, R>) -> Self {
    Self::Circle(circle)
  }
}

impl<T, R> From<Ellipse<T, R>> for Shape<T, R>
where
  T: Scalar + Clone,
  R: Scalar + Unsigned,
{
  fn from(ellipse: Ellipse<T, R>) -> Self {
    Self::Ellipse(ellipse)
  }
}

impl<T, R> From<Triangle<T>> for Shape<T, R>
where
  T: Scalar + Clone,
  R: Scalar + Unsigned,
{
  fn from(triangle: Triangle<T>) -> Self {
    Self::Triangle(triangle)
  }
}

impl<T, R> From<ShapeCollection<T, R>> for Shape<T, R>
where
  T: Scalar + Clone,
  R: Scalar + Unsigned,
{
  fn from(collection: ShapeCollection<T, R>) -> Self {
    Self::Collection(collection)
  }
}

#[cfg(test)]
mod tests {
  use test_strategy::proptest;

  use super::*;
  use crate::testing::*;
  use crate::traits::*;

  #[proptest]
  fn shape_u8_points_inside_are_within_fuzz(shape_view: ShapeView<u8, u8>) {
    let shape = Shape::from(shape_view);
    let points = shape.points_inside();
    for point in points {
      assert!(
        shape.within(&point),
        "point {:?} is not within shape {:?}",
        point,
        shape
      );
    }
  }
}

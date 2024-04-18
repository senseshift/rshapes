mod for_circle;
mod for_ellipse;
mod for_rectangle;
mod for_shape_collection;
mod for_triangle;

pub use for_circle::*;
pub use for_ellipse::*;
pub use for_rectangle::*;
pub use for_shape_collection::*;
pub use for_triangle::*;

use crate::*;
use num::Unsigned;

/// Generic trait to determine if a shape is within another shape.
pub trait Within<T> {
  type Result;

  fn within(&self, other: T) -> Self::Result;
}

impl<T, U> Within<&Point2<T>> for Shape<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
  Ellipse<T, U>: for<'a> Within<&'a Point2<T>, Result = bool>,
  Circle<T, U>: for<'a> Within<&'a Point2<T>, Result = bool>,
  Rectangle<T>: for<'a> Within<&'a Point2<T>, Result = bool>,
  Triangle<T>: for<'a> Within<&'a Point2<T>, Result = bool>,
{
  type Result = bool;

  fn within(&self, other: &Point2<T>) -> Self::Result {
    match self {
      Self::Circle(circle) => circle.within(other),
      Self::Ellipse(ellipse) => ellipse.within(other),
      Self::Rectangle(rectangle) => rectangle.within(other),
      Self::Triangle(triangle) => triangle.within(other),
      Self::Collection(collection) => collection.within(other),
    }
  }
}

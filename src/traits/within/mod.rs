mod for_circle;
mod for_ellipse;
mod for_rectangle;
mod for_shape_collection;
mod for_triangle;

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

impl<T, U> Within<Point2<T>> for Shape<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
  Ellipse<T, U>: for<'a> Within<&'a Point2<T>, Result = bool>,
  Circle<T, U>: for<'a> Within<&'a Point2<T>, Result = bool>,
  Rectangle<T>: for<'a> Within<&'a Point2<T>, Result = bool>,
  Triangle<T>: for<'a> Within<&'a Point2<T>, Result = bool>,
{
  type Result = bool;

  fn within(&self, other: Point2<T>) -> Self::Result {
    self.within(&other)
  }
}

#[cfg(test)]
mod tests {
  use crate::testing::PointView;

  use crate::{
    traits::Within, Circle, Ellipse, Point2, Rectangle, Shape, ShapeCollection, Triangle,
  };

  use crate::proptest::ShapeView;
  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Shape::Rectangle(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10]))), Point2::from([5, 5]) => true; "rectangle center")]
  #[test_case(Shape::Circle(Circle::new(Point2::from([10, 10]), 5)), Point2::from([10, 10]) => true; "circle center")]
  #[test_case(Shape::Ellipse(Ellipse::new(Point2::from([10, 10]), (5, 5))), Point2::from([10, 10]) => true; "ellipse center")]
  #[test_case(Shape::Triangle(Triangle::new(Point2::from([0, 0]), Point2::from([10, 0]), Point2::from([0, 10]))), Point2::from([5, 5]) => true; "triangle center")]
  #[test_case(Shape::Collection(ShapeCollection::new(vec![
    Shape::Rectangle(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10]))),
    Shape::Circle(Circle::new(Point2::from([10, 10]), 5)),
  ])), Point2::from([5, 5]) => true; "collection center")]
  fn shape_within_u8(shape: Shape<u8, u8>, point: Point2<u8>) -> bool {
    shape.within(point)
  }

  #[proptest]
  fn shape_within_u8_fuzz(shape: ShapeView<u8, u8>, point: PointView<u8, 2>) {
    let shape: Shape<_, _> = shape.into();
    let _out = shape.within(&point.into());
  }
}

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

#[cfg_attr(
  feature = "serde-serialize",
  derive(serde::Serialize, serde::Deserialize)
)]
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
  use crate::{Circle, Ellipse, Point2, Rectangle, Shape, ShapeCollection, Triangle};

  #[test]
  fn test_from() {
    let rectangle = Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10]));
    assert!(matches!(
      Shape::<u8, u8>::from(rectangle),
      Shape::Rectangle(_)
    ));

    let circle = Circle::new(Point2::from([10, 10]), 5);
    assert!(matches!(Shape::<u8, u8>::from(circle), Shape::Circle(_)));

    let ellipse = Ellipse::new(Point2::from([10, 10]), (5, 5));
    assert!(matches!(Shape::<u8, u8>::from(ellipse), Shape::Ellipse(_)));

    let triangle = Triangle::new(
      Point2::from([0, 0]),
      Point2::from([10, 0]),
      Point2::from([0, 10]),
    );
    assert!(matches!(
      Shape::<u8, u8>::from(triangle),
      Shape::Triangle(_)
    ));

    let collection = ShapeCollection::new(vec![
      Shape::Rectangle(Rectangle::new(Point2::from([0, 0]), Point2::from([10, 10]))),
      Shape::Circle(Circle::new(Point2::from([10, 10]), 5)),
    ]);
    assert!(matches!(
      Shape::<u8, u8>::from(collection.clone()),
      Shape::Collection(_)
    ));
  }
}

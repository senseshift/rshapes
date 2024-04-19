use crate::traits::{BoundingBox, Within};
use crate::*;
use std::collections::HashSet;

pub trait PointsInside<T: Scalar> {
  fn points_inside(&self) -> Vec<Point2<T>>;
}

impl<T: Scalar, U: PointsInside<T>> PointsInside<T> for &U {
  #[cfg_attr(test, mutants::skip)]
  fn points_inside(&self) -> Vec<Point2<T>> {
    U::points_inside(*self)
  }
}

impl PointsInside<u8> for Shape<u8, u8> {
  fn points_inside(&self) -> Vec<Point2<u8>> {
    match self {
      Shape::Ellipse(ellipse) => ellipse.points_inside(),
      Shape::Circle(circle) => circle.points_inside(),
      Shape::Rectangle(rectangle) => rectangle.points_inside(),
      Shape::Triangle(triangle) => triangle.points_inside(),
      Shape::Collection(collection) => collection.points_inside(),
    }
  }
}

impl PointsInside<u8> for Circle<u8, u8> {
  fn points_inside(&self) -> Vec<Point2<u8>> {
    self
      .bbox()
      .points_inside()
      .into_iter()
      .filter(|point| self.within(point))
      .collect()
  }
}

impl PointsInside<u8> for Ellipse<u8, u8> {
  fn points_inside(&self) -> Vec<Point2<u8>> {
    self
      .bbox()
      .points_inside()
      .into_iter()
      .filter(|point| self.within(*point))
      .collect()
  }
}

impl PointsInside<u8> for Rectangle<u8> {
  /// Returns a vector of all points inside the rectangle.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Rectangle, traits::PointsInside};
  ///
  /// let rectangle = Rectangle::new(Point2::new(0, 0), Point2::new(2, 2));
  /// assert_eq!(rectangle.points_inside(), vec![
  ///   Point2::new(0, 0),
  ///   Point2::new(0, 1),
  ///   Point2::new(0, 2),
  ///   Point2::new(1, 0),
  ///   Point2::new(1, 1),
  ///   Point2::new(1, 2),
  ///   Point2::new(2, 0),
  ///   Point2::new(2, 1),
  ///   Point2::new(2, 2),
  /// ]);
  /// ```
  #[cfg_attr(test, mutants::skip)] // Prevent vector capacity from being mutated
  fn points_inside(&self) -> Vec<Point2<u8>> {
    let mut points = Vec::with_capacity(self.width() as usize * self.height() as usize);

    for x in self.min().x..self.max().x.saturating_add(1) {
      for y in self.min().y..self.max().y.saturating_add(1) {
        points.push(Point2::new(x, y));
      }
    }

    points
  }
}

impl PointsInside<u8> for Triangle<u8> {
  fn points_inside(&self) -> Vec<Point2<u8>> {
    self
      .bbox()
      .points_inside()
      .into_iter()
      .filter(|point| self.within(*point))
      .collect()
  }
}

impl PointsInside<u8> for ShapeCollection<u8, u8> {
  fn points_inside(&self) -> Vec<Point2<u8>> {
    let mut points = Vec::new();
    for geometry in &self.shapes {
      points.extend(geometry.points_inside());
    }
    points
      .into_iter()
      .collect::<HashSet<_>>()
      .into_iter()
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use test_strategy::proptest;

  #[test]
  fn circle_points_u8_inside() {
    let circle = Circle::new(Point2::new(5, 5), 2);
    let points = circle.points_inside();

    let expected: Vec<Point2<u8>> = vec![
      Point2::new(3, 5),
      Point2::new(4, 4),
      Point2::new(4, 5),
      Point2::new(4, 6),
      Point2::new(5, 3),
      Point2::new(5, 4),
      Point2::new(5, 5),
      Point2::new(5, 6),
      Point2::new(5, 7),
      Point2::new(6, 4),
      Point2::new(6, 5),
      Point2::new(6, 6),
      Point2::new(7, 5),
    ];

    assert_vec_eq!(points, expected);
  }

  #[test]
  fn circle_points_inside_u8_edge() {
    let circle = Circle::new(Point2::new(0, 0), 5);
    let points = circle.points_inside();

    let expected = vec![
      Point2::new(0, 0),
      Point2::new(0, 1),
      Point2::new(0, 2),
      Point2::new(0, 3),
      Point2::new(0, 4),
      Point2::new(0, 5),
      Point2::new(1, 0),
      Point2::new(1, 1),
      Point2::new(1, 2),
      Point2::new(1, 3),
      Point2::new(1, 4),
      Point2::new(2, 0),
      Point2::new(2, 1),
      Point2::new(2, 2),
      Point2::new(2, 3),
      Point2::new(2, 4),
      Point2::new(3, 0),
      Point2::new(3, 1),
      Point2::new(3, 2),
      Point2::new(3, 3),
      Point2::new(3, 4),
      Point2::new(4, 0),
      Point2::new(4, 1),
      Point2::new(4, 2),
      Point2::new(4, 3),
      Point2::new(5, 0),
    ];

    assert_vec_eq!(points, expected);
  }

  #[proptest]
  fn circle_points_inside_u8_fuzz(circle: Circle<u8, u8>) {
    let _out = circle.points_inside();
  }

  #[test]
  fn ellipse_points_inside_u8() {
    let ellipse = Ellipse::new(Point2::new(5, 5), (2, 1));
    let points = ellipse.points_inside();

    let expected = vec![
      Point2::new(3, 5),
      Point2::new(4, 5),
      Point2::new(5, 4),
      Point2::new(5, 5),
      Point2::new(5, 6),
      Point2::new(6, 5),
      Point2::new(7, 5),
    ];

    assert_vec_eq!(points, expected);
  }

  #[proptest]
  fn ellipse_points_inside_u8_fuzz(ellipse: Ellipse<u8, u8>) {
    let _out = ellipse.points_inside();
  }

  #[test]
  fn rectangle_points_inside_u8() {
    let rectangle = Rectangle::new(Point2::new(10, 10), Point2::new(13, 14));
    let points = rectangle.points_inside();

    let expected = vec![
      Point2::new(10, 10),
      Point2::new(10, 11),
      Point2::new(10, 12),
      Point2::new(10, 13),
      Point2::new(10, 14),
      Point2::new(11, 10),
      Point2::new(11, 11),
      Point2::new(11, 12),
      Point2::new(11, 13),
      Point2::new(11, 14),
      Point2::new(12, 10),
      Point2::new(12, 11),
      Point2::new(12, 12),
      Point2::new(12, 13),
      Point2::new(12, 14),
      Point2::new(13, 10),
      Point2::new(13, 11),
      Point2::new(13, 12),
      Point2::new(13, 13),
      Point2::new(13, 14),
    ];

    assert_vec_eq!(points, expected);
  }

  #[proptest]
  fn rectangle_points_inside_u8_fuzz(rectangle: Rectangle<u8>) {
    let _out = rectangle.points_inside();
  }

  #[test]
  fn triangle_points_inside_u8() {
    let triangle = Triangle::new(Point2::new(0, 0), Point2::new(2, 0), Point2::new(0, 2));
    let points = triangle.points_inside();

    let expected = vec![
      Point2::new(0, 0),
      Point2::new(0, 1),
      Point2::new(0, 2),
      Point2::new(1, 0),
      Point2::new(1, 1),
      Point2::new(2, 0),
    ];

    assert_vec_eq!(points, expected);
  }

  #[proptest]
  fn triangle_points_inside_u8_fuzz(triangle: Triangle<u8>) {
    let _points = triangle.points_inside();
  }

  #[test]
  fn shape_collection_points_inside_u8() {
    let collection = ShapeCollection::new(vec![
      Shape::Rectangle(Rectangle::new(Point2::new(0, 0), Point2::new(2, 2))),
      Shape::Rectangle(Rectangle::new(Point2::new(2, 2), Point2::new(4, 4))),
    ]);

    let points = collection.points_inside();

    let expected = vec![
      Point2::new(0, 0),
      Point2::new(0, 1),
      Point2::new(0, 2),
      Point2::new(1, 0),
      Point2::new(1, 1),
      Point2::new(1, 2),
      Point2::new(2, 0),
      Point2::new(2, 1),
      Point2::new(2, 2),
      Point2::new(2, 3),
      Point2::new(2, 4),
      Point2::new(3, 2),
      Point2::new(3, 3),
      Point2::new(3, 4),
      Point2::new(4, 2),
      Point2::new(4, 3),
      Point2::new(4, 4),
    ];

    assert_vec_eq_unordered!(points, expected);
  }
}

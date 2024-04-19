use crate::*;
use num::Unsigned;
use std::ops::Div;

pub trait Centroid<T: Scalar> {
  fn centroid(&self) -> Point2<T>;
}

impl<T: Scalar, U: Centroid<T>> Centroid<T> for &U {
  fn centroid(&self) -> Point2<T> {
    U::centroid(*self)
  }
}

impl Centroid<u8> for Shape<u8, u8> {
  fn centroid(&self) -> Point2<u8> {
    match self {
      Self::Ellipse(ellipse) => ellipse.centroid(),
      Self::Circle(circle) => circle.centroid(),
      Self::Rectangle(rectangle) => rectangle.centroid(),
      Self::Triangle(triangle) => triangle.centroid(),
      Self::Collection(collection) => collection.centroid(),
    }
  }
}

impl<T, R> Centroid<T> for Circle<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
{
  fn centroid(&self) -> Point2<T> {
    self.center.clone()
  }
}

impl<T, R> Centroid<T> for Ellipse<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
{
  fn centroid(&self) -> Point2<T> {
    self.center.clone()
  }
}

impl Centroid<u8> for Rectangle<u8> {
  /// Returns the center of the rectangle.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Rectangle, traits::Centroid};
  ///
  /// let rectangle = Rectangle::new(Point2::new(0, 0), Point2::new(10, 10));
  /// assert_eq!(rectangle.centroid(), Point2::new(5, 5));
  ///
  /// ```
  fn centroid(&self) -> Point2<u8> {
    let min = self.min().map(|x| x as u16);
    let max = self.max().map(|x| x as u16);

    Point2::new(((min.x + max.x) / 2) as u8, ((min.y + max.y) / 2) as u8)
  }
}

// impl<T, U> Centroid<T> for ShapeCollection<T, U>
//   where
//     T: Scalar + Eq + From<f64>,
//     U: Scalar + Unsigned + Eq,
//     Shape<T, U>: Centroid<T>,
//     f64: From<T>
// {
//   fn centroid(&self) -> Point2<T> {
//     let mut center = Vector2::new(0., 0.);
//     for geometry in &self.shapes {
//       center += geometry.centroid().coords.map(|x| x.into());
//     }
//     center.div(self.shapes.len() as f64).map(|x| x.into()).into()
//   }
// }

impl Centroid<u8> for Triangle<u8> {
  /// Returns the center of the triangle.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Triangle, traits::Centroid};
  ///
  /// let triangle = Triangle::new([0, 0].into(), [10, 0].into(), [0, 10].into());
  /// assert_eq!(triangle.centroid(), [3, 3].into());
  /// ```
  fn centroid(&self) -> Point2<u8> {
    let x_sum: u16 = self.0.x as u16 + self.1.x as u16 + self.2.x as u16;
    let y_sum: u16 = self.0.y as u16 + self.1.y as u16 + self.2.y as u16;

    Point2::new(
      (x_sum as f64 / 3.0).round() as u8,
      (y_sum as f64 / 3.0).round() as u8,
    )
  }
}

impl Centroid<u8> for ShapeCollection<u8, u8> {
  fn centroid(&self) -> Point2<u8> {
    let mut center = Vector2::new(0., 0.);
    for geometry in &self.shapes {
      center += geometry.centroid().coords.map(|x| x as f64);
    }
    center.div(self.shapes.len() as f64).map(|x| x as u8).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use test_case::test_case;
  use test_strategy::proptest;

  #[proptest]
  fn circle_centroid_u8_fuzz(circle: Circle<u8, u8>) {
    assert_eq!(circle.centroid(), circle.center);
  }

  #[proptest]
  fn ellipse_centroid_u8_fuzz(ellipse: Ellipse<u8, u8>) {
    assert_eq!(ellipse.centroid(), ellipse.center);
  }

  #[test_case(Rectangle::new(Point2::new(0, 0), Point2::new(10, 10)), Point2::new(5, 5); "normal")]
  fn rectangle_centroid_u8(rectangle: Rectangle<u8>, expected_center: Point2<u8>) {
    assert_eq!(rectangle.centroid(), expected_center);
  }

  #[proptest]
  fn rectangle_centroid_u8_fuzz(rectangle: Rectangle<u8>) {
    let _out = rectangle.centroid();
  }

  #[test_case(Triangle::new([0, 0].into(), [10, 0].into(), [0, 10].into()), [3, 3].into(); "normal")]
  fn triangle_centroid_u8(triangle: Triangle<u8>, centroid: Point2<u8>) {
    assert_eq!(triangle.centroid(), centroid);
  }

  #[proptest]
  fn triangle_centroid_u8_fuzz(triangle: Triangle<u8>) {
    let _centroid = triangle.centroid();
  }
}

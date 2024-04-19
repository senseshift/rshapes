use crate::*;
use nalgebra::{Scalar, Vector2};
use num::Num;

pub trait BoundingBox<T: Scalar> {
  fn bbox(&self) -> Rectangle<T>;
}

impl<T: Scalar, U: BoundingBox<T>> BoundingBox<T> for &U {
  fn bbox(&self) -> Rectangle<T> {
    U::bbox(*self)
  }
}

impl BoundingBox<u8> for Shape<u8, u8> {
  fn bbox(&self) -> Rectangle<u8> {
    match self {
      Self::Ellipse(ellipse) => ellipse.bbox(),
      Self::Circle(circle) => circle.bbox(),
      Self::Rectangle(rectangle) => rectangle.bbox(),
      Self::Triangle(triangle) => triangle.bbox(),
      Self::Collection(collection) => collection.bbox(),
    }
  }
}

impl BoundingBox<u8> for Ellipse<u8, u8> {
  fn bbox(&self) -> Rectangle<u8> {
    let x = self.center.x as f64 - self.radius.0 as f64;
    let y = self.center.y as f64 - self.radius.1 as f64;
    let width = self.radius.0 as f64 * 2.0 + 1.;
    let height = self.radius.1 as f64 * 2.0 + 1.;

    Rectangle::new(
      Point2::new(x, y).map(|c| c as u8),
      Point2::new(x + width, y + height).map(|c| c as u8),
    )
  }
}

impl BoundingBox<u8> for Circle<u8, u8> {
  fn bbox(&self) -> Rectangle<u8> {
    let radius = self.radius as i16;
    let center = self.center.map(|x| x as i16);

    let min = center - Vector2::new(radius, radius);
    let max = center + Vector2::new(radius, radius);

    Rectangle::new(
      min.map(|x| x.clamp(0, u8::MAX as i16) as u8),
      max.map(|x| x.clamp(0, u8::MAX as i16) as u8),
    )
  }
}

impl<T> BoundingBox<T> for Rectangle<T>
where
  T: Scalar + Num,
{
  fn bbox(&self) -> Rectangle<T> {
    self.clone()
  }
}

impl BoundingBox<u8> for Triangle<u8> {
  fn bbox(&self) -> Rectangle<u8> {
    let min_x = self.0.x.min(self.1.x).min(self.2.x);
    let min_y = self.0.y.min(self.1.y).min(self.2.y);
    let max_x = self.0.x.max(self.1.x).max(self.2.x);
    let max_y = self.0.y.max(self.1.y).max(self.2.y);

    Rectangle::new(
      Point2::new(min_x, min_y),
      Point2::new(max_x, max_y).map(|x| x.saturating_add(1)),
    )
  }
}

impl BoundingBox<u8> for ShapeCollection<u8, u8> {
  fn bbox(&self) -> Rectangle<u8> {
    let bboxes = self.shapes.iter().map(|x| x.bbox());

    let mut min = Point2::new(u8::MAX, u8::MAX);
    let mut max = Point2::new(u8::MIN, u8::MIN);

    for bbox in bboxes {
      min = Point2::new(min.x.min(bbox.min().x), min.y.min(bbox.min().y));
      max = Point2::new(max.x.max(bbox.max().x), max.y.max(bbox.max().y));
    }

    Rectangle::new(min, max)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Circle::new(Point2::new(12, 12 ), 10), Point2::new(2, 2  ), Point2::new(22, 22 ); "normal")]
  #[test_case(Circle::new(Point2::new(0, 0  ), 10), Point2::new(0, 0  ), Point2::new(10, 10 ); "edge/top+start")]
  #[test_case(Circle::new(Point2::new(255, 0  ), 10), Point2::new(245, 0  ), Point2::new(255, 10 ); "edge/top+end")]
  #[test_case(Circle::new(Point2::new(0, 255), 10), Point2::new(0, 245), Point2::new(10, 255); "edge/bottom+start")]
  #[test_case(Circle::new(Point2::new(255, 255), 10), Point2::new(245, 245), Point2::new(255, 255); "edge/bottom+end")]
  fn test_circle_bbox(circle: Circle<u8, u8>, min: Point2<u8>, max: Point2<u8>) {
    let bbox = circle.bbox();

    assert_eq!(bbox.min(), &min);
    assert_eq!(bbox.max(), &max);
  }

  #[proptest]
  fn ellipse_bbox_u8_fuzz(ellipse: Ellipse<u8, u8>) {
    let _out = ellipse.bbox();
  }

  #[proptest]
  fn circle_bbox_u8_fuzz(circle: Circle<u8, u8>) {
    let _out = circle.bbox();
  }

  #[proptest]
  fn triangle_bbox_u8_fuzz(triangle: Triangle<u8>) {
    let _bbox = triangle.bbox();
  }
}

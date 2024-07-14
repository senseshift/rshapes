use nalgebra::{Point2, Scalar};
use num::traits::Unsigned;

use crate::{
  traits::Distance, Circle, Ellipse, FloatMath, Line, Rectangle, Shape, ShapeCollection, Triangle,
};

/// Calculate the squared distance between two points.
///
/// # Example:
/// ```rust
/// use rshapes::{Point2, distance_squared};
///
/// let a = Point2::new(0u8, 0u8);
/// let b = Point2::new(3u8, 4u8);
///
/// assert_eq!(distance_squared(&a, &b), 25.0);
/// assert_eq!(distance_squared(&a, &b), distance_squared(&b, &a));
/// ```
pub fn distance_squared<T>(a: &Point2<T>, b: &Point2<T>) -> f64
where
  T: FloatMath,
{
  let x_max = if a.x > b.x { a.x } else { b.x };
  let x_min = if a.x < b.x { a.x } else { b.x };
  let y_max = if a.y > b.y { a.y } else { b.y };
  let y_min = if a.y < b.y { a.y } else { b.y };

  let x = x_max - x_min;
  let y = y_max - y_min;

  let x = Into::<f64>::into(x);
  let y = Into::<f64>::into(y);

  x * x + y * y
}

/// Calculate the distance between two points.
///
/// # Example:
/// ```rust
/// use rshapes::{Point2, traits::distance};
///
/// let a = Point2::new(0u8, 0u8);
/// let b = Point2::new(3u8, 4u8);
///
/// assert_eq!(distance(&a, &b), 5.0);
/// assert_eq!(distance(&a, &b), distance(&b, &a));
/// ```
#[inline]
pub fn distance<T>(a: &Point2<T>, b: &Point2<T>) -> f64
where
  T: FloatMath,
{
  distance_squared(a, b).sqrt()
}

impl<T> Distance<&Point2<T>> for Point2<T>
where
  T: FloatMath,
{
  type Result = f64;

  /// Calculate the distance between two points.
  ///
  /// # Example:
  /// ```rust
  /// use rshapes::{Point2, traits::Distance};
  ///
  /// let a = Point2::new(0u8, 0u8);
  /// let b = Point2::new(3u8, 4u8);
  ///
  /// assert_eq!(a.distance(&b), 5.0);
  /// ```
  #[inline]
  fn distance(&self, point: &Point2<T>) -> Self::Result {
    distance(self, point)
  }
}
impl<T> Distance<Point2<T>> for Point2<T>
where
  T: FloatMath,
{
  type Result = f64;

  #[inline]
  fn distance(&self, point: Point2<T>) -> Self::Result {
    self.distance(&point)
  }
}

impl Distance<&Point2<u8>> for Circle<u8, u8> {
  type Result = f64;

  /// Calculate the distance from the edge of the circle to the point.
  /// If the point is inside the circle, return 0.
  ///
  /// # Example:
  /// ```rust
  /// use rshapes::{Point2, Circle, traits::Distance};
  ///
  /// let circle = Circle::<u8, u8>::new([5, 5].into(), 10);
  /// let point = Point2::<u8>::new(20, 5);
  ///
  /// assert_eq!(circle.distance(&point), 5.0);
  /// ```
  fn distance(&self, point: &Point2<u8>) -> Self::Result {
    use crate::traits::Within;

    if self.within(point) {
      return 0.0;
    }

    let distance_to_center = distance(&self.center, point);

    distance_to_center - self.radius as f64
  }
}
impl Distance<Point2<u8>> for Circle<u8, u8> {
  type Result = f64;

  fn distance(&self, point: Point2<u8>) -> Self::Result {
    self.distance(&point)
  }
}

impl Distance<&Point2<u8>> for Ellipse<u8, u8> {
  type Result = f64;

  fn distance(&self, point: &Point2<u8>) -> f64 {
    use crate::traits::Within;

    if self.within(point) {
      return 0.0;
    }

    // optimize if point is on the same axis as the center
    if point.x == self.center.x {
      return distance(&self.center, point) - *self.height() as f64;
    } else if point.y == self.center.y {
      return distance(&self.center, point) - *self.width() as f64;
    }

    let point_on_ellipse = self.point_intersection(point, 10);
    distance(&point_on_ellipse, &point.map(|c| c as f64))
  }
}
impl Distance<Point2<u8>> for Ellipse<u8, u8> {
  type Result = f64;

  fn distance(&self, point: Point2<u8>) -> f64 {
    self.distance(&point)
  }
}

impl Distance<&Point2<u8>> for Line<u8> {
  type Result = f64;

  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Line, traits::Distance};
  ///
  /// let line = Line::new(Point2::new(5, 5), Point2::new(5, 10));
  ///
  /// assert_eq!(line.distance(&Point2::new(5, 7)), 0.0); // Point is on the line
  /// assert_eq!(line.distance(&Point2::new(5, 4)), 1.0); // Point is to the left of the line
  /// assert_eq!(line.distance(&Point2::new(5, 11)), 1.0); // Point is to the right of the line
  /// assert_eq!(line.distance(&Point2::new(6, 7)), 1.0); // Point is above the line
  /// assert_eq!(line.distance(&Point2::new(4, 7)), 1.0); // Point is below the line
  /// ```
  fn distance(&self, point: &Point2<u8>) -> f64 {
    let xy = point.map(|x| x as f64);
    let xy1 = self.start.map(|x| x as f64);
    let xy2 = self.end.map(|x| x as f64);

    let a = xy.x - xy1.x;
    let b = xy.y - xy1.y;
    let c = xy2.x - xy1.x;
    let d = xy2.y - xy1.y;

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;

    let param = if len_sq != 0.0 { dot / len_sq } else { -1.0 };

    let xx;
    let yy;

    if param < 0.0 {
      xx = xy1.x;
      yy = xy1.y;
    } else if param > 1.0 {
      xx = xy2.x;
      yy = xy2.y;
    } else {
      xx = xy1.x + param * c;
      yy = xy1.y + param * d;
    }

    distance(&xy, &Point2::new(xx, yy))
  }
}
impl Distance<Point2<u8>> for Line<u8> {
  type Result = f64;

  fn distance(&self, point: Point2<u8>) -> f64 {
    self.distance(&point)
  }
}

impl Distance<&Point2<u8>> for Rectangle<u8> {
  type Result = f64;

  /// Calculate the distance from the edge of the rectangle to the point.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Rectangle, traits::Distance};
  ///
  /// let rectangle = Rectangle::new(Point2::new(0, 0), Point2::new(10, 10));
  ///
  /// assert_eq!(rectangle.distance(&Point2::new(5, 5)), 0.0); // Point is inside the rectangle
  /// assert_eq!(rectangle.distance(&Point2::new(20, 10)), 10.0); // Point is to the right of the rectangle
  /// ```
  fn distance(&self, point: &Point2<u8>) -> f64 {
    use crate::traits::Within;
    if self.within(point) {
      return 0.0;
    }

    let top = Line::new(*self.min(), Point2::new(self.max().x, self.min().y));
    let right = Line::new(Point2::new(self.max().x, self.min().y), *self.max());
    let bottom = Line::new(*self.max(), Point2::new(self.min().x, self.max().y));
    let left = Line::new(Point2::new(self.min().x, self.max().y), *self.min());

    let distances = [
      top.distance(point),
      right.distance(point),
      bottom.distance(point),
      left.distance(point),
    ];

    distances.iter().copied().fold(f64::MAX, f64::min)
  }
}
impl Distance<Point2<u8>> for Rectangle<u8> {
  type Result = f64;

  fn distance(&self, point: Point2<u8>) -> f64 {
    self.distance(&point)
  }
}

impl Distance<&Point2<u8>> for Triangle<u8> {
  type Result = f64;

  /// Calculate the distance from the edge of the triangle to the point.
  ///
  /// # Example
  /// ```rust
  /// use rshapes::{Point2, Triangle, traits::Distance};
  ///
  /// let triangle = Triangle::new(
  ///   Point2::new(0, 0),
  ///   Point2::new(10, 0),
  ///   Point2::new(0, 10),
  /// );
  ///
  /// assert_eq!(triangle.distance(&Point2::new(5, 5)), 0.0); // Point is inside the triangle
  /// assert_eq!(triangle.distance(&Point2::new(20, 10)).round(), 14.0); // Point is to the right of the triangle
  /// ```
  fn distance(&self, point: &Point2<u8>) -> f64 {
    // use crate::{Within, PointWithin};
    // if self.within(point) == true {
    //   return 0.0;
    // }

    let a = Line::new(self.0, self.1);
    let b = Line::new(self.1, self.2);
    let c = Line::new(self.2, self.0);

    let distances = [a.distance(point), b.distance(point), c.distance(point)];

    distances.iter().copied().fold(f64::MAX, f64::min)
  }
}
impl Distance<Point2<u8>> for Triangle<u8> {
  type Result = f64;

  fn distance(&self, point: Point2<u8>) -> f64 {
    self.distance(&point)
  }
}

impl<T, R> Distance<&Point2<T>> for ShapeCollection<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
  Shape<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
{
  type Result = f64;

  fn distance(&self, point: &Point2<T>) -> f64 {
    let mut distances = Vec::new();

    for shape in self.shapes.iter() {
      let distance = shape.distance(point);
      distances.push(distance);
    }

    distances.iter().copied().fold(f64::MAX, f64::min)
  }
}

impl<T, R> Distance<Point2<T>> for ShapeCollection<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
  Shape<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
{
  type Result = f64;

  fn distance(&self, point: Point2<T>) -> f64 {
    self.distance(&point)
  }
}

impl<T, R> Distance<&Point2<T>> for Shape<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
  Ellipse<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  Circle<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  Rectangle<T>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  Triangle<T>: for<'a> Distance<&'a Point2<T>, Result = f64>,
{
  type Result = f64;

  fn distance(&self, point: &Point2<T>) -> f64 {
    match self {
      Self::Ellipse(ellipse) => ellipse.distance(point),
      Self::Circle(circle) => circle.distance(point),
      Self::Rectangle(rectangle) => rectangle.distance(point),
      Self::Triangle(triangle) => triangle.distance(point),
      Self::Collection(collection) => collection.distance(point),
    }
  }
}
impl<T, R> Distance<Point2<T>> for Shape<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
  Ellipse<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  Circle<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  Rectangle<T>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  Triangle<T>: for<'a> Distance<&'a Point2<T>, Result = f64>,
  ShapeCollection<T, R>: for<'a> Distance<&'a Point2<T>, Result = f64>,
{
  type Result = f64;

  fn distance(&self, point: Point2<T>) -> f64 {
    self.distance(&point)
  }
}

#[cfg(test)]
mod tests {
  use crate::testing::PointView;
  use crate::{
    distance, distance_squared, traits::Distance, Circle, Ellipse, Line, Point2, Rectangle, Shape,
    Triangle,
  };
  use float_cmp::assert_approx_eq;
  use test_case::test_case;
  use test_strategy::proptest;

  #[test_case(Point2::new(0, 0), Point2::new(3, 4) => 25)]
  #[test_case(Point2::new(0, 0), Point2::new(0, 0) => 0)]
  #[test_case(Point2::new(0, 0), Point2::new(1, 1) => 2)]
  #[test_case(Point2::new(0, 0), Point2::new(1, 0) => 1)]
  #[test_case(Point2::new(0, 0), Point2::new(0, 1) => 1)]
  #[test_case(Point2::new(0, 0), Point2::new(255, 255) => 130050)]
  fn distance_squared_u8(a: Point2<u8>, b: Point2<u8>) -> u32 {
    distance_squared(&a, &b) as u32
  }

  #[proptest]
  fn distance_squared_u8_fuzz(a: PointView<u8, 2>, b: PointView<u8, 2>) {
    let _out = distance_squared(&a.into(), &b.into());
  }

  #[proptest]
  fn distance_squared_u16_fuzz(a: PointView<u16, 2>, b: PointView<u16, 2>) {
    let _out = distance_squared(&a.into(), &b.into());
  }

  #[test_case(Point2::new(0, 0), Point2::new(3, 4) => 5)]
  #[test_case(Point2::new(0, 0), Point2::new(0, 0) => 0)]
  #[test_case(Point2::new(0, 0), Point2::new(1, 1) => 1)]
  #[test_case(Point2::new(0, 0), Point2::new(1, 0) => 1)]
  #[test_case(Point2::new(0, 0), Point2::new(0, 1) => 1)]
  #[test_case(Point2::new(0, 0), Point2::new(255, 255) => 360)]
  fn distance_u8(a: Point2<u8>, b: Point2<u8>) -> u32 {
    distance(&a, &b) as u32
  }

  #[proptest]
  fn distance_u8_fuzz(a: PointView<u8, 2>, b: PointView<u8, 2>) {
    let _out = distance(&a.into(), &b.into());
  }

  #[test_case(Circle::<u8, u8>::new([5, 5].into(), 10), Point2::new(5, 5), 0.0f64; "point in the circle")]
  #[test_case(Circle::<u8, u8>::new([5, 5].into(), 10), Point2::new(5, 15), 0.0f64; "point on the edge")]
  #[test_case(Circle::<u8, u8>::new([5, 5].into(), 10), Point2::new(20, 5), 5.0f64; "point on side center")]
  #[test_case(Circle::<u8, u8>::new([5, 5].into(), 10), Point2::new(20, 20), 11.213f64; "point outside")]
  fn circle_distance_u8(circle: Circle<u8, u8>, point: Point2<u8>, expected: f64) {
    assert_approx_eq!(f64, circle.distance(point), expected, epsilon = 0.001);
  }

  #[proptest]
  fn circle_distance_u8_fuzz(circle: Circle<u8, u8>, point: PointView<u8, 2>) {
    let _out = circle.distance(point.0);
  }

  #[test_case(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5)), Point2::new(5, 5), 0.0f64; "point in the ellipse")]
  #[test_case(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5)), Point2::new(5, 10), 0.0f64; "point on the edge / center - x axis")]
  #[test_case(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5)), Point2::new(15, 5), 0.0f64; "point on the edge / center - y axis")]
  #[test_case(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5)), Point2::new(5, 15), 5.0f64; "point to the right of the ellipse center")]
  #[test_case(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5)), Point2::new(20, 5), 5.0f64; "point to the bottom of the ellipse center")]
  #[test_case(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5)), Point2::new(15, 10), 2.788f64; "point on the bbox corner ")]
  fn ellipse_distance_u8(ellipse: Ellipse<u8, u8>, point: Point2<u8>, expected: f64) {
    assert_approx_eq!(f64, ellipse.distance(point), expected, epsilon = 0.001);
  }

  #[proptest]
  fn ellipse_distance_u8_fuzz(ellipse: Ellipse<u8, u8>, point: PointView<u8, 2>) {
    let _out = ellipse.distance(&point.into());
  }

  #[test_case(Line::new(Point2::new(5, 5), Point2::new(5, 10)), Point2::new(5, 7), 0.0f64; "point on the straight line")]
  #[test_case(Line::new(Point2::new(5, 5), Point2::new(5, 10)), Point2::new(5, 4), 1.0f64; "point to the left of the straight line")]
  #[test_case(Line::new(Point2::new(5, 5), Point2::new(5, 10)), Point2::new(5, 11), 1.0f64; "point to the right of the straight line")]
  #[test_case(Line::new(Point2::new(5, 5), Point2::new(5, 10)), Point2::new(6, 7), 1.0f64; "point above the straight line")]
  #[test_case(Line::new(Point2::new(5, 5), Point2::new(5, 10)), Point2::new(4, 7), 1.0f64; "point below the straight line")]
  #[test_case(Line::new(Point2::new(5, 5), Point2::new(10, 10)), Point2::new(7, 7), 0.0f64; "point on the diagonal line")]
  fn line_distance_u8(line: Line<u8>, point: Point2<u8>, expected: f64) {
    assert_approx_eq!(f64, line.distance(point), expected, epsilon = 0.001);
  }

  #[proptest]
  fn line_distance_u8_fuzz(line: Line<u8>, point: PointView<u8, 2>) {
    let _out = line.distance(&point.into());
  }

  #[test_case(Rectangle::new(Point2::new(0, 0), Point2::new(10, 10)), Point2::new(5, 5), 0.0f64; "point in the rectangle")]
  #[test_case(Rectangle::new(Point2::new(0, 0), Point2::new(10, 10)), Point2::new(20, 10), 10.0f64; "point to the right of the rectangle")]
  fn rectangle_distance_u8(rectangle: Rectangle<u8>, point: Point2<u8>, expected: f64) {
    assert_approx_eq!(f64, rectangle.distance(point), expected, epsilon = 0.001);
  }

  #[proptest]
  fn rectangle_distance_u8_fuzz(rectangle: Rectangle<u8>, point: PointView<u8, 2>) {
    let _out = rectangle.distance(&point.into());
  }

  #[test_case(Triangle::new(Point2::new(0, 0), Point2::new(10, 0), Point2::new(0, 10)), Point2::new(5, 5), 0.0f64; "point in the triangle")]
  #[test_case(Triangle::new(Point2::new(0, 0), Point2::new(10, 0), Point2::new(0, 10)), Point2::new(20, 10), 14.142f64; "point to the right of the triangle")]
  fn triangle_distance_u8(triangle: Triangle<u8>, point: Point2<u8>, expected: f64) {
    assert_approx_eq!(f64, triangle.distance(point), expected, epsilon = 0.001);
  }

  #[proptest]
  fn triangle_distance_u8_fuzz(triangle: Triangle<u8>, point: PointView<u8, 2>) {
    let _out = triangle.distance(&point.into());
  }

  #[test]
  fn shape_collection_distance_u8() {
    let shapes = vec![
      Shape::Circle(Circle::<u8, u8>::new([5, 5].into(), 10)),
      Shape::Ellipse(Ellipse::<u8, u8>::new([5, 5].into(), (10, 5))),
      Shape::Rectangle(Rectangle::new(Point2::new(0, 0), Point2::new(10, 10))),
      Shape::Triangle(Triangle::new(
        Point2::new(0, 0),
        Point2::new(10, 0),
        Point2::new(0, 10),
      )),
    ];

    let collection = crate::ShapeCollection::new(shapes);

    assert_approx_eq!(f64, collection.distance(&Point2::new(5, 5)), 0.0);
    assert_approx_eq!(f64, collection.distance(&Point2::new(5, 15)), 0.0);
    assert_approx_eq!(f64, collection.distance(&Point2::new(20, 5)), 5.0);
    assert_approx_eq!(
      f64,
      collection.distance(&Point2::new(15, 10)),
      1.180,
      epsilon = 0.001
    );
  }
}

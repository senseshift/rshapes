use nalgebra::Scalar;
use num::Unsigned;

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use super::Shape;

pub struct ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
{
  pub shapes: Vec<Shape<T, U>>,
}

impl<T, U> Default for ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
{
  fn default() -> Self {
    Self {
      shapes: Vec::default(),
    }
  }
}

impl<T, U> Debug for ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
  Shape<T, U>: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ShapeCollection")
      .field("geometry", &self.shapes)
      .finish()
  }
}

impl<T, U> ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
{
  pub fn new(geometry: Vec<Shape<T, U>>) -> Self {
    Self { shapes: geometry }
  }
}

impl<T, U> Hash for ShapeCollection<T, U>
where
  T: Scalar + Hash,
  U: Scalar + Unsigned + Hash,
  Shape<T, U>: Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.shapes.hash(state);
  }
}

impl<T, U> Clone for ShapeCollection<T, U>
where
  T: Scalar + Clone,
  U: Scalar + Unsigned + Clone,
  Shape<T, U>: Clone,
{
  fn clone(&self) -> Self {
    Self {
      shapes: self.shapes.clone(),
    }
  }
}

impl<T, U> PartialEq for ShapeCollection<T, U>
where
  T: Scalar + PartialEq,
  U: Scalar + Unsigned + PartialEq,
  Shape<T, U>: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.shapes == other.shapes
  }
}

impl<T, U> Eq for ShapeCollection<T, U>
where
  T: Scalar + Eq,
  U: Scalar + Unsigned + Eq,
  Shape<T, U>: Eq,
{
}

#[cfg(test)]
mod tests {}

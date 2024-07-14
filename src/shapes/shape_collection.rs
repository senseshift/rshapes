use nalgebra::Scalar;
use num::Unsigned;

use super::Shape;
use derivative::Derivative;
use getset::Getters;

#[cfg_attr(
  feature = "serde-serialize",
  derive(serde::Serialize, serde::Deserialize),
  serde(transparent)
)]
#[derive(Derivative, Getters)]
#[derivative(Debug, Default, Hash, Clone, PartialEq, Eq)]
pub struct ShapeCollection<T, U>
where
  T: Scalar,
  U: Scalar + Unsigned,
{
  #[getset(get = "pub")]
  #[derivative(Default(value = "Vec::new()"))]
  pub(crate) shapes: Vec<Shape<T, U>>,
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

#[cfg(test)]
mod tests {}

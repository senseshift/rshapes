use derivative::Derivative;

use crate::*;

#[cfg_attr(
  feature = "serde-serialize",
  derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Derivative)]
#[derivative(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Triangle<T: Scalar>(pub Point2<T>, pub Point2<T>, pub Point2<T>);

impl<T> Triangle<T>
where
  T: Scalar,
{
  #[inline]
  pub fn new(a: Point2<T>, b: Point2<T>, c: Point2<T>) -> Self {
    Self(a, b, c)
  }
}

#[cfg(test)]
mod tests {}

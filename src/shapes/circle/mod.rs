use crate::*;

use num::Unsigned;

use derivative::Derivative;
use getset::Getters;

#[derive(Getters, Derivative)]
#[derivative(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Circle<T: Scalar, R: Scalar + Unsigned> {
  #[getset(get = "pub")]
  pub(crate) center: Point2<T>,
  #[getset(get = "pub")]
  pub(crate) radius: R,
}

impl<T, R> Circle<T, R>
where
  T: Scalar,
  R: Scalar + Unsigned,
{
  #[inline]
  pub fn new(center: Point2<T>, radius: R) -> Self {
    Self { center, radius }
  }
}

#[cfg(test)]
mod tests {}

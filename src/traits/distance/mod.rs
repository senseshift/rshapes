pub trait Distance<T> {
  type Result;

  fn distance(&self, other: T) -> Self::Result;
}

mod to_circle;
mod to_point;

pub use to_point::*;

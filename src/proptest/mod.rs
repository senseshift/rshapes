#![allow(dead_code)]

use array_init::{array_init, try_array_init};
use nalgebra::{Point, Scalar};
use num::Unsigned;
use std::fmt::Debug;

use crate::{Circle, Ellipse, Line, Rectangle, Shape, Triangle};
use proptest::arbitrary::{Arbitrary, StrategyFor};
use proptest::collection::vec;
use proptest::prelude::any_with;
use proptest::strategy::{Map, NewTree, Strategy, ValueTree};
use proptest::test_runner::TestRunner;

type Mapped<I, O> = Map<StrategyFor<I>, fn(_: I) -> O>;

///////////////////////////////////////////////////////////////////////////////
// Shrinkable point

#[derive(Clone)]
pub struct ShrinkablePoint<T, const N: usize>
where
  T: Scalar + Debug,
{
  point: Point<T, N>,
  shrink: usize,
  prev_shrink: Option<usize>,
}

impl<T, const N: usize> ValueTree for ShrinkablePoint<T, N>
where
  T: Scalar + Debug + ValueTree,
  <T as ValueTree>::Value: Clone + PartialEq,
{
  type Value = Point<<T as ValueTree>::Value, N>;

  fn current(&self) -> Point<T::Value, N> {
    Point::from(array_init(|i| self.point.coords.index(i).current()))
  }

  fn simplify(&mut self) -> bool {
    for ix in self.shrink..N {
      if !self.point.coords.index_mut(ix).simplify() {
        self.shrink = ix + 1;
      } else {
        self.prev_shrink = Some(ix);
        return true;
      }
    }
    false
  }
  fn complicate(&mut self) -> bool {
    match self.prev_shrink {
      None => false,
      Some(ix) => {
        if self.point.coords.index_mut(ix).complicate() {
          true
        } else {
          self.prev_shrink = None;
          false
        }
      }
    }
  }
}

///////////////////////////////////////////////////////////////////////////////
// Point strategy

pub struct PointView<T: Scalar, const N: usize>(pub Point<T, N>);

pub type PointView2<T> = PointView<T, 2>;

impl<T, const N: usize> Debug for PointView<T, N>
where
  T: Scalar + Debug + Clone + PartialEq,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("PointView").field(&self.0).finish()
  }
}

impl<T, const N: usize> Strategy for PointView<T, N>
where
  T: Scalar + Clone + Debug + Strategy,
  <T as Strategy>::Tree: Clone + Debug + PartialEq,
  <T as Strategy>::Value: Clone + PartialEq,
{
  type Tree = ShrinkablePoint<T::Tree, N>;
  type Value = Point<<T as Strategy>::Value, N>;
  fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
    let tree = ShrinkablePoint {
      point: Point::from(try_array_init(|i| self.0.coords.index(i).new_tree(runner))?),
      shrink: 0,
      prev_shrink: None,
    };

    Ok(tree)
  }
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Point

impl<T, const N: usize> From<PointView<T, N>> for Point<T, N>
where
  T: Scalar + Debug + Clone + PartialEq,
{
  fn from(val: PointView<T, N>) -> Self {
    val.0
  }
}

impl<T, const N: usize> From<[T; N]> for PointView<T, N>
where
  T: Scalar + Debug + Clone + PartialEq,
{
  fn from(array: [T; N]) -> Self {
    Self(Point::from(array))
  }
}

impl<T> Arbitrary for PointView<T, 2>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Clone,
{
  type Parameters = T::Parameters;
  fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
    vec(any_with::<T>(params), 2).prop_map(|vec: Vec<T>| [vec[0].clone(), vec[1].clone()].into())
  }
  type Strategy = Mapped<Vec<T>, PointView<T, 2>>;
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Circle

impl<T, R> Arbitrary for Circle<T, R>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Clone,
  <R as Arbitrary>::Strategy: Clone,
  <R as Arbitrary>::Parameters: Clone,
  R: Arbitrary + Scalar + Unsigned + Clone,
{
  type Parameters = <(T, R) as Arbitrary>::Parameters;
  fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
    any_with::<(PointView<T, 2>, R)>((params.0.clone(), params.1.clone())).prop_map(
      |(center, radius)| Circle {
        center: center.into(),
        radius,
      },
    )
  }
  type Strategy = Mapped<(PointView<T, 2>, R), Circle<T, R>>;
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Ellipse

impl<T, R> Arbitrary for Ellipse<T, R>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Clone,
  <R as Arbitrary>::Strategy: Clone,
  <R as Arbitrary>::Parameters: Clone,
  R: Arbitrary + Scalar + Unsigned + Clone,
{
  type Parameters = <(T, R) as Arbitrary>::Parameters;
  fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
    any_with::<(PointView<T, 2>, (R, R))>((params.0.clone(), (params.1.clone(), params.1.clone())))
      .prop_map(|(center, radius)| Ellipse {
        center: center.into(),
        radius,
      })
  }
  type Strategy = Mapped<(PointView<T, 2>, (R, R)), Ellipse<T, R>>;
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Line

impl<T> Arbitrary for Line<T>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Clone + PartialOrd,
{
  type Parameters = <(PointView<T, 2>, PointView<T, 2>) as Arbitrary>::Parameters;
  fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
    any_with::<(PointView<T, 2>, PointView<T, 2>)>(params)
      .prop_map(|(p1, p2)| Line::new(p1.into(), p2.into()))
  }
  type Strategy = Mapped<(PointView<T, 2>, PointView<T, 2>), Line<T>>;
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Rectangle

impl<T> Arbitrary for Rectangle<T>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Copy + Ord,
{
  type Parameters = <(PointView<T, 2>, PointView<T, 2>) as Arbitrary>::Parameters;
  fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
    any_with::<(PointView<T, 2>, PointView<T, 2>)>(params)
      .prop_map(|(p1, p2)| Rectangle::new(p1.into(), p2.into()))
  }
  type Strategy = Mapped<(PointView<T, 2>, PointView<T, 2>), Rectangle<T>>;
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Triangle

impl<T> Arbitrary for Triangle<T>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Copy + Ord,
{
  type Parameters = <(PointView<T, 2>, PointView<T, 2>, PointView<T, 2>) as Arbitrary>::Parameters;
  fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
    any_with::<(PointView<T, 2>, PointView<T, 2>, PointView<T, 2>)>(params)
      .prop_map(|(p1, p2, p3)| Triangle::new(p1.into(), p2.into(), p3.into()))
  }
  type Strategy = Mapped<(PointView<T, 2>, PointView<T, 2>, PointView<T, 2>), Triangle<T>>;
}

///////////////////////////////////////////////////////////////////////////////
// Arbitrary Shape

#[derive(proptest_derive::Arbitrary, Debug)]
pub enum ShapeView<T, R>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Copy + Ord,
  <R as Arbitrary>::Strategy: Clone,
  <R as Arbitrary>::Parameters: Clone,
  R: Arbitrary + Scalar + Unsigned + Copy + Ord,
{
  Rectangle(Rectangle<T>),
  Circle(Circle<T, R>),
  Ellipse(Ellipse<T, R>),
  Triangle(Triangle<T>),
}

impl<T, R> From<ShapeView<T, R>> for Shape<T, R>
where
  <T as Arbitrary>::Strategy: Clone,
  <T as Arbitrary>::Parameters: Clone,
  T: Arbitrary + Scalar + Copy + Ord,
  <R as Arbitrary>::Strategy: Clone,
  <R as Arbitrary>::Parameters: Clone,
  R: Arbitrary + Scalar + Unsigned + Copy + Ord,
{
  fn from(val: ShapeView<T, R>) -> Self {
    match val {
      ShapeView::Rectangle(rectangle) => Shape::Rectangle(rectangle),
      ShapeView::Circle(circle) => Shape::Circle(circle),
      ShapeView::Ellipse(ellipse) => Shape::Ellipse(ellipse),
      ShapeView::Triangle(triangle) => Shape::Triangle(triangle),
    }
  }
}

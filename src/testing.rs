#[cfg(feature = "proptest-support")]
pub use crate::proptest::*;

#[macro_export]
macro_rules! assert_vec_eq {
  ($left:expr, $right:expr) => {
    // compare debug representations
    assert_eq!(format!("{:?}", $left), format!("{:?}", $right));
  };
}

#[macro_export]
macro_rules! assert_vec_eq_unordered {
  ($left:expr, $right:expr) => {
    $left.iter().for_each(|a| {
      assert!(
        $right.contains(a),
        "Expected {:?} to contain {:?}",
        $right,
        a
      );
    });
    $right.iter().for_each(|a| {
      assert!($left.contains(a), "Expected {:?} to contain {:?}", $left, a);
    });
    assert_eq!($left.len(), $right.len());
  };
}

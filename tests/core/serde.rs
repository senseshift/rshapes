#![cfg(feature = "serde-serialize")]

use rshapes::{Circle, Ellipse, Rectangle, Triangle};

use test_strategy::proptest;

macro_rules! test_serde(
    ($($test: ident, $ty: ident $(<$($gen: ident),+>)?);* $(;)*) => {$(
        #[proptest]
        fn $test(v: $ty $(<$($gen),+>)?) {
            let serialized = serde_json::to_string(&v).unwrap();
            let deserialized: $ty $(<$($gen),+>)? = serde_json::from_str(&serialized).unwrap();
            assert_eq!(v, deserialized);
        }
    )*}
);

test_serde!(
  serde_circle, Circle<u8, u8>;
  serde_ellipse, Ellipse<u8, u8>;
  serde_rectangle, Rectangle<u8>;
  serde_triangle, Triangle<u8>;
);

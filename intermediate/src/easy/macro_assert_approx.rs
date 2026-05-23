/*
  Problem 58: Macro — assert_approx_eq!

  Write a declarative macro assert_approx_eq! that takes two f64 expressions
  and an optional epsilon (default 1e-10). It should panic if the values
  differ by more than epsilon.

  Run the tests for this problem with:
    cargo test --test macro_assert_approx_test
*/

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        assert_approx_eq!($a, $b, 1e-10f64)
    };
    ($a:expr, $b:expr, $eps:expr) => {
        let diff = ($a as f64 - $b as f64).abs();
        if diff > $eps as f64 {
            panic!(
                "assertion failed: `|{} - {}| = {}` is not <= epsilon `{}`",
                $a, $b, diff, $eps
            );
        }
    };
}
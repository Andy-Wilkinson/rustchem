pub fn assert_f64_eq(left: f64, right: f64) {
    assert!(
        (left - right).abs() < f64::EPSILON,
        "assertion failed: `(left == right)`\n    left: `{:.?}`,\n   right: `{:.?}`",
        left,
        right
    );
}

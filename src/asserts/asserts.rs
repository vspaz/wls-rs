pub fn assert_almost_equal(expected: f64, actual: f64, delta: f64) {
    assert!(delta > expected - actual);
}

pub fn assert_true(is_true: bool) {
    assert!(is_true)
}
pub fn assert_almost_equal(expected: f64, actual: f64, delta: f64) {
    assert!(delta > expected - actual);
}

#[allow(dead_code)]
pub fn assert_true(condition: bool) {
    assert!(condition)
}

pub fn assert_have_same_size(size_one: usize, size_two: usize) {
    assert_eq!(size_one, size_two)
}

pub fn assert_have_size_greater_than_two(size_one: usize) {
    assert!(size_one >= 2)
}
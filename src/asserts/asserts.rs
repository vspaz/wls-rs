pub fn assert_almost_equal(expected: f64, actual: f64, delta: f64) {
    assert!(delta > expected - actual);
}

#[allow(dead_code)]
pub fn assert_true(condition: bool) {
    assert!(condition)
}

pub fn assert_have_same_size(one: &Vec<f64>, two: &Vec<f64>) {
    assert_eq!(one.len(), two.len())
}

pub fn assert_have_size_greater_than_two(one: &Vec<f64>) {
    assert!(one.len() >= 2)
}
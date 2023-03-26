pub fn assert_almost_equal(expected: f64, actual: f64, delta: f64) {
    assert!(delta > expected - actual);
}

#[allow(dead_code)]
pub fn assert_true(condition: bool) {
    assert!(condition)
}

pub fn assert_have_same_size(items_1: &Vec<f64>, items_2: &Vec<f64>) {
    assert_eq!(items_1.len(), items_2.len())
}

pub fn assert_have_size_greater_than_two(items: &Vec<f64>) {
    assert!(items.len() >= 2)
}

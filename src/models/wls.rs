pub struct Wls {
    x: Vec<f64>,
    y: Vec<f64>,
    w: Vec<f64>,
}

impl Wls {
    fn populate_weights(capacity: u64, value: f64) -> Vec<f64> {
        vec![value; capacity as usize]
    }

    fn assert_have_same_size(size_one: u64, size_two: u64) {
        assert_eq!(size_one, size_two)
    }

    fn assert_have_size_greater_than_two(size_one: u64) {
        assert!(size_one > 2)
    }
}

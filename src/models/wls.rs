pub struct Wls {
    x: Vec<f64>,
    y: Vec<f64>,
    w: Vec<f64>,
}

impl Wls {
    fn populate_weights(capacity: usize, value: f64) -> Vec<f64> {
        vec![value; capacity]
    }

    fn assert_have_same_size(size_one: usize, size_two: usize) {
        assert_eq!(size_one, size_two)
    }
}

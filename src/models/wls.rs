pub struct Wls {
    x: Vec<f64>,
    y: Vec<f64>,
    w: Vec<f64>,
}

fn populate_weights(capacity: usize, value: f64) -> Vec<f64> {
    vec![value; capacity]
}

fn assert_have_same_size(size_one: usize, size_two: usize) {
    assert_eq!(size_one, size_two)
}

fn assert_have_size_greater_than_two(size_one: usize) {
    assert!(size_one > 2)
}

pub fn new(x: Vec<f64>, y: Vec<f64>, w: Option<Vec<f64>>) -> Wls {
    assert_have_same_size(x.len(), y.len());
    if w.is_some() {
        assert_have_same_size(x.len(), w.len());
    }
    assert_have_size_greater_than_two(x.len());
    return Wls {
        x,
        y,
        w: w.or_else(populate_weights(x.len(), 1.0)).unwrap(),
    };
}

impl Wls {}

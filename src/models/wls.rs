pub struct Wls {
    x: Vec<f64>,
    y: Vec<f64>,
    w: Vec<f64>,
}

impl Wls {
    pub fn populate_weights(capacity: usize, value: f64) -> Vec<f64> {
        return  vec![value; capacity];
    }
}
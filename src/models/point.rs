pub struct Point {
    intercept: f64,
    slope: f64,
}

impl Point {
    pub fn new(intercept: f64, slope: f64) -> Point {
        return Point { intercept, slope };
    }

    pub fn get_intercept(&self) -> f64 {
        return self.intercept;
    }
}

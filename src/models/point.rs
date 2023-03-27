pub struct Point {
    intercept: f64,
    slope: f64,
}

impl Point {
    pub fn new(intercept: f64, slope: f64) -> Self {
        Self { intercept, slope }
    }

    pub fn get_intercept(&self) -> f64 {
        self.intercept
    }

    pub fn get_slope(&self) -> f64 {
        self.slope
    }
}

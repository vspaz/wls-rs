pub struct Point {
    intercept: f64,
    slope: f64,
}

impl Point {
    pub fn new(intercept: f64, slope: f64) -> Point {
        return Point { intercept, slope };
    }
}

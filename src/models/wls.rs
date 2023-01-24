use crate::models::point::Point;

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
    let x_size = x.len().to_owned();
    let y_size = x.len().to_owned();
    let mut weights: Vec<f64> = vec![];

    assert_have_same_size(x_size, y_size);
    if w.is_some() {
        weights = w.unwrap();
        assert_have_same_size(x.len(), weights.len());
    }
    assert_have_size_greater_than_two(x.len());
    let size = x.len().to_owned();
    if weights.is_empty() {
        weights = populate_weights(size, 1.0);
    }
    Wls { x, y, w: weights }
}

impl Wls {
    pub fn fit_linear_regression(&self) -> Option<Point> {
        let mut sum_of_weights: f64 = 0.0;
        let mut sum_of_weights_by_x_squared: f64 = 0.0;
        let mut sum_of_x_by_y_by_weights: f64 = 0.0;
        let mut sum_of_x_by_weights: f64 = 0.0;
        let mut sum_of_y_by_weights: f64 = 0.0;

        let mut x_i: f64 = 0.0;
        let mut y_i: f64 = 0.0;
        let mut w_i: f64 = 0.0;
        let mut x_i_by_w_i = 0.0;

        for i in 0..self.w.len() {
            x_i = self.x[i];
            y_i = self.y[i];
            w_i = self.w[i];

            sum_of_weights += w_i;
            x_i_by_w_i = x_i * w_i;
            sum_of_x_by_weights += x_i_by_w_i;
            sum_of_x_by_y_by_weights += x_i_by_w_i * y_i;
            sum_of_y_by_weights += y_i * w_i;
            sum_of_weights_by_x_squared += x_i_by_w_i * x_i;
        }

        let dividend =
            sum_of_weights * sum_of_x_by_y_by_weights - sum_of_x_by_weights * sum_of_y_by_weights;
        let divisor = sum_of_weights * sum_of_weights_by_x_squared
            - sum_of_x_by_weights * sum_of_x_by_weights;
        if divisor == 0.0 {
            return None;
        }
        let slope = dividend / divisor;
        let intercept = (sum_of_y_by_weights - slope * sum_of_x_by_weights) / sum_of_weights;
        Some(Point::new(intercept, slope))
    }
}

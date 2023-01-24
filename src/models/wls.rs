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

impl Wls {
    pub fn new(x: Vec<f64>, y: Vec<f64>, w: Option<Vec<f64>>) -> Wls {
        let x_size = x.len().to_owned();
        let y_size = x.len().to_owned();
        let mut weights: Vec<f64> = vec![];

        assert_have_same_size(x_size, y_size);
        if let Some(w) = w {
            weights = w;
            assert_have_same_size(x.len(), weights.len());
        }
        assert_have_size_greater_than_two(x.len());
        let size = x.len().to_owned();
        if weights.is_empty() {
            weights = populate_weights(size, 1.0);
        }
        Wls { x, y, w: weights }
    }

    pub fn fit_linear_regression(&self) -> Option<Point> {
        let mut total_of_weights: f64 = 0.0;
        let mut total_of_products_of_weights_and_x_squared: f64 = 0.0;
        let mut total_of_products_of_x_and_y_and_weights: f64 = 0.0;
        let mut total_of_products_of_xi_and_wi: f64 = 0.0;
        let mut total_of_products_of_y_and_weights: f64 = 0.0;

        let mut xi: f64;
        let mut yi: f64;
        let mut wi: f64;
        let mut product_of_xi_and_wi: f64;

        for i in 0..self.x.len() {
            xi = self.x[i];
            yi = self.y[i];
            wi = self.w[i];

            total_of_weights += wi;
            product_of_xi_and_wi = xi * wi;
            total_of_products_of_xi_and_wi += product_of_xi_and_wi;
            total_of_products_of_x_and_y_and_weights += product_of_xi_and_wi * yi;
            total_of_products_of_y_and_weights += yi * wi;
            total_of_products_of_weights_and_x_squared += product_of_xi_and_wi * xi;
        }

        let dividend = total_of_weights * total_of_products_of_x_and_y_and_weights
            - total_of_products_of_xi_and_wi * total_of_products_of_y_and_weights;
        let divisor = total_of_weights * total_of_products_of_weights_and_x_squared
            - total_of_products_of_xi_and_wi * total_of_products_of_xi_and_wi;
        if divisor == 0.0 {
            return None;
        }
        let slope = dividend / divisor;
        let intercept = (total_of_products_of_y_and_weights
            - slope * total_of_products_of_xi_and_wi)
            / total_of_weights;
        Some(Point::new(intercept, slope))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::wls::Wls;

    #[test]
    fn test_wls_model_with_stable_weights_ok() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
        let wls = Wls::new(x, y, None);
        let point = wls.fit_linear_regression().unwrap();
        assert!(0.000001 > 2.14285714 - point.get_intercept());
        assert_eq!(0.25, point.get_slope());
    }
}

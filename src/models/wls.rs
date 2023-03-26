use crate::models::point::Point;
use crate::asserts::asserts::{assert_have_same_size, assert_have_size_greater_than_two};

pub struct Wls {
    x_points: Vec<f64>,
    y_points: Vec<f64>,
    weights: Vec<f64>,
}

fn populate_weights(capacity: usize, value: f64) -> Vec<f64> {
    vec![value; capacity]
}


impl Wls {
    pub fn new(x_points: Vec<f64>, y_points: Vec<f64>, weights: Option<Vec<f64>>) -> Wls {
        let mut weights_normalized: Vec<f64> = vec![];

        assert_have_same_size(&x_points, &y_points);
        if let Some(weights) = weights {
            weights_normalized = weights;
            assert_have_same_size(&x_points, &weights_normalized);
        }
        assert_have_size_greater_than_two(x_points.len());
        let size = x_points.len().to_owned();
        if weights_normalized.is_empty() {
            weights_normalized = populate_weights(size, 1.0);
        }
        Wls {
            x_points,
            y_points,
            weights: weights_normalized,
        }
    }

    pub fn fit_linear_regression(&self) -> Option<Point> {
        let mut sum_of_weights: f64 = 0.0;
        let mut sum_of_products_of_weights_and_x_squared: f64 = 0.0;
        let mut sum_of_products_of_x_and_y_and_weights: f64 = 0.0;
        let mut sum_of_products_of_xi_and_wi: f64 = 0.0;
        let mut sum_of_products_of_y_and_weights: f64 = 0.0;

        let mut xi: f64;
        let mut yi: f64;
        let mut wi: f64;
        let mut product_of_xi_and_wi: f64;

        for i in 0..self.x_points.len() {
            xi = self.x_points[i];
            yi = self.y_points[i];
            wi = self.weights[i];

            sum_of_weights += wi;
            product_of_xi_and_wi = xi * wi;
            sum_of_products_of_xi_and_wi += product_of_xi_and_wi;
            sum_of_products_of_x_and_y_and_weights += product_of_xi_and_wi * yi;
            sum_of_products_of_y_and_weights += yi * wi;
            sum_of_products_of_weights_and_x_squared += product_of_xi_and_wi * xi;
        }

        let dividend = sum_of_weights * sum_of_products_of_x_and_y_and_weights
            - sum_of_products_of_xi_and_wi * sum_of_products_of_y_and_weights;
        let divisor = sum_of_weights * sum_of_products_of_weights_and_x_squared
            - sum_of_products_of_xi_and_wi * sum_of_products_of_xi_and_wi;
        if divisor == 0.0 {
            return None;
        }
        let slope = dividend / divisor;
        let intercept = (sum_of_products_of_y_and_weights - slope * sum_of_products_of_xi_and_wi)
            / sum_of_weights;
        Some(Point::new(intercept, slope))
    }
}

#[cfg(test)]
mod tests {
    use crate::asserts::asserts::{assert_almost_equal, assert_true};
    use crate::models::wls::Wls;

    #[test]
    fn test_wls_model_with_weights_ok() {
        let x_points = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let y_points = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
        let weights = vec![1.0, 2.0, 3.0, 1.0, 8.0, 1.0, 5.0];

        let wls = Wls::new(x_points, y_points, Some(weights));
        let point = wls.fit_linear_regression().unwrap();

        assert_almost_equal(2.14285714, point.get_intercept(), 1.0e-6);
        assert_almost_equal(0.150862, point.get_slope(), 1.0e-6);
    }

    #[test]
    fn test_wls_model_with_stable_weights_ok() {
        let x_points = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let y_points = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];

        let wls = Wls::new(x_points, y_points, None);
        let point = wls.fit_linear_regression().unwrap();

        assert_almost_equal(2.14285714, point.get_intercept(), 1.0e-6);
        assert_eq!(0.25, point.get_slope());
    }

    #[test]
    fn test_horizontal_line_ok() {
        let x_points = vec![0.0, 1.0];
        let y_points = vec![10.0, 10.0];

        let wls = Wls::new(x_points, y_points, None);
        let point = wls.fit_linear_regression().unwrap();

        assert_eq!(10.0, point.get_intercept());
        assert_eq!(0.0, point.get_slope());
    }

    #[test]
    fn test_vertical_line_ok() {
        let x_points = vec![1.0, 1.0];
        let y_points = vec![0.0, 1.0];

        let wls = Wls::new(x_points, y_points, None);
        assert_true(wls.fit_linear_regression().is_none());
    }

    #[test]
    fn test_run_uphill_ok() {
        let x_points = vec![0.0, 1.0];
        let y_points = vec![0.0, 1.0];

        let wls = Wls::new(x_points, y_points, None);
        let point = wls.fit_linear_regression().unwrap();

        assert_eq!(0.0, point.get_intercept());
        assert_eq!(1.0, point.get_slope());
    }

    #[test]
    fn test_run_downhill_ok() {
        let x_points = vec![1.0, 0.0];
        let y_points = vec![0.0, 1.0];

        let wls = Wls::new(x_points, y_points, None);
        let point = wls.fit_linear_regression().unwrap();

        assert_eq!(1.0, point.get_intercept());
        assert_eq!(-1.0, point.get_slope());
    }
}

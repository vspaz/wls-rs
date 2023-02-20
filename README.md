# wls-rust

weighted linear regression in pure Rust w/o any 3d party dependencies or frameworks.

### How-to

```rust
mod models;

use crate::models::wls::Wls;
use crate::models::wls::assert_almost_equal;

fn main() {
    let x_points = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let y_points = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
    let weights = vec![10.0, 1.0, 3.0, 8.0, 14.0, 21.0, 13.0];

    let wls = Wls::new(x_points, y_points, Some(weights));
    let point = wls.fit_linear_regression().unwrap();

    assert_almost_equal(1.410964913,point.get_intercept(), 0.00000001);
    assert_almost_equal(0.321271930, point.get_slope(), 0.00000001);
}

```

## Description

WLS is based on the OLS method and help solve problems of model inadequacy or violations of the basic regression
assumptions.

Estimating a linear regression with WLS is useful, but can appear to be daunting w/o special stats packages, such as
Python statsmodels or Pandas.

## References

- [Wikipedia: Weighted least squares](https://en.wikipedia.org/wiki/Weighted_least_squares)
- [Introduction to Linear Regression Analysis, 5th edition](https://tinyurl.com/y3clfnrs)
- [Least Squares Regression Analysis in Terms of Linear Algebra](https://tinyurl.com/y485qhlg) 

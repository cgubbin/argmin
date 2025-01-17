// Copyright 2018-2024 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Rosenbrock function
//!
//! In 2D, it is defined as
//!
//! `f(x_1, x_2) = (a - x_1)^2 + b * (x_2 - x_1^2)^2`
//!
//! where `x_i \in (-\infty, \infty)`. The parameters a and b usually are: `a = 1` and `b = 100`.
//!
//! The multidimensional Rosenbrock function is defined as:
//!
//! `f(x_1, x_2, ..., x_n) = \sum_{i=1}^{n-1} \left[ (a - x_i)^2 + b * (x_{i+1} - x_i^2)^2 \right]`
//!
//! The minimum is at `f(x_1, x_2, ..., x_n) = f(1, 1, ..., 1) = 0`.

use num::{Float, FromPrimitive};
use std::{iter::Sum, ops::AddAssign};

/// Multidimensional Rosenbrock test function
///
/// Defined as
///
/// `f(x_1, x_2, ..., x_n) = \sum_{i=1}^{n-1} \left[ (a - x_i)^2 + b * (x_{i+1} - x_i^2)^2 \right]`
///
/// where `x_i \in (-\infty, \infty)`. The parameters a and b usually are: `a = 1` and `b = 100`.
///
/// The global minimum is at `f(x_1, x_2, ..., x_n) = f(1, 1, ..., 1) = 0`.
pub fn rosenbrock<T>(param: &[T], a: T, b: T) -> T
where
    T: Float + FromPrimitive + Sum,
{
    param
        .iter()
        .zip(param.iter().skip(1))
        .map(|(&xi, &xi1)| (a - xi).powi(2) + b * (xi1 - xi.powi(2)).powi(2))
        .sum()
}

/// Derivative of the multidimensional Rosenbrock test function
pub fn rosenbrock_derivative<T>(param: &[T], a: T, b: T) -> Vec<T>
where
    T: Float + FromPrimitive + AddAssign,
{
    let n0 = T::from_f64(0.0).unwrap();
    let n2 = T::from_f64(2.0).unwrap();
    let n4 = T::from_f64(4.0).unwrap();

    let n = param.len();

    let mut result = vec![n0; n];

    for i in 0..(n - 1) {
        let xi = param[i];
        let xi1 = param[i + 1];

        let t1 = -n4 * b * xi * (xi1 - xi.powi(2));
        let t2 = n2 * b * (xi1 - xi.powi(2));

        result[i] += t1 + n2 * (xi - a);
        result[i + 1] += t2;
    }
    result
}

/// Hessian of the multidimensional Rosenbrock test function
pub fn rosenbrock_hessian<T>(param: &[T], a: T, b: T) -> Vec<Vec<T>>
where
    T: Float + FromPrimitive + AddAssign,
{
    let n0 = T::from_f64(0.0).unwrap();
    let n2 = T::from_f64(2.0).unwrap();
    let n4 = T::from_f64(4.0).unwrap();
    let n12 = T::from_f64(12.0).unwrap();

    let n = param.len();
    let mut hessian = vec![vec![n0; n]; n];

    for i in 0..n - 1 {
        let xi = param[i];
        let xi1 = param[i + 1];

        hessian[i][i] += n12 * b * xi.powi(2) - n4 * b * xi1 + n2 * a;
        hessian[i + 1][i + 1] = n2 * b;
        hessian[i][i + 1] = -n4 * b * xi;
        hessian[i + 1][i] = -n4 * b * xi;
    }
    hessian
}

/// Derivative of the multidimensional Rosenbrock test function
///
/// This is the const generics version, which requires the number of parameters to be known
/// at compile time.
pub fn rosenbrock_derivative_const<const N: usize, T>(param: &[T; N], a: T, b: T) -> [T; N]
where
    T: Float + FromPrimitive + AddAssign,
{
    let n0 = T::from_f64(0.0).unwrap();
    let n2 = T::from_f64(2.0).unwrap();
    let n4 = T::from_f64(4.0).unwrap();

    let mut result = [n0; N];

    for i in 0..(N - 1) {
        let xi = param[i];
        let xi1 = param[i + 1];

        let t1 = -n4 * b * xi * (xi1 - xi.powi(2));
        let t2 = n2 * b * (xi1 - xi.powi(2));

        result[i] += t1 + n2 * (xi - a);
        result[i + 1] += t2;
    }
    result
}

/// Hessian of the multidimensional Rosenbrock test function
pub fn rosenbrock_hessian_const<const N: usize, T>(x: &[T; N], a: T, b: T) -> [[T; N]; N]
where
    T: Float + FromPrimitive + AddAssign,
{
    let n0 = T::from_f64(0.0).unwrap();
    let n2 = T::from_f64(2.0).unwrap();
    let n4 = T::from_f64(4.0).unwrap();
    let n12 = T::from_f64(12.0).unwrap();

    let mut hessian = [[n0; N]; N];

    for i in 0..(N - 1) {
        let xi = x[i];
        let xi1 = x[i + 1];

        hessian[i][i] += n12 * b * xi.powi(2) - n4 * b * xi1 + n2 * a;
        hessian[i + 1][i + 1] = n2 * b;
        hessian[i][i + 1] = -n4 * b * xi;
        hessian[i + 1][i] = -n4 * b * xi;
    }
    hessian
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use finitediff::FiniteDiff;
    use proptest::prelude::*;

    #[test]
    fn test_rosenbrock_optimum() {
        assert_relative_eq!(
            rosenbrock(&[1.0_f32, 1.0_f32], 1.0, 100.0),
            0.0,
            epsilon = std::f32::EPSILON
        );
        assert_relative_eq!(
            rosenbrock(&[1.0, 1.0], 1.0, 100.0),
            0.0,
            epsilon = std::f64::EPSILON
        );
        assert_relative_eq!(
            rosenbrock(&[1.0, 1.0, 1.0], 1.0, 100.0),
            0.0,
            epsilon = std::f64::EPSILON
        );
    }

    #[test]
    fn test_rosenbrock_derivative_optimum() {
        let derivative =
            rosenbrock_derivative(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], 1.0, 100.0);
        for elem in derivative {
            assert_relative_eq!(elem, 0.0, epsilon = std::f64::EPSILON);
        }
    }

    #[test]
    fn test_rosenbrock_derivative_const_optimum() {
        let derivative =
            rosenbrock_derivative_const(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], 1.0, 100.0);
        for elem in derivative {
            assert_relative_eq!(elem, 0.0, epsilon = std::f64::EPSILON);
        }
    }

    #[test]
    fn test_rosenbrock_hessian() {
        // Same testcase as in scipy
        let hessian = rosenbrock_hessian(&[0.0, 0.1, 0.2, 0.3], 1.0, 100.0);
        let res = vec![
            vec![-38.0, 0.0, 0.0, 0.0],
            vec![0.0, 134.0, -40.0, 0.0],
            vec![0.0, -40.0, 130.0, -80.0],
            vec![0.0, 0.0, -80.0, 200.0],
        ];
        let n = hessian.len();
        for i in 0..n {
            assert_eq!(hessian[i].len(), n);
            for j in 0..n {
                assert_relative_eq!(hessian[i][j], res[i][j], epsilon = std::f64::EPSILON);
            }
        }
    }

    #[test]
    fn test_rosenbrock_hessian_const() {
        // Same testcase as in scipy
        let hessian = rosenbrock_hessian_const(&[0.0, 0.1, 0.2, 0.3], 1.0, 100.0);
        let res = vec![
            vec![-38.0, 0.0, 0.0, 0.0],
            vec![0.0, 134.0, -40.0, 0.0],
            vec![0.0, -40.0, 130.0, -80.0],
            vec![0.0, 0.0, -80.0, 200.0],
        ];
        let n = hessian.len();
        for i in 0..n {
            assert_eq!(hessian[i].len(), n);
            for j in 0..n {
                assert_relative_eq!(hessian[i][j], res[i][j], epsilon = std::f64::EPSILON);
            }
        }
    }

    proptest! {
        #[test]
        fn test_rosenbrock_derivative_finitediff(a in -1.0..1.0,
                                                 b in -1.0..1.0,
                                                 c in -1.0..1.0,
                                                 d in -1.0..1.0,
                                                 e in -1.0..1.0,
                                                 f in -1.0..1.0,
                                                 g in -1.0..1.0,
                                                 h in -1.0..1.0) {
            let param = [a, b, c, d, e, f, g, h];
            let derivative = rosenbrock_derivative(&param, 1.0, 100.0);
            let derivative_fd = Vec::from(param).central_diff(&|x| rosenbrock(&x, 1.0, 100.0));
            for i in 0..derivative.len() {
                assert_relative_eq!(derivative[i], derivative_fd[i], epsilon = 1e-4);
            }
        }
    }

    proptest! {
        #[test]
        fn test_rosenbrock_derivative_const_finitediff(a in -1.0..1.0,
                                                       b in -1.0..1.0,
                                                       c in -1.0..1.0,
                                                       d in -1.0..1.0,
                                                       e in -1.0..1.0,
                                                       f in -1.0..1.0,
                                                       g in -1.0..1.0,
                                                       h in -1.0..1.0) {
            let param = [a, b, c, d, e, f, g, h];
            let derivative = rosenbrock_derivative_const(&param, 1.0, 100.0);
            let derivative_fd = Vec::from(param).central_diff(&|x| rosenbrock(&x, 1.0, 100.0));
            for i in 0..derivative.len() {
                assert_relative_eq!(derivative[i], derivative_fd[i], epsilon = 1e-4);
            }
        }
    }

    proptest! {
        #[test]
        fn test_rosenbrock_hessian_finitediff(a in -1.0..1.0,
                                              b in -1.0..1.0,
                                              c in -1.0..1.0,
                                              d in -1.0..1.0,
                                              e in -1.0..1.0,
                                              f in -1.0..1.0,
                                              g in -1.0..1.0,
                                              h in -1.0..1.0) {
            let param = [a, b, c, d, e, f, g, h];
            let hessian = rosenbrock_hessian(&param, 1.0, 100.0);
            let hessian_fd =
                Vec::from(param).forward_hessian(&|x| rosenbrock_derivative(&x, 1.0, 100.0));
            let n = hessian.len();
            for i in 0..n {
                assert_eq!(hessian[i].len(), n);
                for j in 0..n {
                    assert_relative_eq!(hessian[i][j], hessian_fd[i][j], epsilon = 1e-4);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_rosenbrock_hessian_const_finitediff(a in -1.0..1.0,
                                                    b in -1.0..1.0,
                                                    c in -1.0..1.0,
                                                    d in -1.0..1.0,
                                                    e in -1.0..1.0,
                                                    f in -1.0..1.0,
                                                    g in -1.0..1.0,
                                                    h in -1.0..1.0) {
            let param = [a, b, c, d, e, f, g, h];
            let hessian = rosenbrock_hessian_const(&param, 1.0, 100.0);
            let hessian_fd =
                Vec::from(param).forward_hessian(&|x| rosenbrock_derivative(&x, 1.0, 100.0));
            let n = hessian.len();
            for i in 0..n {
                assert_eq!(hessian[i].len(), n);
                for j in 0..n {
                    assert_relative_eq!(hessian[i][j], hessian_fd[i][j], epsilon = 1e-4);
                }
            }
        }
    }
}

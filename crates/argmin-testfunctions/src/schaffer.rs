// Copyright 2018-2024 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Schaffer test function No. 2
//!
//! Defined as
//!
//! `f(x_1, x_2) = 0.5 + (sin^2(x_1^2 - x_2^2) - 0.5) / (1 + 0.001*(x_1^2 + x_2^2))^2`
//!
//! where `x_i \in [-100, 100]`.
//!
//! The global minimum is at `f(x_1, x_2) = f(0, 0) = 0`.
//!
//! # Schaffer test function No. 4
//!
//! Defined as
//!
//! `f(x_1, x_2) = 0.5 + (cos(sin(abs(x_1^2 - x_2^2)))^2 - 0.5) / (1 + 0.001*(x_1^2 + x_2^2))^2`
//!
//! where `x_i \in [-100, 100]`.
//!
//! The global minimum is at `f(x_1, x_2) = f(0, 1.25313) = 0.291992`.

use num::{Float, FromPrimitive};

/// Schaffer test function No. 2
///
/// Defined as
///
/// `f(x_1, x_2) = 0.5 + (sin^2(x_1^2 - x_2^2) - 0.5) / (1 + 0.001*(x_1^2 + x_2^2))^2`
///
/// where `x_i \in [-100, 100]`.
///
/// The global minimum is at `f(x_1, x_2) = f(0, 0) = 0`.
pub fn schaffer_n2<T>(param: &[T; 2]) -> T
where
    T: Float + FromPrimitive,
{
    let [x1, x2] = *param;
    let n05 = T::from_f64(0.5).unwrap();
    let n1 = T::from_f64(1.0).unwrap();
    let n0001 = T::from_f64(0.0001).unwrap();
    n05 + ((x1.powi(2) - x2.powi(2)).sin().powi(2) - n05)
        / (n1 + n0001 * (x1.powi(2) + x2.powi(2))).powi(2)
}

/// Schaffer test function No. 4
///
/// Defined as
///
/// `f(x_1, x_2) = 0.5 + (cos(sin(abs(x_1^2 - x_2^2)))^2 - 0.5) / (1 + 0.001*(x_1^2 + x_2^2))^2`
///
/// where `x_i \in [-100, 100]`.
///
/// The global minimum is at `f(x_1, x_2) = f(0, 1.25313) = 0.291992`.
pub fn schaffer_n4<T>(param: &[T; 2]) -> T
where
    T: Float + FromPrimitive,
{
    let [x1, x2] = *param;
    let n05 = T::from_f64(0.5).unwrap();
    let n1 = T::from_f64(1.0).unwrap();
    let n0001 = T::from_f64(0.0001).unwrap();
    n05 + ((x1.powi(2) - x2.powi(2)).abs().sin().cos().powi(2) - n05)
        / (n1 + n0001 * (x1.powi(2) + x2.powi(2))).powi(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::{f32, f64};

    #[test]
    fn test_schaffer_n2_optimum() {
        assert_relative_eq!(schaffer_n2(&[0_f32, 0_f32]), 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(schaffer_n2(&[0_f64, 0_f64]), 0.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_schaffer_n4_optimum() {
        assert_relative_eq!(
            schaffer_n4(&[0_f32, 1.25313_f32]),
            0.291992,
            epsilon = f32::EPSILON
        );
    }
}

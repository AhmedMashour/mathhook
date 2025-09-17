//! Adaptive step size methods for numerical ODE solving
//!
//! Implements Runge-Kutta-Fehlberg method (RKF45) with automatic step size adjustment
//! based on error tolerance. Uses embedded 4th and 5th order Runge-Kutta formulas
//! to estimate local truncation error.

use crate::calculus::ode::first_order::ODEError;

/// Configuration for adaptive step size solver
#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
    /// Error tolerance for step size adjustment
    pub tolerance: f64,
    /// Minimum allowed step size
    pub min_step: f64,
    /// Maximum allowed step size
    pub max_step: f64,
    /// Safety factor for step size adjustment
    pub safety_factor: f64,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            min_step: 1e-10,
            max_step: 1.0,
            safety_factor: 0.9,
        }
    }
}

/// Solves a first-order ODE using Runge-Kutta-Fehlberg method with adaptive step size
///
/// # Arguments
///
/// * `f` - The derivative function f(x, y) where dy/dx = f(x, y)
/// * `x0` - Initial x value
/// * `y0` - Initial y value
/// * `x_end` - Final x value
/// * `initial_step` - Initial step size
/// * `config` - Configuration for adaptive stepping
///
/// # Returns
///
/// Vector of (x, y) solution points with adaptively chosen steps
///
/// # Examples
///
/// ```
/// use mathhook_core::calculus::ode::numerical::adaptive::{rkf45_method, AdaptiveConfig};
///
/// let solution = rkf45_method(
///     |_x, y| y,
///     0.0,
///     1.0,
///     1.0,
///     0.1,
///     &AdaptiveConfig::default()
/// );
///
/// assert!(solution.len() > 0);
/// let (_, y_final) = solution.last().unwrap();
/// let expected = 1.0_f64.exp();
/// assert!((y_final - expected).abs() < 1e-5);
/// ```
pub fn rkf45_method<F>(
    f: F,
    x0: f64,
    y0: f64,
    x_end: f64,
    initial_step: f64,
    config: &AdaptiveConfig,
) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    if initial_step <= 0.0 {
        return vec![(x0, y0)];
    }

    let mut solution = Vec::new();
    let mut x = x0;
    let mut y = y0;
    let direction = if x_end > x0 { 1.0 } else { -1.0 };
    let mut h = initial_step.min(config.max_step) * direction;

    solution.push((x, y));
    while (direction > 0.0 && x < x_end) || (direction < 0.0 && x > x_end) {
        if h.abs() < config.min_step {
            h = config.min_step * direction;
        }

        if (direction > 0.0 && x + h > x_end) || (direction < 0.0 && x + h < x_end) {
            h = x_end - x;
        }

        let (y_new, error, h_new) = rkf45_step(&f, x, y, h, config);

        if error <= config.tolerance || h.abs() <= config.min_step {
            x += h;
            y = y_new;
            solution.push((x, y));
        }

        h = h_new;
    }

    solution
}

fn rkf45_step<F>(f: &F, x: f64, y: f64, h: f64, config: &AdaptiveConfig) -> (f64, f64, f64)
where
    F: Fn(f64, f64) -> f64,
{
    let k1 = f(x, y);
    let k2 = f(x + h / 4.0, y + h * k1 / 4.0);
    let k3 = f(x + 3.0 * h / 8.0, y + h * (3.0 * k1 + 9.0 * k2) / 32.0);
    let k4 = f(
        x + 12.0 * h / 13.0,
        y + h * (1932.0 * k1 - 7200.0 * k2 + 7296.0 * k3) / 2197.0,
    );
    let k5 = f(
        x + h,
        y + h * (439.0 * k1 / 216.0 - 8.0 * k2 + 3680.0 * k3 / 513.0 - 845.0 * k4 / 4104.0),
    );
    let k6 = f(
        x + h / 2.0,
        y + h
            * (-8.0 * k1 / 27.0 + 2.0 * k2 - 3544.0 * k3 / 2565.0 + 1859.0 * k4 / 4104.0
                - 11.0 * k5 / 40.0),
    );

    let y4 = y + h * (25.0 * k1 / 216.0 + 1408.0 * k3 / 2565.0 + 2197.0 * k4 / 4104.0 - k5 / 5.0);

    let y5 = y + h
        * (16.0 * k1 / 135.0 + 6656.0 * k3 / 12825.0 + 28561.0 * k4 / 56430.0 - 9.0 * k5 / 50.0
            + 2.0 * k6 / 55.0);

    let error = (y5 - y4).abs();

    let h_new = if error > 0.0 {
        let scale = config.safety_factor * (config.tolerance / error).powf(0.2);
        (h * scale.clamp(0.1, 4.0))
            .abs()
            .max(config.min_step)
            .min(config.max_step)
            * h.signum()
    } else {
        (h.abs() * 2.0).min(config.max_step) * h.signum()
    };

    (y5, error, h_new)
}

/// Solves a first-order ODE using adaptive RKF45 method with Result type
///
/// # Arguments
///
/// * `f` - The derivative function f(x, y)
/// * `x0` - Initial x value
/// * `y0` - Initial y value
/// * `x_end` - Final x value
/// * `initial_step` - Initial step size
/// * `config` - Adaptive configuration
///
/// # Returns
///
/// Result containing vector of (x, y) solution points
pub fn solve_adaptive<F>(
    f: F,
    x0: f64,
    y0: f64,
    x_end: f64,
    initial_step: f64,
    config: AdaptiveConfig,
) -> Result<Vec<(f64, f64)>, ODEError>
where
    F: Fn(f64, f64) -> f64,
{
    if initial_step <= 0.0 {
        return Err(ODEError::InvalidInput {
            message: "Initial step size must be positive".to_owned(),
        });
    }

    if config.tolerance <= 0.0 {
        return Err(ODEError::InvalidInput {
            message: "Tolerance must be positive".to_owned(),
        });
    }

    if !x0.is_finite() || !y0.is_finite() || !x_end.is_finite() {
        return Err(ODEError::InvalidInput {
            message: "Initial values and endpoints must be finite".to_owned(),
        });
    }

    Ok(rkf45_method(f, x0, y0, x_end, initial_step, &config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_constant_derivative() {
        let solution = rkf45_method(|_x, _y| 2.0, 0.0, 0.0, 1.0, 0.1, &AdaptiveConfig::default());

        assert!(!solution.is_empty());
        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);
        assert!((y_final - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_adaptive_linear_ode() {
        let solution = rkf45_method(|x, _y| x, 0.0, 0.0, 1.0, 0.1, &AdaptiveConfig::default());

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);
        assert!((y_final - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_adaptive_exponential_growth() {
        let solution = rkf45_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1, &AdaptiveConfig::default());

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);

        let expected = 1.0_f64.exp();
        let relative_error = (y_final - expected).abs() / expected;
        assert!(relative_error < 1e-5);
    }

    #[test]
    fn test_adaptive_stiff_problem() {
        let config = AdaptiveConfig {
            tolerance: 1e-8,
            min_step: 1e-12,
            max_step: 0.1,
            safety_factor: 0.9,
        };

        let solution = rkf45_method(|_x, y| -100.0 * y, 0.0, 1.0, 1.0, 0.1, &config);

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);

        let expected = (-100.0_f64).exp();
        assert!((y_final - expected).abs() < 1e-6);
    }

    #[test]
    fn test_adaptive_backward_integration() {
        let solution = rkf45_method(|x, _y| x, 1.0, 0.5, 0.0, 0.1, &AdaptiveConfig::default());

        assert!(solution.len() > 1);
        let (x_first, y_first) = solution[0];
        let (x_final, y_final) = solution.last().unwrap();

        assert_eq!((x_first, y_first), (1.0, 0.5));
        assert!((x_final - 0.0).abs() < 1e-10);
        assert!((y_final - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_adaptive_step_adjustment() {
        let solution = rkf45_method(|_x, y| y, 0.0, 1.0, 1.0, 0.5, &AdaptiveConfig::default());

        let steps: Vec<f64> = solution
            .windows(2)
            .map(|w| (w[1].0 - w[0].0).abs())
            .collect();

        let min_step = steps.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_step = steps.iter().cloned().fold(0.0, f64::max);

        assert!(max_step > min_step);
    }

    #[test]
    fn test_solve_adaptive_invalid_input() {
        let result = solve_adaptive(|x, _y| x, 0.0, 0.0, 1.0, -0.1, AdaptiveConfig::default());
        assert!(result.is_err());

        let config = AdaptiveConfig {
            tolerance: -1.0,
            ..Default::default()
        };
        let result = solve_adaptive(|x, _y| x, 0.0, 0.0, 1.0, 0.1, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_adaptive_better_accuracy() {
        use crate::calculus::ode::numerical::runge_kutta::rk4_method;

        let adaptive_sol = rkf45_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1, &AdaptiveConfig::default());
        let rk4_sol = rk4_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1);

        let expected = 1.0_f64.exp();
        let (_, y_adaptive) = adaptive_sol.last().unwrap();
        let (_, y_rk4) = rk4_sol.last().unwrap();

        let error_adaptive = (y_adaptive - expected).abs();
        let error_rk4 = (y_rk4 - expected).abs();

        assert!(error_adaptive <= error_rk4 * 1.1);
    }
}

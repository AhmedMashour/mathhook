//! Euler's method for numerical ODE solving
//!
//! Implements the forward Euler method for first-order ODEs: y' = f(x, y)
//! Formula: y_{n+1} = y_n + h * f(x_n, y_n)

use crate::ode::first_order::ODEError;

/// Solves a first-order ODE using forward Euler method
///
/// # Arguments
///
/// * `f` - The derivative function f(x, y) where dy/dx = f(x, y)
/// * `x0` - Initial x value
/// * `y0` - Initial y value
/// * `x_end` - Final x value
/// * `step` - Step size h
///
/// # Returns
///
/// Vector of (x, y) solution points
///
/// # Examples
///
/// ```
/// use mathhook_core::ode::numerical::euler::euler_method;
///
/// let solution = euler_method(
///     |x, _y| x,
///     0.0,
///     0.0,
///     1.0,
///     0.1
/// );
///
/// assert!(solution.len() > 0);
/// assert_eq!(solution[0], (0.0, 0.0));
/// ```
pub fn euler_method<F>(f: F, x0: f64, y0: f64, x_end: f64, step: f64) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    if step <= 0.0 {
        return vec![(x0, y0)];
    }

    let num_steps = ((x_end - x0) / step).abs().ceil() as usize;
    let mut solution = Vec::with_capacity(num_steps + 1);

    let mut x = x0;
    let mut y = y0;
    solution.push((x, y));

    let h = if x_end > x0 { step } else { -step };

    for _ in 0..num_steps {
        if (x_end > x0 && x >= x_end) || (x_end < x0 && x <= x_end) {
            break;
        }

        let slope = f(x, y);
        y = y + h * slope;
        x = x + h;

        solution.push((x, y));
    }

    solution
}

/// Solves a first-order ODE using Euler method with Result type
///
/// # Arguments
///
/// * `f` - The derivative function f(x, y)
/// * `x0` - Initial x value
/// * `y0` - Initial y value
/// * `x_end` - Final x value
/// * `step` - Step size
///
/// # Returns
///
/// Result containing vector of (x, y) solution points
pub fn solve_euler<F>(
    f: F,
    x0: f64,
    y0: f64,
    x_end: f64,
    step: f64,
) -> Result<Vec<(f64, f64)>, ODEError>
where
    F: Fn(f64, f64) -> f64,
{
    if step <= 0.0 {
        return Err(ODEError::InvalidInput {
            message: "Step size must be positive".to_string(),
        });
    }

    if !x0.is_finite() || !y0.is_finite() || !x_end.is_finite() {
        return Err(ODEError::InvalidInput {
            message: "Initial values and endpoints must be finite".to_string(),
        });
    }

    Ok(euler_method(f, x0, y0, x_end, step))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_constant_derivative() {
        let solution = euler_method(|_x, _y| 2.0, 0.0, 0.0, 1.0, 0.1);

        assert_eq!(solution.len(), 11);
        assert_eq!(solution[0], (0.0, 0.0));

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);
        assert!((y_final - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_euler_linear_ode() {
        let solution = euler_method(|x, _y| x, 0.0, 0.0, 1.0, 0.1);

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);
        assert!((y_final - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_euler_exponential_growth() {
        let solution = euler_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1);

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);

        let expected = 1.0_f64.exp();
        let relative_error = (y_final - expected).abs() / expected;
        assert!(relative_error < 0.1);
    }

    #[test]
    fn test_euler_backward_integration() {
        let solution = euler_method(|x, _y| x, 1.0, 0.5, 0.0, 0.1);

        assert!(solution.len() > 1);
        let (x_first, y_first) = solution[0];
        let (x_final, y_final) = solution.last().unwrap();

        assert_eq!((x_first, y_first), (1.0, 0.5));
        assert!((x_final - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_euler_zero_step_size() {
        let solution = euler_method(|x, _y| x, 0.0, 0.0, 1.0, 0.0);

        assert_eq!(solution.len(), 1);
        assert_eq!(solution[0], (0.0, 0.0));
    }

    #[test]
    fn test_solve_euler_invalid_input() {
        let result = solve_euler(|x, _y| x, 0.0, 0.0, 1.0, -0.1);
        assert!(result.is_err());

        let result = solve_euler(|x, _y| x, f64::NAN, 0.0, 1.0, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_euler_variable_step() {
        let solution1 = euler_method(|x, _y| x, 0.0, 0.0, 1.0, 0.1);
        let solution2 = euler_method(|x, _y| x, 0.0, 0.0, 1.0, 0.05);

        let (_, y1) = solution1.last().unwrap();
        let (_, y2) = solution2.last().unwrap();

        assert!(solution2.len() > solution1.len());
        assert!((y2 - 0.5).abs() < (y1 - 0.5).abs());
    }
}

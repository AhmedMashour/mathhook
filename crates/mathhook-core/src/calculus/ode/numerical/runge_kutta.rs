//! Runge-Kutta 4th order method for numerical ODE solving
//!
//! Implements the classic RK4 method for first-order ODEs: y' = f(x, y)
//! Formula:
//!   k1 = f(x_n, y_n)
//!   k2 = f(x_n + h/2, y_n + h*k1/2)
//!   k3 = f(x_n + h/2, y_n + h*k2/2)
//!   k4 = f(x_n + h, y_n + h*k3)
//!   y_{n+1} = y_n + h/6 * (k1 + 2*k2 + 2*k3 + k4)

use crate::calculus::ode::first_order::ODEError;

/// Solves a first-order ODE using Runge-Kutta 4th order method
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
/// use mathhook_core::calculus::ode::numerical::runge_kutta::rk4_method;
///
/// let solution = rk4_method(
///     |x, _y| x,
///     0.0,
///     0.0,
///     1.0,
///     0.1
/// );
///
/// assert!(solution.len() > 0);
/// assert_eq!(solution[0], (0.0, 0.0));
/// let (_, y_final) = solution.last().unwrap();
/// assert!((y_final - 0.5).abs() < 0.001);
/// ```
pub fn rk4_method<F>(f: F, x0: f64, y0: f64, x_end: f64, step: f64) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    if step <= 0.0 {
        return vec![(x0, y0)];
    }

    let mut solution = Vec::new();

    let mut x = x0;
    let mut y = y0;
    solution.push((x, y));

    let direction = if x_end > x0 { 1.0 } else { -1.0 };
    let h = direction * step;

    loop {
        // Check if we would overshoot the endpoint
        if direction > 0.0 && x + h > x_end {
            // Take a final partial step to exactly reach x_end
            let final_h = x_end - x;
            if final_h > 1e-10 {
                let k1 = f(x, y);
                let k2 = f(x + final_h / 2.0, y + final_h * k1 / 2.0);
                let k3 = f(x + final_h / 2.0, y + final_h * k2 / 2.0);
                let k4 = f(x + final_h, y + final_h * k3);
                y += final_h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
                x = x_end;
                solution.push((x, y));
            }
            break;
        } else if direction < 0.0 && x + h < x_end {
            // Take a final partial step to exactly reach x_end (backward)
            let final_h = x_end - x;
            if final_h.abs() > 1e-10 {
                let k1 = f(x, y);
                let k2 = f(x + final_h / 2.0, y + final_h * k1 / 2.0);
                let k3 = f(x + final_h / 2.0, y + final_h * k2 / 2.0);
                let k4 = f(x + final_h, y + final_h * k3);
                y += final_h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
                x = x_end;
                solution.push((x, y));
            }
            break;
        }

        // Normal RK4 step
        let k1 = f(x, y);
        let k2 = f(x + h / 2.0, y + h * k1 / 2.0);
        let k3 = f(x + h / 2.0, y + h * k2 / 2.0);
        let k4 = f(x + h, y + h * k3);

        y += h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        x += h;

        solution.push((x, y));

        // Check if we've reached the endpoint (within floating point tolerance)
        if (direction > 0.0 && x >= x_end - 1e-10) || (direction < 0.0 && x <= x_end + 1e-10) {
            break;
        }
    }

    solution
}

/// Solves a first-order ODE using RK4 method with Result type
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
pub fn solve_rk4<F>(
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
            message: "Step size must be positive".to_owned(),
        });
    }

    if !x0.is_finite() || !y0.is_finite() || !x_end.is_finite() {
        return Err(ODEError::InvalidInput {
            message: "Initial values and endpoints must be finite".to_owned(),
        });
    }

    Ok(rk4_method(f, x0, y0, x_end, step))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rk4_constant_derivative() {
        let solution = rk4_method(|_x, _y| 2.0, 0.0, 0.0, 1.0, 0.1);

        assert!(solution.len() >= 11);
        assert_eq!(solution[0], (0.0, 0.0));

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);
        assert!((y_final - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_rk4_linear_ode() {
        let solution = rk4_method(|x, _y| x, 0.0, 0.0, 1.0, 0.1);

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);
        assert!((y_final - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_rk4_exponential_growth() {
        let solution = rk4_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1);

        let (x_final, y_final) = solution.last().unwrap();
        assert!((x_final - 1.0).abs() < 1e-10);

        let expected = 1.0_f64.exp();
        let relative_error = (y_final - expected).abs() / expected;
        assert!(relative_error < 1e-4);
    }

    #[test]
    fn test_rk4_trigonometric() {
        // dy/dx = cos(x), y(0) = 0 => y = sin(x)
        // At x = π, y = sin(π) = 0
        let solution = rk4_method(|x, _y| x.cos(), 0.0, 0.0, std::f64::consts::PI, 0.1);

        let (x_final, y_final) = solution.last().unwrap();

        // Verify we actually reach π (not overshoot)
        assert!(
            (x_final - std::f64::consts::PI).abs() < 1e-10,
            "Expected x_final ≈ {}, got {}",
            std::f64::consts::PI,
            x_final
        );

        // Verify y(π) ≈ sin(π) = 0
        let expected = std::f64::consts::PI.sin();
        assert!(
            (y_final - expected).abs() < 1e-4,
            "Expected y_final ≈ {}, got {}",
            expected,
            y_final
        );
    }

    #[test]
    fn test_rk4_backward_integration() {
        let solution = rk4_method(|x, _y| x, 1.0, 0.5, 0.0, 0.1);

        assert!(solution.len() > 1);
        let (x_first, y_first) = solution[0];
        let (x_final, y_final) = solution.last().unwrap();

        assert_eq!((x_first, y_first), (1.0, 0.5));
        assert!((x_final - 0.0).abs() < 1e-10);
        assert!((y_final - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_rk4_zero_step_size() {
        let solution = rk4_method(|x, _y| x, 0.0, 0.0, 1.0, 0.0);

        assert_eq!(solution.len(), 1);
        assert_eq!(solution[0], (0.0, 0.0));
    }

    #[test]
    fn test_solve_rk4_invalid_input() {
        let result = solve_rk4(|x, _y| x, 0.0, 0.0, 1.0, -0.1);
        assert!(result.is_err());

        let result = solve_rk4(|x, _y| x, f64::NAN, 0.0, 1.0, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_rk4_variable_step() {
        // dy/dx = y, y(0) = 1 => y = e^x
        // This is a more sensitive test case than dy/dx = x
        // because RK4's truncation error is more visible
        let solution1 = rk4_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1);
        let solution2 = rk4_method(|_x, y| y, 0.0, 1.0, 1.0, 0.05);

        let (_, y1) = solution1.last().unwrap();
        let (_, y2) = solution2.last().unwrap();

        // More steps with smaller step size
        assert!(solution2.len() > solution1.len());

        // Expected value: e^1
        let expected = 1.0_f64.exp();

        // Smaller step size should give more accurate result
        let error1 = (y1 - expected).abs();
        let error2 = (y2 - expected).abs();

        assert!(
            error2 < error1,
            "Smaller step should be more accurate: error(h=0.05)={} should be < error(h=0.1)={}",
            error2,
            error1
        );
    }

    #[test]
    fn test_rk4_better_than_euler() {
        use crate::calculus::ode::numerical::euler::euler_method;

        let rk4_sol = rk4_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1);
        let euler_sol = euler_method(|_x, y| y, 0.0, 1.0, 1.0, 0.1);

        let expected = 1.0_f64.exp();
        let (_, y_rk4) = rk4_sol.last().unwrap();
        let (_, y_euler) = euler_sol.last().unwrap();

        let error_rk4 = (y_rk4 - expected).abs();
        let error_euler = (y_euler - expected).abs();

        assert!(error_rk4 < error_euler);
    }
}

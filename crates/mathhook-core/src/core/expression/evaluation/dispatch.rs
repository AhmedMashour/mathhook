//! Function dispatch for expression evaluation
//!
//! Contains the performance-critical function dispatch table that routes
//! function calls to their implementations.
//!
//! This module provides:
//! - `evaluate_function_dispatch()` - O(1) dispatch via compiler jump table
//! - `evaluate_orthogonal_polynomial()` - helper for orthogonal polynomial evaluation

use super::super::Expression;
use crate::core::{MathConstant, Number};
use crate::simplify::Simplify;
use num_traits::ToPrimitive;
use std::collections::HashMap;

/// Performance-critical function dispatch
///
/// Routes function calls to their implementations using a compiler-optimized
/// jump table for O(1) dispatch overhead (<10ns).
///
/// # Arguments
///
/// * `name` - Function name (e.g., "sin", "cos", "gamma")
/// * `args` - Function arguments
///
/// # Returns
///
/// Some(Expression) if the function can be evaluated, None otherwise
///
/// # Performance
///
/// This function is marked `#[inline]` and uses a match statement that the
/// compiler optimizes to a jump table. Dispatch overhead is <10ns.
///
/// # Supported Functions
///
/// ## Elementary Functions
/// - Trigonometric: sin, cos, tan, arcsin, arccos, arctan
/// - Hyperbolic: sinh, cosh, tanh
/// - Exponential/Logarithmic: exp, ln, log10
/// - Roots: sqrt
/// - Rounding: sign, floor, ceil, round
/// - Absolute value: abs
///
/// ## Special Functions
/// - Gamma family: gamma, digamma, polygamma, beta, factorial
/// - Bessel functions: bessel_j, bessel_y
/// - Error functions: erf, erfc
/// - Zeta function: zeta
///
/// ## Number Theory
/// - gcd, lcm, mod, isprime
///
/// ## Polynomial Operations
/// - degree, roots, expand, factor
///
/// ## Orthogonal Polynomials
/// - Legendre: legendrep, legendre_p
/// - Chebyshev first kind: chebyshevt, chebyshev_t
/// - Chebyshev second kind: chebyshevu, chebyshev_u
/// - Hermite: hermiteh, hermite
/// - Laguerre: laguerrel, laguerre
#[inline]
pub fn evaluate_function_dispatch(name: &str, args: &[Expression]) -> Option<Expression> {
    if args.is_empty() {
        return None;
    }

    match name {
        "sin" => Some(crate::functions::elementary::trigonometric::sin(&args[0])),
        "cos" => Some(crate::functions::elementary::trigonometric::cos(&args[0])),
        "tan" => Some(crate::functions::elementary::trigonometric::tan(&args[0])),
        "arcsin" | "asin" => Some(crate::functions::elementary::trigonometric::arcsin(
            &args[0],
        )),
        "arccos" | "acos" => Some(crate::functions::elementary::trigonometric::arccos(
            &args[0],
        )),
        "arctan" | "atan" => Some(crate::functions::elementary::trigonometric::arctan(
            &args[0],
        )),
        "abs" => Some(crate::functions::elementary::abs_eval::abs(&args[0])),
        "sqrt" => Some(crate::functions::elementary::sqrt_eval::sqrt(&args[0])),
        "exp" => Some(crate::functions::elementary::exp_eval::exp(&args[0])),
        "ln" => Some(crate::functions::elementary::log_eval::ln(&args[0])),
        "log10" | "log" => Some(crate::functions::elementary::log_eval::log10(&args[0])),
        "sign" => Some(crate::functions::elementary::rounding::sign(&args[0])),
        "floor" => Some(crate::functions::elementary::rounding::floor(&args[0])),
        "ceil" => Some(crate::functions::elementary::rounding::ceil(&args[0])),
        "round" => Some(crate::functions::elementary::rounding::round(&args[0])),
        "sinh" => Some(crate::functions::elementary::hyperbolic_eval::sinh(
            &args[0],
        )),
        "cosh" => Some(crate::functions::elementary::hyperbolic_eval::cosh(
            &args[0],
        )),
        "tanh" => Some(crate::functions::elementary::hyperbolic_eval::tanh(
            &args[0],
        )),
        "gamma" => Some(crate::functions::special::gamma::gamma(&args[0])),
        "digamma" => Some(crate::functions::special::digamma(&args[0])),
        "polygamma" if args.len() >= 2 => {
            if let Expression::Number(Number::Integer(n)) = &args[0] {
                return Some(crate::functions::special::polygamma(*n as i32, &args[1]));
            }
            None
        }
        "bessel_j" | "besselj" if args.len() >= 2 => {
            if let Expression::Number(Number::Integer(n)) = &args[0] {
                let order = (*n) as i32;
                return Some(crate::functions::special::bessel_j(order, &args[1]));
            }
            None
        }
        "bessel_y" | "bessely" if args.len() >= 2 => {
            if let Expression::Number(Number::Integer(n)) = &args[0] {
                let order = (*n) as i32;
                return Some(crate::functions::special::bessel_y(order, &args[1]));
            }
            None
        }
        "zeta" => Some(crate::functions::special::zeta(&args[0])),
        "erf" => Some(crate::functions::special::erf(&args[0])),
        "erfc" => Some(crate::functions::special::erfc(&args[0])),
        "factorial" => Some(crate::functions::special::factorial(&args[0])),
        "beta" if args.len() >= 2 => Some(crate::functions::special::beta(&args[0], &args[1])),
        "gcd" if args.len() >= 2 => Some(args[0].gcd(&args[1])),
        "lcm" if args.len() >= 2 => Some(crate::functions::number_theory_eval::lcm(
            &args[0], &args[1],
        )),
        "mod" if args.len() >= 2 => Some(crate::functions::number_theory_eval::modulo(
            &args[0], &args[1],
        )),
        "isprime" => Some(crate::functions::number_theory_eval::isprime(&args[0])),
        "degree" if args.len() >= 2 => {
            if let Expression::Symbol(var) = &args[1] {
                return Some(crate::functions::polynomials::degree(&args[0], var));
            }
            None
        }
        "roots" if args.len() >= 2 => {
            if let Expression::Symbol(var) = &args[1] {
                return Some(crate::functions::polynomials::roots(&args[0], var));
            }
            None
        }
        "expand" => Some(crate::functions::polynomials::expand(&args[0])),
        "factor" => Some(crate::functions::polynomials::factor(&args[0])),
        "undefined" => Some(Expression::constant(MathConstant::Undefined)),
        "legendrep" | "legendre_p" if args.len() >= 2 => evaluate_orthogonal_polynomial(
            &args[0],
            &args[1],
            crate::functions::polynomials::symbolic::expand_legendre_symbolic,
        ),
        "chebyshevt" | "chebyshev_t" | "chebyshev_first" if args.len() >= 2 => {
            evaluate_orthogonal_polynomial(
                &args[0],
                &args[1],
                crate::functions::polynomials::symbolic::expand_chebyshev_first_symbolic,
            )
        }
        "chebyshevu" | "chebyshev_u" | "chebyshev_second" if args.len() >= 2 => {
            evaluate_orthogonal_polynomial(
                &args[0],
                &args[1],
                crate::functions::polynomials::symbolic::expand_chebyshev_second_symbolic,
            )
        }
        "hermiteh" | "hermite" if args.len() >= 2 => evaluate_orthogonal_polynomial(
            &args[0],
            &args[1],
            crate::functions::polynomials::symbolic::expand_hermite_symbolic,
        ),
        "laguerrel" | "laguerre" if args.len() >= 2 => evaluate_orthogonal_polynomial(
            &args[0],
            &args[1],
            crate::functions::polynomials::symbolic::expand_laguerre_symbolic,
        ),
        _ => None,
    }
}

/// Helper for evaluating orthogonal polynomials
///
/// Extracts the degree `n` from the first argument and substitutes the
/// variable from the second argument into the expanded polynomial.
///
/// # Arguments
///
/// * `degree_arg` - Expression containing the polynomial degree (must be non-negative integer)
/// * `var_arg` - Expression for the variable (typically a Symbol, but can be any expression)
/// * `expander` - Function that expands the polynomial symbolically
///
/// # Returns
///
/// Some(Expression) if degree is a valid non-negative integer, None otherwise
///
/// # Example
///
/// For `legendrep(2, x)`:
/// 1. Extract n=2 from degree_arg
/// 2. Call expander(2) to get P_2(x) = (3x^2 - 1)/2
/// 3. If var_arg is just `x`, return the expanded form
/// 4. Otherwise substitute var_arg for x and simplify
pub fn evaluate_orthogonal_polynomial<F>(
    degree_arg: &Expression,
    var_arg: &Expression,
    expander: F,
) -> Option<Expression>
where
    F: Fn(usize) -> Expression,
{
    let n = match degree_arg {
        Expression::Number(Number::Integer(i)) if *i >= 0 => *i as usize,
        Expression::Number(Number::BigInteger(bi)) => {
            if let Some(n) = bi.to_u64() {
                n as usize
            } else {
                return None;
            }
        }
        _ => return None,
    };

    let expanded = expander(n);

    match var_arg {
        Expression::Symbol(sym) if sym.name() == "x" => Some(expanded),
        _ => {
            let mut subs = HashMap::new();
            subs.insert("x".to_owned(), var_arg.clone());
            Some(expanded.substitute(&subs).simplify())
        }
    }
}

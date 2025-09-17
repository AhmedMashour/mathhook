//! U-substitution for integration
//!
//! Implements automatic u-substitution detection and execution for composite
//! functions. Handles patterns like f'(g(x)) * g'(x) by substituting u = g(x).
//!
//! # Algorithm
//!
//! 1. Identify candidate substitutions u = g(x) from the integrand structure
//! 2. Compute du = g'(x) dx for each candidate
//! 3. Check if integrand can be rewritten as f(u) * du (possibly with constant factor)
//! 4. Integrate f(u) with respect to u
//! 5. Substitute back u = g(x) in the result
//!
//! # Supported Patterns
//!
//! - Polynomial inner functions: ∫2x*sin(x²) dx = -cos(x²)
//! - Exponential compositions: ∫e^x*sin(e^x) dx = -cos(e^x)
//! - Logarithmic patterns: ∫1/(x*ln(x)) dx = ln|ln(x)|
//! - Rational functions: ∫x/(x²+1) dx = (1/2)*ln(x²+1)
//! - Linear inner functions: ∫sqrt(x+1) dx = (2/3)(x+1)^(3/2)
//!
//! # Patterns Recognized
//!
//! **Pattern 1**: `f'(x)·g(f(x))` - Exact derivative match
//! - Example: `2x·e^(x²)` where u = x², du = 2x dx
//!
//! **Pattern 2**: `c·f'(x)·g(f(x))` - Derivative with coefficient
//! - Example: `x·sin(x²)` where u = x², du = 2x dx, coefficient = 1/2
//!
//! **Pattern 3**: `f^n(x)·f'(x)` - Power of function times derivative
//! - Example: `sin³(x)·cos(x)` where u = sin(x), du = cos(x) dx
//!
//! **Pattern 4**: `f(ax+b)` - Constant derivative (linear inner function)
//! - Example: `sqrt(x+1)` where u = x+1, du = 1 dx (constant derivative)

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Maximum recursion depth for integration attempts.
///
/// Based on analysis of SymPy's integration behavior, most legitimate
/// integrals converge within 3-4 recursive calls. A limit of 10 provides
/// a safe margin while preventing pathological infinite recursion cases.
///
/// This depth limit prevents infinite loops in cases like:
/// - Circular substitutions (u = v, v = u)
/// - Self-referential patterns that don't simplify
/// - Nested substitutions that don't converge
///
/// **SymPy Comparison**: SymPy uses cache-based loop detection, but our
/// MAX_DEPTH approach is simpler, safer, and sufficient for MathHook's needs.
const MAX_DEPTH: usize = 10;

/// Try to integrate using u-substitution
///
/// Automatically detects composite function patterns and applies substitution.
///
/// # Arguments
///
/// * `expr` - The integrand expression
/// * `var` - The variable of integration
/// * `depth` - Current recursion depth (prevents infinite recursion)
///
/// # Returns
///
/// Some(result) if substitution succeeds, None if no suitable substitution found
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::substitution::try_substitution;
/// use mathhook_core::symbol;
/// use mathhook_core::core::Expression;
///
/// let x = symbol!(x);
/// // ∫2x*sin(x²) dx
/// let integrand = Expression::mul(vec![
///     Expression::integer(2),
///     Expression::symbol(x.clone()),
///     Expression::function("sin", vec![
///         Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
///     ])
/// ]);
///
/// let result = try_substitution(&integrand, &x, 0);
/// assert!(result.is_some());
/// ```
pub fn try_substitution(expr: &Expression, var: &Symbol, depth: usize) -> Option<Expression> {
    if depth >= MAX_DEPTH {
        return None;
    }

    let candidates = find_substitution_candidates(expr, var);

    for candidate in candidates.iter() {
        let g_prime = candidate.derivative(var.clone());

        if let Some((f_of_u, constant_factor)) =
            check_derivative_match(expr, candidate, &g_prime, var)
        {
            let u_symbol = Symbol::scalar("u");
            let u_expr = Expression::symbol(u_symbol.clone());

            let integrated = integrate_in_u(&f_of_u, u_symbol, depth)?;

            let result = substitute_back(&integrated, &u_expr, candidate);

            let final_result = if (constant_factor - 1.0).abs() > 1e-10 {
                if constant_factor.abs() < 1.0 {
                    let denom = (1.0 / constant_factor) as i64;
                    Expression::mul(vec![Expression::rational(1, denom), result])
                } else {
                    let numer = constant_factor as i64;
                    Expression::mul(vec![Expression::integer(numer), result])
                }
            } else {
                result
            };

            return Some(final_result);
        }
    }

    None
}

/// Find candidate expressions for substitution u = g(x)
///
/// Looks for inner functions, polynomial expressions, exponential/logarithm arguments.
fn find_substitution_candidates(expr: &Expression, var: &Symbol) -> Vec<Expression> {
    let mut candidates = Vec::new();

    collect_candidates_recursive(expr, var, &mut candidates);

    candidates.sort_by_key(|c| std::cmp::Reverse(expression_complexity(c)));
    candidates.dedup_by(|a, b| expressions_equivalent(a, b));

    candidates
}

/// Recursively collect substitution candidates from expression tree
fn collect_candidates_recursive(expr: &Expression, var: &Symbol, candidates: &mut Vec<Expression>) {
    match expr {
        Expression::Function { name: _, args } => {
            // For function arguments, consider the function itself as a candidate
            // Example: sin(x) is a candidate in sin³(x)·cos(x)
            if args.len() == 1 && args[0].contains_variable(var) {
                // If argument is just x, consider the whole function
                if is_simple_variable(&args[0], var) {
                    candidates.push(expr.clone());
                } else {
                    // If argument is composite, consider the argument
                    candidates.push(args[0].clone());
                }
            }
            for arg in args.iter() {
                if arg.contains_variable(var) && !is_simple_variable(arg, var) {
                    candidates.push(arg.clone());
                }
                collect_candidates_recursive(arg, var, candidates);
            }
        }
        Expression::Pow(base, exp) => {
            if base.contains_variable(var) && !is_simple_variable(base, var) {
                candidates.push((**base).clone());
            }
            if exp.contains_variable(var) && !is_simple_variable(exp, var) {
                candidates.push((**exp).clone());
            }
            collect_candidates_recursive(base, var, candidates);
            collect_candidates_recursive(exp, var, candidates);
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                collect_candidates_recursive(term, var, candidates);
            }
        }
        Expression::Mul(factors) => {
            for factor in factors.iter() {
                collect_candidates_recursive(factor, var, candidates);
            }
        }
        _ => {}
    }
}

/// Check if expression contains the given candidate expression
///
/// This is used to separate f(u) from g'(x): factors containing the candidate are f(u)
fn contains_expression(expr: &Expression, candidate: &Expression) -> bool {
    if expr == candidate {
        return true;
    }

    match expr {
        Expression::Add(terms) => terms.iter().any(|t| contains_expression(t, candidate)),
        Expression::Mul(factors) => factors.iter().any(|f| contains_expression(f, candidate)),
        Expression::Pow(base, exp) => {
            contains_expression(base, candidate) || contains_expression(exp, candidate)
        }
        Expression::Function { name: _, args } => {
            args.iter().any(|a| contains_expression(a, candidate))
        }
        _ => false,
    }
}

/// Check if expression is just the variable itself
fn is_simple_variable(expr: &Expression, var: &Symbol) -> bool {
    matches!(expr, Expression::Symbol(s) if s == var)
}

/// Measure complexity of expression (for prioritizing candidates)
fn expression_complexity(expr: &Expression) -> usize {
    match expr {
        Expression::Number(_) | Expression::Symbol(_) | Expression::Constant(_) => 1,
        Expression::Add(terms) => terms.iter().map(expression_complexity).sum::<usize>() + 1,
        Expression::Mul(factors) => factors.iter().map(expression_complexity).sum::<usize>() + 1,
        Expression::Pow(base, exp) => expression_complexity(base) + expression_complexity(exp) + 1,
        Expression::Function { name: _, args } => {
            args.iter().map(expression_complexity).sum::<usize>() + 2
        }
        _ => 1,
    }
}

/// Check if two expressions are equivalent
fn expressions_equivalent(a: &Expression, b: &Expression) -> bool {
    a == b
}

/// Check if derivative is constant (does not depend on variable)
fn is_constant_derivative(g_prime: &Expression, var: &Symbol) -> bool {
    !g_prime.contains_variable(var)
}

/// Check if derivative g'(x) appears in the integrand
///
/// Returns Some((f(u), constant_factor)) if a match is found, where:
/// - f(u) is the expression in terms of u
/// - constant_factor accounts for numerical differences between g'(x) and actual factor
///
/// This function recognizes patterns where the derivative appears as:
/// 1. Exact match: `g'(x)` appears as-is
/// 2. With coefficient: `c·g'(x)` where c is a constant
/// 3. Distributed across factors: g'(x) = a*b and both a and b appear separately in the product
/// 4. Constant derivative (implicit): For f(ax+b), derivative is constant and doesn't appear explicitly
fn check_derivative_match(
    expr: &Expression,
    g: &Expression,
    g_prime: &Expression,
    var: &Symbol,
) -> Option<(Expression, f64)> {
    let expr_simplified = expr.clone().simplify();
    let g_prime_simplified = g_prime.clone().simplify();

    // Pattern 4: Check for constant derivative (linear inner function like x+1, 2x+3)
    // For expressions like sqrt(x+1), derivative is 1 (constant), not appearing explicitly
    if is_constant_derivative(&g_prime_simplified, var) {
        // Extract the constant value of the derivative
        if let Some(derivative_value) = extract_constant_value(&g_prime_simplified) {
            // If the expression contains g, we can use it for substitution
            if contains_expression(&expr_simplified, g) {
                let u_symbol = Symbol::scalar("u");
                let u_expr = Expression::symbol(u_symbol);

                // Replace g with u in the entire expression
                let f_of_u = replace_expression(&expr_simplified, g, &u_expr);

                // Return with reciprocal of derivative as constant factor
                // For du = c·dx, we need 1/c factor when substituting
                return Some((f_of_u, 1.0 / derivative_value));
            }
        }
    }

    if let Expression::Mul(factors) = &expr_simplified {
        let u_symbol = Symbol::scalar("u");
        let u_expr = Expression::symbol(u_symbol);

        // NEW STRATEGY: Separate factors into:
        // 1. Those that contain the candidate g (these are f(u))
        // 2. The rest (these could be g'(x) or constants)
        let (f_of_g_factors, derivative_candidate_factors): (Vec<_>, Vec<_>) =
            factors.iter().partition(|f| contains_expression(f, g));

        if !f_of_g_factors.is_empty() && !derivative_candidate_factors.is_empty() {
            // Reconstruct what we think is the derivative from available factors
            let derivative_candidate = if derivative_candidate_factors.len() == 1 {
                derivative_candidate_factors[0].clone()
            } else {
                Expression::mul(
                    derivative_candidate_factors
                        .iter()
                        .map(|f| (*f).clone())
                        .collect(),
                )
            };

            // Check if this matches the derivative (possibly with a constant ratio)
            if let Some(ratio) = compute_constant_ratio(&derivative_candidate, &g_prime_simplified)
            {
                // Success! We found the derivative (with coefficient ratio)
                // The remaining factors (those containing g) become f(u)
                let remaining = if f_of_g_factors.is_empty() {
                    Expression::integer(1)
                } else if f_of_g_factors.len() == 1 {
                    f_of_g_factors[0].clone()
                } else {
                    Expression::mul(f_of_g_factors.iter().map(|f| (*f).clone()).collect())
                };

                // Replace g with u in the remaining expression
                let f_of_u = replace_expression(&remaining, g, &u_expr);

                return Some((f_of_u, ratio));
            }
        }

        // Fallback: Try the old partitioning strategy for backward compatibility
        let (derivative_factors, other_factors): (Vec<_>, Vec<_>) = factors
            .iter()
            .partition(|f| factor_matches_derivative(f, &g_prime_simplified, var));

        if derivative_factors.is_empty() {
            return None;
        }

        let derivative_product = if derivative_factors.len() == 1 {
            derivative_factors[0].clone()
        } else {
            Expression::mul(derivative_factors.iter().map(|f| (*f).clone()).collect())
        };

        let constant_factor = compute_constant_ratio(&derivative_product, &g_prime_simplified)?;

        let remaining = if other_factors.is_empty() {
            Expression::integer(1)
        } else if other_factors.len() == 1 {
            other_factors[0].clone()
        } else {
            Expression::mul(other_factors.iter().map(|f| (*f).clone()).collect())
        };

        let f_of_u = replace_expression(&remaining, g, &u_expr);

        Some((f_of_u, constant_factor))
    } else {
        let constant_factor = compute_constant_ratio(&expr_simplified, &g_prime_simplified)?;
        let f_of_u = Expression::integer(1);
        Some((f_of_u, constant_factor))
    }
}

/// Extract constant value from an expression that doesn't depend on variables
///
/// Returns Some(value) if expression is a constant number, None otherwise
fn extract_constant_value(expr: &Expression) -> Option<f64> {
    match expr {
        Expression::Number(n) => number_to_f64(n),
        _ => None,
    }
}

/// Replace all occurrences of `pattern` with `replacement` in `expr`
///
/// This is used to convert f(g(x)) to f(u) by replacing g(x) with u.
fn replace_expression(
    expr: &Expression,
    pattern: &Expression,
    replacement: &Expression,
) -> Expression {
    // Check if the entire expression matches the pattern
    if expr == pattern {
        return replacement.clone();
    }

    // Recursively replace in subexpressions
    match expr {
        Expression::Add(terms) => Expression::add(
            terms
                .iter()
                .map(|t| replace_expression(t, pattern, replacement))
                .collect(),
        ),
        Expression::Mul(factors) => Expression::mul(
            factors
                .iter()
                .map(|f| replace_expression(f, pattern, replacement))
                .collect(),
        ),
        Expression::Pow(base, exp) => Expression::pow(
            replace_expression(base, pattern, replacement),
            replace_expression(exp, pattern, replacement),
        ),
        Expression::Function { name, args } => Expression::function(
            name,
            args.iter()
                .map(|a| replace_expression(a, pattern, replacement))
                .collect(),
        ),
        _ => expr.clone(),
    }
}

/// Check if a factor matches the derivative (possibly with constant multiple)
fn factor_matches_derivative(factor: &Expression, derivative: &Expression, var: &Symbol) -> bool {
    if factor == derivative {
        return true;
    }

    let factor_simplified = factor.clone().simplify();
    let derivative_simplified = derivative.clone().simplify();

    if factor_simplified == derivative_simplified {
        return true;
    }

    if let (Expression::Mul(f_factors), Expression::Mul(d_factors)) =
        (&factor_simplified, &derivative_simplified)
    {
        let f_non_const: Vec<_> = f_factors
            .iter()
            .filter(|f| f.contains_variable(var))
            .collect();
        let d_non_const: Vec<_> = d_factors
            .iter()
            .filter(|f| f.contains_variable(var))
            .collect();

        if f_non_const.len() == d_non_const.len() {
            return f_non_const
                .iter()
                .zip(d_non_const.iter())
                .all(|(f, d)| f == d);
        }
    }

    match (&factor_simplified, &derivative_simplified) {
        (Expression::Symbol(f_sym), Expression::Symbol(d_sym)) => f_sym == d_sym,
        (Expression::Pow(f_base, f_exp), Expression::Pow(d_base, d_exp)) => {
            f_base == d_base && f_exp == d_exp
        }
        _ => false,
    }
}

/// Compute constant ratio between two expressions
///
/// Returns Some(ratio) where expr = ratio * target
/// This handles cases like:
/// - expr = 2x, target = 2x → ratio = 1.0
/// - expr = x, target = 2x → ratio = 0.5
/// - expr = 3x, target = 2x → ratio = 1.5
fn compute_constant_ratio(expr: &Expression, target: &Expression) -> Option<f64> {
    if expr == target {
        return Some(1.0);
    }

    let expr_simp = expr.clone().simplify();
    let target_simp = target.clone().simplify();

    if expr_simp == target_simp {
        return Some(1.0);
    }

    // Try to match structurally by extracting coefficients
    match (&expr_simp, &target_simp) {
        (Expression::Number(n1), Expression::Number(n2)) => {
            let v1 = number_to_f64(n1)?;
            let v2 = number_to_f64(n2)?;
            if v2.abs() > 1e-10 {
                let ratio = v1 / v2;
                Some(ratio)
            } else {
                None
            }
        }
        // Both are products - try to extract coefficients
        (Expression::Mul(e_factors), Expression::Mul(t_factors)) => {
            let e_coeff = extract_coefficient(e_factors);
            let t_coeff = extract_coefficient(t_factors);

            let e_non_const: Vec<_> = e_factors
                .iter()
                .filter(|f| !matches!(f, Expression::Number(_)))
                .collect();
            let t_non_const: Vec<_> = t_factors
                .iter()
                .filter(|f| !matches!(f, Expression::Number(_)))
                .collect();

            // Check if non-constant parts match
            if e_non_const.len() == t_non_const.len()
                && e_non_const
                    .iter()
                    .zip(t_non_const.iter())
                    .all(|(a, b)| *a == *b)
                && t_coeff.abs() > 1e-10
            {
                let ratio = e_coeff / t_coeff;
                return Some(ratio);
            }
            None
        }
        // expr is product, target is not - check if they match structurally
        (Expression::Mul(factors), _) => {
            let coeff = extract_coefficient(factors);
            let non_const: Vec<_> = factors
                .iter()
                .filter(|f| !matches!(f, Expression::Number(_)))
                .collect();

            let non_const_product = if non_const.is_empty() {
                Expression::integer(1)
            } else if non_const.len() == 1 {
                (*non_const[0]).clone()
            } else {
                Expression::mul(non_const.iter().map(|f| (*f).clone()).collect())
            };

            if non_const_product == target_simp {
                Some(coeff)
            } else {
                None
            }
        }
        // target is product, expr is not
        (_, Expression::Mul(factors)) => {
            let coeff = extract_coefficient(factors);
            let non_const: Vec<_> = factors
                .iter()
                .filter(|f| !matches!(f, Expression::Number(_)))
                .collect();

            let non_const_product = if non_const.is_empty() {
                Expression::integer(1)
            } else if non_const.len() == 1 {
                (*non_const[0]).clone()
            } else {
                Expression::mul(non_const.iter().map(|f| (*f).clone()).collect())
            };

            if expr_simp == non_const_product && coeff.abs() > 1e-10 {
                let ratio = 1.0 / coeff;
                Some(ratio)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extract numeric coefficient from a product of factors
///
/// Returns the product of all numeric factors, or 1.0 if there are none
fn extract_coefficient(factors: &[Expression]) -> f64 {
    let nums: Vec<f64> = factors
        .iter()
        .filter_map(|f| {
            if let Expression::Number(n) = f {
                number_to_f64(n)
            } else {
                None
            }
        })
        .collect();

    if nums.is_empty() {
        1.0
    } else {
        nums.iter().product()
    }
}

/// Convert Number to f64
fn number_to_f64(num: &Number) -> Option<f64> {
    match num {
        Number::Integer(i) => Some(*i as f64),
        Number::Rational(r) => {
            use num_traits::ToPrimitive;
            r.to_f64()
        }
        Number::Float(f) => Some(*f),
        _ => None,
    }
}

/// Integrate expression with respect to u
///
/// Depth is incremented and passed to prevent infinite recursion in nested substitutions.
/// When depth reaches MAX_DEPTH, integration returns None to break recursion chains.
fn integrate_in_u(expr: &Expression, u: Symbol, depth: usize) -> Option<Expression> {
    use crate::calculus::integrals::strategy::integrate_with_strategy;

    let result = integrate_with_strategy(expr, u, depth + 1);

    if matches!(result, Expression::Calculus(_)) {
        None
    } else {
        Some(result)
    }
}

/// Substitute u = g(x) back into the result
///
/// After integrating f(u), we have a result in terms of u.
/// This function replaces u with g(x) to get the final answer.
fn substitute_back(expr: &Expression, u: &Expression, g: &Expression) -> Expression {
    replace_expression(expr, u, g)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_is_simple_variable() {
        let x = symbol!(x);

        assert!(is_simple_variable(&Expression::symbol(x.clone()), &x));
        assert!(!is_simple_variable(&Expression::integer(5), &x));
        assert!(!is_simple_variable(
            &Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            &x
        ));
    }

    #[test]
    fn test_expression_complexity() {
        let x = symbol!(x);

        assert_eq!(expression_complexity(&Expression::integer(5)), 1);
        assert_eq!(expression_complexity(&Expression::symbol(x.clone())), 1);

        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert_eq!(expression_complexity(&x_squared), 3);

        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert_eq!(expression_complexity(&sin_x), 3);
    }

    #[test]
    fn test_find_substitution_candidates_basic() {
        let x = symbol!(x);
        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let sin_x_squared = Expression::function("sin", vec![x_squared.clone()]);

        let candidates = find_substitution_candidates(&sin_x_squared, &x);

        assert!(!candidates.is_empty());
        assert!(candidates.contains(&x_squared));
    }

    #[test]
    fn test_replace_expression() {
        let x = symbol!(x);
        let u = symbol!(u);

        // Test replacing x² with u in exp(x²)
        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let expr = Expression::function("exp", vec![x_squared.clone()]);
        let u_expr = Expression::symbol(u.clone());

        let result = replace_expression(&expr, &x_squared, &u_expr);
        let expected = Expression::function("exp", vec![u_expr]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_constant_derivative() {
        let x = symbol!(x);

        // Constant expressions don't contain x
        assert!(is_constant_derivative(&Expression::integer(1), &x));
        assert!(is_constant_derivative(&Expression::integer(2), &x));
        assert!(is_constant_derivative(&Expression::rational(3, 2), &x));

        // Non-constant expressions contain x
        assert!(!is_constant_derivative(&Expression::symbol(x.clone()), &x));
        assert!(!is_constant_derivative(
            &Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            &x
        ));
    }

    #[test]
    fn test_extract_constant_value() {
        assert_eq!(extract_constant_value(&Expression::integer(1)), Some(1.0));
        assert_eq!(extract_constant_value(&Expression::integer(5)), Some(5.0));
        assert_eq!(
            extract_constant_value(&Expression::rational(3, 2)),
            Some(1.5)
        );

        let x = symbol!(x);
        assert_eq!(extract_constant_value(&Expression::symbol(x.clone())), None);
    }

    #[test]
    fn test_exponential_chain_rule_pattern() {
        // Test 3: ∫2x·e^(x²) dx
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::function(
                "exp",
                vec![Expression::pow(
                    Expression::symbol(x.clone()),
                    Expression::integer(2),
                )],
            ),
        ]);

        let result = try_substitution(&expr, &x, 0);
        assert!(
            result.is_some(),
            "Exponential chain rule pattern should succeed"
        );
    }

    #[test]
    fn test_trig_substitution_with_coefficient() {
        // Test 4: ∫x·sin(x²) dx
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function(
                "sin",
                vec![Expression::pow(
                    Expression::symbol(x.clone()),
                    Expression::integer(2),
                )],
            ),
        ]);

        let result = try_substitution(&expr, &x, 0);
        assert!(
            result.is_some(),
            "Trig substitution with coefficient should succeed"
        );
    }

    #[test]
    fn test_power_chain_rule_pattern() {
        // Test 7: ∫sin³(x)·cos(x) dx
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::pow(
                Expression::function("sin", vec![Expression::symbol(x.clone())]),
                Expression::integer(3),
            ),
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
        ]);

        let result = try_substitution(&expr, &x, 0);
        assert!(result.is_some(), "Power chain rule pattern should succeed");
    }

    #[test]
    fn test_constant_derivative_linear() {
        // Test: ∫sqrt(x+1) dx - constant derivative case
        let x = symbol!(x);
        let inner = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        let expr = Expression::function("sqrt", vec![inner.clone()]);

        let result = try_substitution(&expr, &x, 0);
        assert!(
            result.is_some(),
            "Constant derivative substitution should succeed for sqrt(x+1)"
        );
    }

    #[test]
    fn test_max_depth_prevents_infinite_recursion() {
        let x = symbol!(x);

        let simple_expr = Expression::symbol(x.clone());
        let _result_at_limit = try_substitution(&simple_expr, &x, MAX_DEPTH - 1);

        let result_over_limit = try_substitution(&simple_expr, &x, MAX_DEPTH);
        assert_eq!(
            result_over_limit, None,
            "Should return None when depth >= MAX_DEPTH"
        );
    }
}

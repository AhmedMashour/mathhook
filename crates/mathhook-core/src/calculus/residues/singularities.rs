//! Singularity classification for complex analysis
//!
//! Implements classification of singularities into removable, poles, and essential types.

use crate::calculus::limits::Limits;
use crate::core::{Expression, Symbol};

use super::helpers::{is_expression_zero, is_finite};

/// Types of singularities in complex analysis
#[derive(Debug, Clone, PartialEq)]
pub enum SingularityType {
    /// Removable singularity (limit exists and is finite)
    Removable,
    /// Pole of given order
    Pole(u32),
    /// Essential singularity (neither removable nor pole)
    Essential,
    /// Unable to classify
    Unknown,
}

/// Trait for complex analysis operations related to singularities
pub trait ComplexAnalysis {
    /// Check if function is analytic at a point
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ComplexAnalysis;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::pow(Expression::symbol(z.clone()), Expression::integer(2));
    /// let point = Expression::integer(1);
    /// let is_analytic = expr.is_analytic_at(&z, &point);
    /// ```
    fn is_analytic_at(&self, variable: &Symbol, point: &Expression) -> bool;

    /// Determine the order of a pole
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ComplexAnalysis;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::pow(
    ///     Expression::pow(
    ///         Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-1)]),
    ///         Expression::integer(2)
    ///     ),
    ///     Expression::integer(-1)
    /// );
    /// let pole = Expression::integer(1);
    /// let order = expr.pole_order(&z, &pole);
    /// ```
    fn pole_order(&self, variable: &Symbol, pole: &Expression) -> u32;

    /// Check if point is a removable singularity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ComplexAnalysis;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::mul(vec![
    ///     Expression::function("sin", vec![Expression::symbol(z.clone())]),
    ///     Expression::pow(Expression::symbol(z.clone()), Expression::integer(-1))
    /// ]);
    /// let point = Expression::integer(0);
    /// let is_removable = expr.is_removable_singularity(&z, &point);
    /// ```
    fn is_removable_singularity(&self, variable: &Symbol, point: &Expression) -> bool;

    /// Check if point is an essential singularity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ComplexAnalysis;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::function(
    ///     "exp",
    ///     vec![Expression::pow(Expression::symbol(z.clone()), Expression::integer(-1))]
    /// );
    /// let point = Expression::integer(0);
    /// let is_essential = expr.is_essential_singularity(&z, &point);
    /// ```
    fn is_essential_singularity(&self, variable: &Symbol, point: &Expression) -> bool;
}

impl ComplexAnalysis for Expression {
    fn is_analytic_at(&self, variable: &Symbol, point: &Expression) -> bool {
        use crate::calculus::ResidueCalculus;
        // A function is analytic if it has no singularities
        let poles = self.find_poles(variable);
        !poles.contains(point)
    }

    fn pole_order(&self, variable: &Symbol, pole: &Expression) -> u32 {
        use crate::calculus::derivatives::Derivative;
        use crate::calculus::integrals::rational::helpers::substitute_variable;
        use crate::simplify::Simplify;

        use super::helpers::extract_denominator;

        // Extract the denominator from the expression
        let denominator = match extract_denominator(self) {
            Some(denom) => denom,
            None => return 0, // Not a rational function, no pole
        };

        // Check if the denominator is actually zero at the pole point
        let denom_at_pole = substitute_variable(&denominator, variable, pole).simplify();
        if !is_expression_zero(&denom_at_pole) {
            return 0; // Denominator is non-zero at this point, so no pole
        }

        // Special case: denominator is (expr)^n where expr evaluates to zero at pole
        if let Expression::Pow(base, exp) = &denominator {
            // Check if base is zero at the pole
            let base_at_pole = substitute_variable(base, variable, pole).simplify();
            if is_expression_zero(&base_at_pole) {
                // Extract the exponent if it's a positive integer
                if let Expression::Number(crate::core::Number::Integer(n)) = exp.as_ref() {
                    if *n > 0 {
                        return *n as u32;
                    }
                }
            }
        }

        // General case: Use repeated differentiation
        const MAX_ORDER: u32 = 100;
        let mut current_deriv = denominator;

        for order in 1..=MAX_ORDER {
            current_deriv = current_deriv.derivative(variable.clone());
            let deriv_at_pole = substitute_variable(&current_deriv, variable, pole).simplify();

            if !is_expression_zero(&deriv_at_pole) {
                return order;
            }
        }

        // If we reach here, either it's an essential singularity or numerical issues
        0
    }

    fn is_removable_singularity(&self, variable: &Symbol, point: &Expression) -> bool {
        // A singularity is removable if:
        // 1. The function is not defined at the point (has singularity)
        // 2. BUT lim(z→point) f(z) exists and is finite

        // Compute the limit at the point
        let limit_result = self.limit(variable, point);

        // Check if the limit is finite (not infinity, not undefined)
        is_finite(&limit_result)
    }

    fn is_essential_singularity(&self, variable: &Symbol, point: &Expression) -> bool {
        // Essential singularities occur when a function has infinitely many
        // negative power terms in its Laurent series expansion.
        // Common patterns: exp(1/z), sin(1/z), cos(1/z) at z=0

        // Check for common essential singularity patterns
        match self {
            // exp(1/(z-point)) at z=point is an essential singularity
            Expression::Function { name, args } if name == "exp" && args.len() == 1 => {
                has_reciprocal_of_shifted_variable(&args[0], variable, point)
            }
            // sin(1/(z-point)) at z=point is an essential singularity
            Expression::Function { name, args } if name == "sin" && args.len() == 1 => {
                has_reciprocal_of_shifted_variable(&args[0], variable, point)
            }
            // cos(1/(z-point)) at z=point is an essential singularity
            Expression::Function { name, args } if name == "cos" && args.len() == 1 => {
                has_reciprocal_of_shifted_variable(&args[0], variable, point)
            }
            // log(z-point) at z=point is a logarithmic singularity (branch point)
            Expression::Function { name, args } if name == "log" && args.len() == 1 => {
                is_shifted_variable(&args[0], variable, point)
            }
            _ => false,
        }
    }
}

/// Check if expression is 1/(variable - point)
///
/// # Arguments
///
/// * `expr` - The expression to check
/// * `variable` - The variable symbol
/// * `point` - The point to check against
///
/// # Returns
///
/// `true` if expr represents 1/(variable - point), `false` otherwise
fn has_reciprocal_of_shifted_variable(
    expr: &Expression,
    variable: &Symbol,
    point: &Expression,
) -> bool {
    use crate::calculus::integrals::rational::helpers::substitute_variable;
    use crate::simplify::Simplify;

    use super::helpers::{is_infinity, is_undefined};

    // Check for (variable - point)^(-1) or similar patterns
    match expr {
        // Direct power: (z-point)^(-1)
        Expression::Pow(base, exp) => {
            if let Expression::Number(crate::core::Number::Integer(n)) = exp.as_ref() {
                if *n == -1 && is_shifted_variable(base, variable, point) {
                    return true;
                }
            }
            false
        }
        // Multiplication form: constant * (z-point)^(-1)
        Expression::Mul(factors) => factors
            .iter()
            .any(|f| has_reciprocal_of_shifted_variable(f, variable, point)),
        // Check if it simplifies to the pattern
        _ => {
            // Try evaluating at a test point near the singularity
            let test_expr = substitute_variable(expr, variable, point);
            is_infinity(&test_expr.simplify()) || is_undefined(&test_expr.simplify())
        }
    }
}

/// Check if expression equals (variable - point)
///
/// # Arguments
///
/// * `expr` - The expression to check
/// * `variable` - The variable symbol
/// * `point` - The point to check against
///
/// # Returns
///
/// `true` if expr represents (variable - point), `false` otherwise
fn is_shifted_variable(expr: &Expression, variable: &Symbol, point: &Expression) -> bool {
    // Check if expr is of the form (variable - point)
    match expr {
        // Direct variable match (when point is 0)
        Expression::Symbol(s) if s == variable && is_expression_zero(point) => true,
        // Addition: variable + (-point)
        Expression::Add(terms) if terms.len() == 2 => {
            let has_variable = terms
                .iter()
                .any(|t| matches!(t, Expression::Symbol(s) if s == variable));
            let has_negative_point = terms.iter().any(|t| {
                matches!(t, Expression::Mul(factors) if factors.len() == 2
                    && matches!(&factors[0], Expression::Number(n) if *n == crate::core::Number::Integer(-1))
                    && &factors[1] == point)
            });
            has_variable && has_negative_point
        }
        _ => false,
    }
}

/// Determine singularity type at a point
///
/// Uses pole order analysis and limit evaluation to classify singularities:
/// - Pole: `pole_order() > 0`
/// - Removable: limit exists and is finite
/// - Essential: neither pole nor removable (e.g., exp(1/z) at z=0)
///
/// # Arguments
///
/// * `expr` - The expression to analyze
/// * `variable` - The variable for the singularity
/// * `point` - The point to classify the singularity at
///
/// # Returns
///
/// The type of singularity: Removable, Pole(order), Essential, or Unknown
pub fn classify_singularity(
    expr: &Expression,
    variable: &Symbol,
    point: &Expression,
) -> SingularityType {
    // First check if it's a pole
    let order = expr.pole_order(variable, point);
    if order > 0 {
        return SingularityType::Pole(order);
    }

    // Check if it's a removable singularity
    if expr.is_removable_singularity(variable, point) {
        return SingularityType::Removable;
    }

    // Check if it's an essential singularity
    if expr.is_essential_singularity(variable, point) {
        return SingularityType::Essential;
    }

    // Unable to classify
    SingularityType::Unknown
}

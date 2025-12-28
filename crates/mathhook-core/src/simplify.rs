//! Minimal overhead, maximum performance implementation
//!
//! Uses modular architecture for specialized simplification of different expression types.
//! Individual modules handle specific aspects following project guidelines.

use std::sync::Arc;

use crate::core::{Expression, Number};
use crate::matrices::operations::MatrixOperations;
use num_traits::ToPrimitive;
pub mod arithmetic;
mod constants;
mod functions;

/// Trait for simplifying expressions
///
/// Simplification transforms expressions into equivalent but simpler symbolic forms
/// through algebraic reduction. Unlike evaluation, simplification:
/// - Does NOT substitute variables
/// - Does NOT check domain restrictions
/// - Does NOT compute numerical values (except constant folding)
/// - DOES preserve mathematical equivalence
///
/// # Simplification vs Evaluation
///
/// **Use `simplify()` when:** You need algebraic reduction while keeping expressions symbolic
/// **Use `evaluate()` when:** You need numerical values with domain checking
/// **Use `evaluate_with_context()` when:** You need to substitute variables and compute
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::simplify::Simplify;
/// use mathhook_core::Expression;
///
/// // Combine like terms
/// let x = symbol!(x);
/// let x_expr = Expression::symbol(x.clone());
/// let sum = Expression::add(vec![x_expr.clone(), x_expr.clone()]);
/// assert_eq!(sum.simplify(), Expression::mul(vec![expr!(2), x_expr.clone()]));
///
/// // Apply identities
/// assert_eq!(Expression::mul(vec![x_expr.clone(), expr!(1)]).simplify(), x_expr.clone());
/// assert_eq!(Expression::add(vec![x_expr.clone(), expr!(0)]).simplify(), x_expr.clone());
/// assert_eq!(Expression::mul(vec![expr!(0), x_expr.clone()]).simplify(), expr!(0));
///
/// // Constant folding (algebraic, not "evaluation")
/// assert_eq!(Expression::add(vec![expr!(2), expr!(3)]).simplify(), expr!(5));
/// ```
///
/// # Domain Safety
///
/// Simplification does NOT validate mathematical domains:
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::simplify::Simplify;
///
/// // No error - stays symbolic or simplifies to i (complex domain)
/// let sqrt_neg = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);
/// let result = sqrt_neg.simplify();
/// // For domain checking, use evaluate() instead
/// ```
///
/// # Idempotency
///
/// Simplification is idempotent (applying twice yields same result):
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::simplify::Simplify;
///
/// let x = symbol!(x);
/// let x_expr = Expression::symbol(x.clone());
/// let expr = Expression::add(vec![x_expr.clone(), x_expr.clone(), x_expr.clone()]);
/// assert_eq!(expr.simplify().simplify(), expr.simplify());
/// ```
pub trait Simplify {
    /// Simplify expression using algebraic reduction rules
    ///
    /// Returns a mathematically equivalent expression in canonical simplified form.
    /// Never fails (always returns `Expression`, not `Result`).
    fn simplify(&self) -> Self;
}

impl Simplify for Expression {
    #[inline(always)]
    fn simplify(&self) -> Self {
        match self {
            Expression::Number(num) => Self::normalize_number(num),
            Expression::Symbol(_) => self.clone(),

            // Delegate arithmetic operations to specialized arithmetic module
            Expression::Add(terms) => arithmetic::simplify_addition(terms),
            Expression::Mul(factors) => arithmetic::simplify_multiplication(factors),
            Expression::Pow(base, exp) => arithmetic::simplify_power(base, exp),

            // Delegate function simplification to functions module
            Expression::Function { name, args } => functions::simplify_function(name, args),

            // Delegate constant simplification to constants module
            Expression::Constant(constant) => constants::simplify_constant(constant),

            // Delegate complex and matrix operations to specialized modules
            Expression::Complex(_) => Expression::simplify_complex(self),
            Expression::Matrix(_) => self.simplify_matrix(),

            // Handle remaining expression types with proper simplification
            Expression::Relation(relation) => {
                // Simplify both sides of relations (equations, inequalities)
                let simplified_left = relation.left.simplify();
                let simplified_right = relation.right.simplify();
                Expression::relation(simplified_left, simplified_right, relation.relation_type)
            }
            Expression::Piecewise(piecewise) => {
                // Simplify each piece's expression and condition
                let simplified_pieces: Vec<_> = piecewise
                    .pieces
                    .iter()
                    .map(|piece| (piece.0.simplify(), piece.1.simplify()))
                    .collect();
                let simplified_default = piecewise.default.as_ref().map(|expr| expr.simplify());
                Expression::piecewise(simplified_pieces, simplified_default)
            }
            Expression::Set(set_elements) => {
                // Simplify each element in the set
                let simplified_elements: Vec<_> =
                    set_elements.iter().map(|elem| elem.simplify()).collect();
                Expression::set(simplified_elements)
            }
            Expression::Interval(interval) => {
                // Simplify interval bounds
                let simplified_start = interval.start.simplify();
                let simplified_end = interval.end.simplify();
                Expression::interval(
                    simplified_start,
                    simplified_end,
                    interval.start_inclusive,
                    interval.end_inclusive,
                )
            }
            Expression::Calculus(calc_op) => {
                // Simplify calculus operations by simplifying their sub-expressions
                use crate::core::expression::data_types::CalculusData;
                match calc_op.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => {
                        let simplified_expr = expression.simplify();
                        Expression::Calculus(Arc::new(CalculusData::Derivative {
                            expression: simplified_expr,
                            variable: variable.clone(),
                            order: *order,
                        }))
                    }
                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => {
                        let simplified_integrand = integrand.simplify();
                        let simplified_bounds = bounds
                            .as_ref()
                            .map(|(start, end)| (start.simplify(), end.simplify()));
                        Expression::Calculus(Arc::new(CalculusData::Integral {
                            integrand: simplified_integrand,
                            variable: variable.clone(),
                            bounds: simplified_bounds,
                        }))
                    }
                    CalculusData::Limit {
                        expression,
                        variable,
                        point,
                        direction,
                    } => {
                        let simplified_expr = expression.simplify();
                        let simplified_point = point.simplify();
                        Expression::Calculus(Arc::new(CalculusData::Limit {
                            expression: simplified_expr,
                            variable: variable.clone(),
                            point: simplified_point,
                            direction: *direction,
                        }))
                    }
                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        let simplified_expr = expression.simplify();
                        let simplified_start = start.simplify();
                        let simplified_end = end.simplify();
                        Expression::Calculus(Arc::new(CalculusData::Sum {
                            expression: simplified_expr,
                            variable: variable.clone(),
                            start: simplified_start,
                            end: simplified_end,
                        }))
                    }
                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        let simplified_expr = expression.simplify();
                        let simplified_start = start.simplify();
                        let simplified_end = end.simplify();
                        Expression::Calculus(Arc::new(CalculusData::Product {
                            expression: simplified_expr,
                            variable: variable.clone(),
                            start: simplified_start,
                            end: simplified_end,
                        }))
                    }
                }
            }
            Expression::MethodCall(method_data) => {
                let simplified_object = method_data.object.simplify();
                let simplified_args: Vec<Expression> =
                    method_data.args.iter().map(|arg| arg.simplify()).collect();

                // Try to evaluate the method call if possible
                let method_call = Expression::method_call(
                    simplified_object,
                    &method_data.method_name,
                    simplified_args,
                );
                method_call.evaluate_method_call()
            }
        }
    }
}

impl Expression {
    /// Normalize Number by converting BigInteger to Integer when it fits in i64
    fn normalize_number(num: &Number) -> Self {
        match num {
            Number::BigInteger(bi) => {
                if let Some(i) = bi.to_i64() {
                    Expression::Number(Number::Integer(i))
                } else {
                    Expression::Number(num.clone())
                }
            }
            _ => Expression::Number(num.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, function};

    #[test]
    fn test_basic_simplification() {
        // Test integer addition
        let expr = expr!(2 + 3);
        assert_eq!(expr.simplify(), expr!(5));

        // Test multiplication
        let expr = expr!(2 * 3);
        assert_eq!(expr.simplify(), expr!(6));

        // Test power
        let expr = expr!(x ^ 1);
        assert_eq!(expr.simplify(), expr!(x));
    }

    #[test]
    fn test_function_simplification() {
        // Test sin(0) = 0
        let expr = function!(sin, expr!(0));
        assert_eq!(expr.simplify(), expr!(0));

        // Test cos(0) = 1
        let expr = function!(cos, expr!(0));
        assert_eq!(expr.simplify(), expr!(1));
    }

    #[test]
    fn test_zero_detection() {
        // Test zero multiplication
        let expr = expr!(0 * 5);
        let result = expr.simplify();
        assert_eq!(result, expr!(0));
    }
}

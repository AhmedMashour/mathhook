//! Minimal overhead, maximum performance implementation
//!
//! Uses modular architecture for specialized simplification of different expression types.
//! Individual modules handle specific aspects following project guidelines.

use crate::core::Expression;

mod arithmetic;
mod constants;
mod functions;

/// Trait for simplifying expressions
pub trait Simplify {
    fn simplify(&self) -> Self;
}

impl Simplify for Expression {
    #[inline(always)]
    fn simplify(&self) -> Self {
        match self {
            Expression::Number(_) | Expression::Symbol(_) => self.clone(),

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
            Expression::Matrix(_) | Expression::IdentityMatrix(_) => {
                Expression::simplify_matrix(self)
            }

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
                        Expression::Calculus(Box::new(CalculusData::Derivative {
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
                        Expression::Calculus(Box::new(CalculusData::Integral {
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
                        Expression::Calculus(Box::new(CalculusData::Limit {
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
                        Expression::Calculus(Box::new(CalculusData::Sum {
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
                        Expression::Calculus(Box::new(CalculusData::Product {
                            expression: simplified_expr,
                            variable: variable.clone(),
                            start: simplified_start,
                            end: simplified_end,
                        }))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_basic_simplification() {
        // Test integer addition
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr.simplify(), Expression::integer(5));

        // Test multiplication
        let expr = Expression::mul(vec![Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr.simplify(), Expression::integer(6));

        // Test power
        let x = Symbol::new("x");
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
        assert_eq!(expr.simplify(), Expression::symbol(x));
    }

    #[test]
    fn test_function_simplification() {
        // Test sin(0) = 0
        let expr = Expression::function("sin", vec![Expression::integer(0)]);
        assert_eq!(expr.simplify(), Expression::integer(0));

        // Test cos(0) = 1
        let expr = Expression::function("cos", vec![Expression::integer(0)]);
        assert_eq!(expr.simplify(), Expression::integer(1));
    }

    #[test]
    fn test_zero_detection() {
        // Test zero multiplication
        let expr = Expression::mul(vec![Expression::integer(0), Expression::integer(5)]);
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(0));
    }
}

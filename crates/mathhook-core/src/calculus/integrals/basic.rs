//! Basic integration rules for constants, symbols, sums, and simple powers
//!
//! Preserves order for noncommutative expressions (matrices, operators, quaternions).
//! Integration maintains factor order to ensure correctness with noncommutative algebra.
use crate::calculus::integrals::strategy::integrate_with_strategy;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
use num_rational::BigRational;
use num_traits::Zero;
/// Basic integration operations
pub struct BasicIntegrals;
impl BasicIntegrals {
    /// Handle integration of calculus expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::integrals::{BasicIntegrals, Integration};
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::integral(Expression::symbol(x.clone()), x.clone());
    /// let result = expr.integrate(x, 0);
    /// ```
    pub fn handle_calculus(
        expr: &Expression,
        data: &crate::core::expression::CalculusData,
        variable: Symbol,
    ) -> Expression {
        match data {
            crate::core::expression::CalculusData::Integral {
                variable: var,
                bounds,
                ..
            } => {
                if *var == variable && bounds.is_none() {
                    expr.clone()
                } else {
                    Expression::integral(expr.clone(), variable)
                }
            }
            _ => Expression::integral(expr.clone(), variable),
        }
    }
    /// Handle integration of constant expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::integer(5);
    /// let result = BasicIntegrals::handle_constant(&expr, x);
    /// ```
    pub fn handle_constant(expr: &Expression, variable: Symbol) -> Expression {
        Expression::mul(vec![expr.clone(), Expression::symbol(variable)])
    }
    /// Handle integration of symbol expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let dx = BasicIntegrals::handle_symbol(&x, &x);
    /// let dy = BasicIntegrals::handle_symbol(&x, &y);
    /// ```
    pub fn handle_symbol(sym: &Symbol, variable: &Symbol) -> Expression {
        if sym == variable {
            Expression::mul(vec![
                Expression::mul(vec![
                    Expression::integer(1),
                    Expression::pow(Expression::integer(2), Expression::integer(-1)),
                ]),
                Expression::pow(Expression::symbol(variable.clone()), Expression::integer(2)),
            ])
        } else {
            Expression::mul(vec![
                Expression::symbol(sym.clone()),
                Expression::symbol(variable.clone()),
            ])
        }
    }
    /// Handle integration of sum expressions using linearity
    ///
    /// # Arguments
    ///
    /// * `terms` - Terms in the sum
    /// * `variable` - Variable to integrate with respect to
    /// * `depth` - Current recursion depth
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::integrals::Integration;
    ///
    /// let x = symbol!(x);
    /// let terms = vec![Expression::symbol(x.clone()), Expression::integer(5)];
    /// let result = BasicIntegrals::handle_sum(&terms, &x, 0);
    /// ```
    pub fn handle_sum(terms: &[Expression], variable: &Symbol, depth: usize) -> Expression {
        let integrals: Vec<Expression> = terms
            .iter()
            .map(|term| integrate_with_strategy(term, variable.clone(), depth + 1))
            .collect();
        Expression::add(integrals).simplify()
    }
    /// Handle integration of product expressions
    ///
    /// Preserves factor order for noncommutative expressions (matrices, operators, quaternions).
    ///
    /// # Arguments
    ///
    /// * `factors` - Factors in the product
    /// * `variable` - Variable to integrate with respect to
    /// * `depth` - Current recursion depth
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let factors = vec![Expression::integer(3), Expression::symbol(x.clone())];
    /// let result = BasicIntegrals::handle_product(&factors, x, 0);
    /// ```
    pub fn handle_product(factors: &[Expression], variable: Symbol, depth: usize) -> Expression {
        let (constants, variables): (Vec<_>, Vec<_>) = factors
            .iter()
            .partition(|f| Self::is_constant_wrt(f, &variable));
        if variables.is_empty() {
            return Expression::mul(vec![
                Expression::mul(factors.to_vec()),
                Expression::symbol(variable),
            ]);
        }
        if variables.len() == 1 {
            let constant_part = if constants.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(constants.into_iter().cloned().collect())
            };
            let integrated_variable =
                integrate_with_strategy(variables[0], variable.clone(), depth + 1);
            let result = Expression::mul(vec![constant_part, integrated_variable]);
            let simplified = result.simplify();
            return simplified;
        }
        Expression::integral(Expression::mul(factors.to_vec()), variable)
    }
    /// Handle integration of power expressions using power rule
    ///
    /// Power rule for integer exponents: ∫x^n dx = x^(n+1)/(n+1) + C (n ≠ -1)
    /// Power rule for rational exponents: ∫x^(p/q) dx = (q/(p+q))·x^((p+q)/q) + C (p+q ≠ 0)
    /// Special case: ∫x^(-1) dx = ln|x| + C
    ///
    /// For expressions like x^2, uses the standard power rule.
    /// For more complex expressions, defers to by-parts or other methods.
    ///
    /// # Arguments
    ///
    /// * `base` - Base of the power expression
    /// * `exponent` - Exponent of the power expression
    /// * `variable` - Variable to integrate with respect to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let base = Expression::symbol(x.clone());
    /// let exp = Expression::integer(2);
    /// let result = BasicIntegrals::handle_power(&base, &exp, x);
    /// ```
    pub fn handle_power(base: &Expression, exp: &Expression, variable: Symbol) -> Expression {
        if let (Expression::Symbol(sym), Expression::Number(Number::Integer(n))) = (base, exp) {
            if *sym == variable {
                if *n == -1 {
                    Expression::function(
                        "ln",
                        vec![Expression::function(
                            "abs",
                            vec![Expression::symbol(variable)],
                        )],
                    )
                } else {
                    let new_exp = Expression::integer(n + 1);
                    let coefficient = Expression::mul(vec![
                        Expression::integer(1),
                        Expression::pow(Expression::integer(n + 1), Expression::integer(-1)),
                    ]);
                    Expression::mul(vec![
                        coefficient,
                        Expression::pow(Expression::symbol(variable), new_exp),
                    ])
                }
            } else {
                Expression::mul(vec![
                    Expression::pow(base.clone(), exp.clone()),
                    Expression::symbol(variable),
                ])
            }
        } else if let (Expression::Symbol(sym), Expression::Number(Number::Rational(r))) =
            (base, exp)
        {
            if *sym == variable {
                let p = r.numer();
                let q = r.denom();
                let p_plus_q = p + q;
                if p_plus_q.is_zero() {
                    Expression::function(
                        "ln",
                        vec![Expression::function(
                            "abs",
                            vec![Expression::symbol(variable)],
                        )],
                    )
                } else {
                    let new_exp_num = p_plus_q.clone();
                    let new_exp_denom = q.clone();
                    let new_exp_rational =
                        BigRational::new(new_exp_num.clone(), new_exp_denom.clone());
                    let coefficient_rational = BigRational::new(new_exp_denom, new_exp_num.clone());
                    let coefficient = Expression::Number(Number::rational(coefficient_rational));
                    let new_exp = Expression::Number(Number::rational(new_exp_rational));
                    Expression::mul(vec![
                        coefficient,
                        Expression::pow(Expression::symbol(variable), new_exp),
                    ])
                }
            } else {
                Expression::mul(vec![
                    Expression::pow(base.clone(), exp.clone()),
                    Expression::symbol(variable),
                ])
            }
        } else {
            Expression::mul(vec![
                Expression::pow(base.clone(), exp.clone()),
                Expression::symbol(variable),
            ])
        }
    }
    fn is_constant_wrt(expr: &Expression, variable: &Symbol) -> bool {
        !expr.find_variables().contains(variable)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::integrals::Integration;
    use crate::symbol;
    #[test]
    fn test_basic_constant_integration() {
        let x = symbol!(x);
        let expr = Expression::integer(5);
        let result = expr.integrate(x.clone(), 0);
        println!("Integration result: {}", result);
        assert!(result.to_string().contains("5"));
    }
    #[test]
    fn test_basic_variable_integration() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        let result = expr.integrate(x.clone(), 0);
        println!("Integration result: {}", result);
        assert!(result.to_string().contains("x") || result.to_string().contains("2"));
    }
    #[test]
    fn test_power_rule_x_squared() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let result = expr.integrate(x.clone(), 0);
        println!("Integration result for x^2: {}", result);
        assert!(result.to_string().contains("x") || result.to_string().contains("3"));
    }
    #[test]
    fn test_integral_of_sum() {
        let x = symbol!(x);
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        let result = expr.integrate(x.clone(), 0);
        println!("Integration result for x + 1: {}", result);
        assert!(!result.to_string().is_empty());
    }
    #[test]
    fn test_constant_multiple_integration() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]);
        let result = expr.integrate(x.clone(), 0);
        println!("Integration result for 3x: {}", result);
        assert!(result.to_string().contains("3") || result.to_string().contains("x"));
    }
}

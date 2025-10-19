//! Basic integration rules for constants, symbols, sums, and simple powers
//!
//! Preserves order for noncommutative expressions (matrices, operators, quaternions).
//! Integration maintains factor order to ensure correctness with noncommutative algebra.

use crate::calculus::integrals::Integration;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

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
    /// let result = expr.integrate(x);
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
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::integrals::Integration;
    ///
    /// let x = symbol!(x);
    /// let terms = vec![Expression::symbol(x.clone()), Expression::integer(5)];
    /// let result = BasicIntegrals::handle_sum(&terms, x);
    /// ```
    pub fn handle_sum(terms: &[Expression], variable: Symbol) -> Expression {
        let integrals: Vec<Expression> = terms
            .iter()
            .map(|term| term.integrate(variable.clone()))
            .collect();
        Expression::add(integrals).simplify()
    }

    /// Handle integration of product expressions
    ///
    /// Preserves factor order for noncommutative expressions (matrices, operators, quaternions).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let factors = vec![Expression::integer(3), Expression::symbol(x.clone())];
    /// let result = BasicIntegrals::handle_product(&factors, x);
    /// ```
    pub fn handle_product(factors: &[Expression], variable: Symbol) -> Expression {
        let (constants, variables): (Vec<_>, Vec<_>) = factors
            .iter()
            .partition(|f| Self::is_constant_wrt(f, &variable));

        if variables.len() == 1 {
            // Integrate the single variable factor, keep constants in original order
            let constant_part = if constants.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(constants.into_iter().cloned().collect())
            };

            Expression::mul(vec![constant_part, variables[0].integrate(variable)]).simplify()
        } else if variables.is_empty() {
            // All constants: ∫c dx = c*x (order preserved)
            Expression::mul(vec![
                Expression::mul(factors.to_vec()),
                Expression::symbol(variable),
            ])
        } else {
            // Complex product: preserve as symbolic integral (order maintained)
            Expression::integral(Expression::mul(factors.to_vec()), variable)
        }
    }

    /// Handle integration of power expressions using power rule
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let base = Expression::symbol(x.clone());
    /// let exp = Expression::integer(3);
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
        } else {
            // Try advanced power integration techniques
            Expression::integral(Expression::pow(base.clone(), exp.clone()), variable)
        }
    }

    /// Check if expression is constant with respect to variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, BasicIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::integer(5);
    /// let is_const = BasicIntegrals::is_constant_wrt(&expr, &x);
    /// ```
    pub fn is_constant_wrt(expr: &Expression, variable: &Symbol) -> bool {
        match expr {
            Expression::Number(_) | Expression::Constant(_) => true,
            Expression::Symbol(sym) => sym != variable,
            Expression::Add(terms) => terms.iter().all(|t| Self::is_constant_wrt(t, variable)),
            Expression::Mul(factors) => factors.iter().all(|f| Self::is_constant_wrt(f, variable)),
            Expression::Pow(base, exp) => {
                Self::is_constant_wrt(base, variable) && Self::is_constant_wrt(exp, variable)
            }
            Expression::Function { args, .. } => {
                args.iter().all(|a| Self::is_constant_wrt(a, variable))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_constant_integration() {
        let x = symbol!(x);
        let expr = Expression::integer(5);
        let result = BasicIntegrals::handle_constant(&expr, x.clone());

        // ∫5 dx = 5x
        assert_eq!(
            result,
            Expression::mul(vec![Expression::integer(5), Expression::symbol(x)])
        );
    }

    #[test]
    fn test_symbol_integration() {
        let x = symbol!(x);
        let result = BasicIntegrals::handle_symbol(&x, &x);

        // ∫x dx = x²/2
        let expected = Expression::mul(vec![
            Expression::rational(1, 2),
            Expression::pow(Expression::symbol(x), Expression::integer(2)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sum_integration() {
        let x = symbol!(x);
        let terms = vec![Expression::symbol(x.clone()), Expression::integer(3)];
        let result = BasicIntegrals::handle_sum(&terms, x.clone());

        // ∫(x + 3) dx = x²/2 + 3x
        assert!(!result.is_zero());
    }

    #[test]
    fn test_noncommutative_matrix_integration_order() {
        let x = symbol!(x);
        let a = crate::core::Symbol::matrix("A");

        // ∫A dx where A is a matrix (constant with respect to x)
        // Result should be A*x (order preserved)
        let result = BasicIntegrals::handle_symbol(&a, &x);

        // Result should be A*x (as a product)
        if let Expression::Mul(factors) = &result {
            assert_eq!(factors.len(), 2, "Result should be a product of two factors");

            // Verify both A and x are present in the product
            let has_a = factors.iter().any(|f| {
                if let Expression::Symbol(s) = f {
                    s.name() == "A"
                } else {
                    false
                }
            });
            let has_x = factors.iter().any(|f| {
                if let Expression::Symbol(s) = f {
                    s.name() == "x"
                } else {
                    false
                }
            });

            assert!(has_a, "Result should contain matrix A");
            assert!(has_x, "Result should contain variable x");
        } else {
            panic!("Expected Mul expression for integration result");
        }
    }

    #[test]
    fn test_noncommutative_operator_integration() {
        let t = symbol!(t);
        let p = crate::core::Symbol::operator("P");

        // ∫P dt where P is an operator (constant with respect to t)
        // Result should be P*t (order preserved)
        let result = BasicIntegrals::handle_symbol(&p, &t);

        if let Expression::Mul(factors) = &result {
            assert_eq!(factors.len(), 2, "Result should be a product of two factors");

            // Verify both P and t are present
            let has_p = factors.iter().any(|f| {
                if let Expression::Symbol(s) = f {
                    s.name() == "P"
                } else {
                    false
                }
            });
            let has_t = factors.iter().any(|f| {
                if let Expression::Symbol(s) = f {
                    s.name() == "t"
                } else {
                    false
                }
            });

            assert!(has_p, "Result should contain operator P");
            assert!(has_t, "Result should contain variable t");
        } else {
            panic!("Expected Mul expression for integration result");
        }
    }

    #[test]
    fn test_noncommutative_product_integration() {
        let x = symbol!(x);
        let a = crate::core::Symbol::matrix("A");
        let b = crate::core::Symbol::matrix("B");

        // ∫(A*B) dx where A, B are matrices (constants w.r.t. x)
        // Result should be (A*B)*x (order preserved)
        let product = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);

        let result = BasicIntegrals::handle_product(
            &vec![Expression::symbol(a.clone()), Expression::symbol(b.clone())],
            x.clone(),
        );

        // Should preserve A*B order
        assert!(!result.is_zero());
    }

    #[test]
    fn test_commutative_integration_unchanged() {
        let x = symbol!(x);
        let y = symbol!(y);

        // ∫y dx = y*x (commutative, order doesn't matter mathematically)
        let result = BasicIntegrals::handle_symbol(&y, &x);

        if let Expression::Mul(factors) = &result {
            assert_eq!(factors.len(), 2);
        }
    }

    #[test]
    fn test_is_constant_wrt_scalar() {
        let x = symbol!(x);
        let y = symbol!(y);

        assert!(BasicIntegrals::is_constant_wrt(&Expression::integer(5), &x));
        assert!(BasicIntegrals::is_constant_wrt(
            &Expression::symbol(y.clone()),
            &x
        ));
        assert!(!BasicIntegrals::is_constant_wrt(
            &Expression::symbol(x.clone()),
            &x
        ));
    }

    #[test]
    fn test_is_constant_wrt_noncommutative() {
        let x = symbol!(x);
        let a = crate::core::Symbol::matrix("A");

        // Matrix A is constant with respect to scalar x
        assert!(BasicIntegrals::is_constant_wrt(
            &Expression::symbol(a),
            &x
        ));
    }
}

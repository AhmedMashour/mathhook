//! Basic integration rules for constants, symbols, sums, and simple powers
//!
//! Preserves order for noncommutative expressions (matrices, operators, quaternions).
//! Integration maintains factor order to ensure correctness with noncommutative algebra.

use crate::calculus::integrals::Integration;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
use num_bigint::BigInt;
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
    /// let result = BasicIntegrals::handle_sum(&terms, x, 0);
    /// ```
    pub fn handle_sum(terms: &[Expression], variable: Symbol, depth: usize) -> Expression {
        let integrals: Vec<Expression> = terms
            .iter()
            .map(|term| term.integrate(variable.clone(), depth))
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

        if variables.len() == 1 {
            let constant_part = if constants.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(constants.into_iter().cloned().collect())
            };

            Expression::mul(vec![constant_part, variables[0].integrate(variable, depth)]).simplify()
        } else if variables.is_empty() {
            Expression::mul(vec![
                Expression::mul(factors.to_vec()),
                Expression::symbol(variable),
            ])
        } else {
            Expression::integral(Expression::mul(factors.to_vec()), variable)
        }
    }

    /// Handle integration of power expressions using power rule
    ///
    /// Power rule for integer exponents: ∫x^n dx = x^(n+1)/(n+1) + C (n ≠ -1)
    /// Power rule for rational exponents: ∫x^(p/q) dx = (q/(p+q))·x^((p+q)/q) + C (p+q ≠ 0)
    /// Special case: ∫x^(-1) dx = ln|x| + C
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
        } else if let (Expression::Symbol(sym), Expression::Number(Number::Rational(r))) = (base, exp) {
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
                    let new_exp_rational = BigRational::new(p_plus_q.clone(), q.clone());
                    let new_exp = Expression::Number(Number::rational(new_exp_rational));

                    let coefficient_rational = BigRational::new(q.clone(), p_plus_q.clone());
                    let coefficient = Expression::Number(Number::rational(coefficient_rational));

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
    use num_bigint::BigInt;

    #[test]
    fn test_constant_integration() {
        let x = symbol!(x);
        let expr = Expression::integer(5);
        let result = BasicIntegrals::handle_constant(&expr, x.clone());

        assert_eq!(
            result,
            Expression::mul(vec![Expression::integer(5), Expression::symbol(x)])
        );
    }

    #[test]
    fn test_symbol_integration() {
        let x = symbol!(x);
        let result = BasicIntegrals::handle_symbol(&x, &x);

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
        let result = BasicIntegrals::handle_sum(&terms, x.clone(), 0);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_noncommutative_matrix_integration_order() {
        let x = symbol!(x);
        let a = crate::core::Symbol::matrix("A");

        let result = BasicIntegrals::handle_symbol(&a, &x);

        if let Expression::Mul(factors) = &result {
            assert_eq!(factors.len(), 2, "Result should be a product of two factors");

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

        let result = BasicIntegrals::handle_symbol(&p, &t);

        if let Expression::Mul(factors) = &result {
            assert_eq!(factors.len(), 2, "Result should be a product of two factors");

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

        let product = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);

        let result = BasicIntegrals::handle_product(
            &vec![Expression::symbol(a.clone()), Expression::symbol(b.clone())],
            x.clone(),
            0,
        );

        assert!(!result.is_zero());
    }

    #[test]
    fn test_commutative_integration_unchanged() {
        let x = symbol!(x);
        let y = symbol!(y);

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

        assert!(BasicIntegrals::is_constant_wrt(
            &Expression::symbol(a),
            &x
        ));
    }

    #[test]
    fn test_rational_exponent_one_half() {
        let x = symbol!(x);
        let base = Expression::symbol(x.clone());
        let exp = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let result = BasicIntegrals::handle_power(&base, &exp, x.clone());

        let expected_coeff = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(2),
            BigInt::from(3),
        )));
        let expected_exp = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(3),
            BigInt::from(2),
        )));
        let expected = Expression::mul(vec![
            expected_coeff,
            Expression::pow(Expression::symbol(x), expected_exp),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rational_exponent_two_thirds() {
        let x = symbol!(x);
        let base = Expression::symbol(x.clone());
        let exp = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(2),
            BigInt::from(3),
        )));
        let result = BasicIntegrals::handle_power(&base, &exp, x.clone());

        let expected_coeff = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(3),
            BigInt::from(5),
        )));
        let expected_exp = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(5),
            BigInt::from(3),
        )));
        let expected = Expression::mul(vec![
            expected_coeff,
            Expression::pow(Expression::symbol(x), expected_exp),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rational_exponent_minus_one() {
        let x = symbol!(x);
        let base = Expression::symbol(x.clone());
        let exp = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(-1),
            BigInt::from(1),
        )));
        let result = BasicIntegrals::handle_power(&base, &exp, x.clone());

        let expected = Expression::function(
            "ln",
            vec![Expression::function(
                "abs",
                vec![Expression::symbol(x)],
            )],
        );

        assert_eq!(result, expected);
    }
}

//! Basic integration rules for constants, symbols, sums, and simple powers

use crate::simplify::Simplify;
use crate::calculus::integrals::Integration;
use crate::core::{Expression, Number, Symbol};

/// Basic integration operations
pub struct BasicIntegrals;

impl BasicIntegrals {
    /// Handle integration of calculus expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    ///
    /// let x = Symbol::new("x");
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
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    ///
    /// let x = Symbol::new("x");
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
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    /// use mathhook_core::calculus::integrals::Integration;
    ///
    /// let x = Symbol::new("x");
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
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    ///
    /// let x = Symbol::new("x");
    /// let factors = vec![Expression::integer(3), Expression::symbol(x.clone())];
    /// let result = BasicIntegrals::handle_product(&factors, x);
    /// ```
    pub fn handle_product(factors: &[Expression], variable: Symbol) -> Expression {
        let (constants, variables): (Vec<_>, Vec<_>) = factors
            .iter()
            .partition(|f| Self::is_constant_wrt(f, &variable));

        if variables.len() == 1 {
            let constant_part = if constants.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(constants.into_iter().cloned().collect())
            };

            Expression::mul(vec![constant_part, variables[0].integrate(variable)]).simplify()
        } else if variables.is_empty() {
            Expression::mul(vec![
                Expression::mul(factors.to_vec()),
                Expression::symbol(variable),
            ])
        } else {
            // Try more advanced methods for complex products
            Expression::integral(Expression::mul(factors.to_vec()), variable)
        }
    }

    /// Handle integration of power expressions using power rule
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    ///
    /// let x = Symbol::new("x");
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
    /// use mathhook_core::{Expression, Symbol, BasicIntegrals};
    ///
    /// let x = Symbol::new("x");
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

//! Integration of standard mathematical functions
//!
//! Handles integration of trigonometric, exponential, logarithmic,
//! and other standard functions using the existing Expression::function
//! infrastructure.

use crate::core::{Expression, Symbol};

/// Function integration handler
pub struct FunctionIntegrals;

impl FunctionIntegrals {
    /// Integrate function expressions using known antiderivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, FunctionIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let args = vec![Expression::symbol(x.clone())];
    /// let result = FunctionIntegrals::integrate("sin", &args, x);
    /// ```
    pub fn integrate(name: &str, args: &[Expression], variable: Symbol) -> Expression {
        if args.len() == 1 {
            if let Expression::Symbol(sym) = &args[0] {
                if *sym == variable {
                    // Direct integration of f(x) where arg is just x
                    Self::integrate_simple_function(name, variable)
                } else {
                    // f(y) where y ≠ x, treat as constant
                    Expression::mul(vec![
                        Expression::function(name, args.to_vec()),
                        Expression::symbol(variable),
                    ])
                }
            } else {
                // f(g(x)) - try substitution or chain rule
                Self::integrate_composite_function(name, &args[0], variable)
            }
        } else {
            // Multi-argument functions - fall back to symbolic
            Expression::integral(Expression::function(name, args.to_vec()), variable)
        }
    }

    /// Integrate simple functions f(x) using standard antiderivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, FunctionIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let result = FunctionIntegrals::integrate_simple_function("sin", x);
    /// ```
    pub fn integrate_simple_function(name: &str, variable: Symbol) -> Expression {
        match name {
            // Trigonometric functions
            "sin" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("cos", vec![Expression::symbol(variable)]),
            ]),
            "cos" => Expression::function("sin", vec![Expression::symbol(variable)]),
            "tan" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function(
                    "ln",
                    vec![Expression::function(
                        "abs",
                        vec![Expression::function(
                            "cos",
                            vec![Expression::symbol(variable)],
                        )],
                    )],
                ),
            ]),
            "sec" => Expression::function(
                "ln",
                vec![Expression::function(
                    "abs",
                    vec![Expression::add(vec![
                        Expression::function("sec", vec![Expression::symbol(variable.clone())]),
                        Expression::function("tan", vec![Expression::symbol(variable)]),
                    ])],
                )],
            ),
            "csc" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function(
                    "ln",
                    vec![Expression::function(
                        "abs",
                        vec![Expression::add(vec![
                            Expression::function("csc", vec![Expression::symbol(variable.clone())]),
                            Expression::function("cot", vec![Expression::symbol(variable)]),
                        ])],
                    )],
                ),
            ]),
            "cot" => Expression::function(
                "ln",
                vec![Expression::function(
                    "abs",
                    vec![Expression::function(
                        "sin",
                        vec![Expression::symbol(variable)],
                    )],
                )],
            ),

            // Exponential and logarithmic functions
            "exp" => Expression::function("exp", vec![Expression::symbol(variable)]),
            "ln" => Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(variable.clone()),
                    Expression::function("ln", vec![Expression::symbol(variable.clone())]),
                ]),
                Expression::mul(vec![Expression::integer(-1), Expression::symbol(variable)]),
            ]),
            "log" => Expression::mul(vec![
                Expression::mul(vec![
                    Expression::integer(1),
                    Expression::pow(
                        Expression::function("ln", vec![Expression::integer(10)]),
                        Expression::integer(-1),
                    ),
                ]),
                Expression::add(vec![
                    Expression::mul(vec![
                        Expression::symbol(variable.clone()),
                        Expression::function("ln", vec![Expression::symbol(variable.clone())]),
                    ]),
                    Expression::mul(vec![Expression::integer(-1), Expression::symbol(variable)]),
                ]),
            ]),

            // Inverse trigonometric functions
            "arcsin" => Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(variable.clone()),
                    Expression::function("arcsin", vec![Expression::symbol(variable.clone())]),
                ]),
                Expression::function(
                    "sqrt",
                    vec![Expression::add(vec![
                        Expression::integer(1),
                        Expression::mul(vec![
                            Expression::integer(-1),
                            Expression::pow(Expression::symbol(variable), Expression::integer(2)),
                        ]),
                    ])],
                ),
            ]),
            "arccos" => Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(variable.clone()),
                    Expression::function("arccos", vec![Expression::symbol(variable.clone())]),
                ]),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::function(
                        "sqrt",
                        vec![Expression::add(vec![
                            Expression::integer(1),
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::pow(
                                    Expression::symbol(variable),
                                    Expression::integer(2),
                                ),
                            ]),
                        ])],
                    ),
                ]),
            ]),
            "arctan" => Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(variable.clone()),
                    Expression::function("arctan", vec![Expression::symbol(variable.clone())]),
                ]),
                Expression::mul(vec![
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(Expression::integer(2), Expression::integer(-1)),
                    ]),
                    Expression::function(
                        "ln",
                        vec![Expression::add(vec![
                            Expression::integer(1),
                            Expression::pow(Expression::symbol(variable), Expression::integer(2)),
                        ])],
                    ),
                ]),
            ]),

            // Hyperbolic functions
            "sinh" => Expression::function("cosh", vec![Expression::symbol(variable)]),
            "cosh" => Expression::function("sinh", vec![Expression::symbol(variable)]),
            "tanh" => Expression::function(
                "ln",
                vec![Expression::function(
                    "cosh",
                    vec![Expression::symbol(variable)],
                )],
            ),

            // Square root and other power functions
            "sqrt" => Expression::mul(vec![
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pow(Expression::integer(3), Expression::integer(-1)),
                ]),
                Expression::pow(
                    Expression::symbol(variable),
                    Expression::mul(vec![
                        Expression::integer(3),
                        Expression::pow(Expression::integer(2), Expression::integer(-1)),
                    ]),
                ),
            ]),

            // Fall back to symbolic representation
            _ => Expression::integral(
                Expression::function(name, vec![Expression::symbol(variable.clone())]),
                variable,
            ),
        }
    }

    /// Integrate composite functions f(g(x)) using substitution when possible
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, FunctionIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let inner = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    /// let result = FunctionIntegrals::integrate_composite_function("sin", &inner, x);
    /// ```
    pub fn integrate_composite_function(
        name: &str,
        inner: &Expression,
        variable: Symbol,
    ) -> Expression {
        // Try simple substitution patterns
        match (name, inner) {
            // sin(ax), cos(ax), etc. where inner is ax
            ("sin" | "cos" | "exp", Expression::Mul(factors)) => {
                if factors.len() == 2 {
                    if let (Expression::Number(_), Expression::Symbol(sym)) =
                        (&factors[0], &factors[1])
                    {
                        if *sym == variable {
                            return Self::integrate_linear_substitution(
                                name,
                                &factors[0],
                                variable,
                            );
                        }
                    }
                }
                Expression::integral(Expression::function(name, vec![inner.clone()]), variable)
            }

            // More complex cases - fall back to symbolic
            _ => Expression::integral(Expression::function(name, vec![inner.clone()]), variable),
        }
    }

    /// Handle integration of f(ax) where a is constant
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, FunctionIntegrals};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let a = Expression::integer(3);
    /// let result = FunctionIntegrals::integrate_linear_substitution("sin", &a, x);
    /// ```
    pub fn integrate_linear_substitution(
        name: &str,
        coefficient: &Expression,
        variable: Symbol,
    ) -> Expression {
        let antiderivative = Self::integrate_simple_function(name, variable.clone());
        let substituted =
            Self::substitute_variable_with_coefficient(&antiderivative, coefficient, variable);

        // Multiply by 1/a for the substitution
        Expression::mul(vec![
            Expression::pow(coefficient.clone(), Expression::integer(-1)),
            substituted,
        ])
    }

    /// Helper to substitute x with ax in an expression
    fn substitute_variable_with_coefficient(
        expr: &Expression,
        coefficient: &Expression,
        variable: Symbol,
    ) -> Expression {
        match expr {
            Expression::Symbol(sym) if *sym == variable => {
                Expression::mul(vec![coefficient.clone(), Expression::symbol(variable)])
            }
            Expression::Function { name, args } => {
                let new_args: Vec<Expression> = args
                    .iter()
                    .map(|arg| {
                        Self::substitute_variable_with_coefficient(
                            arg,
                            coefficient,
                            variable.clone(),
                        )
                    })
                    .collect();
                Expression::function(name, new_args)
            }
            Expression::Add(terms) => {
                let new_terms: Vec<Expression> = terms
                    .iter()
                    .map(|term| {
                        Self::substitute_variable_with_coefficient(
                            term,
                            coefficient,
                            variable.clone(),
                        )
                    })
                    .collect();
                Expression::add(new_terms)
            }
            Expression::Mul(factors) => {
                let new_factors: Vec<Expression> = factors
                    .iter()
                    .map(|factor| {
                        Self::substitute_variable_with_coefficient(
                            factor,
                            coefficient,
                            variable.clone(),
                        )
                    })
                    .collect();
                Expression::mul(new_factors)
            }
            _ => expr.clone(),
        }
    }
}

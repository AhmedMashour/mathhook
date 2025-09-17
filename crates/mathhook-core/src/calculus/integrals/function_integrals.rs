//! Integration of standard mathematical functions
//!
//! Handles integration of trigonometric, exponential, logarithmic,
//! and other standard functions using the existing Expression::function
//! infrastructure.

use crate::core::{Expression, Symbol};
use crate::functions::intelligence::get_universal_registry;
use crate::functions::properties::{AntiderivativeRule, AntiderivativeRuleType};

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
    /// let result = FunctionIntegrals::integrate("sin", &args, x.clone());
    ///
    /// let expected = Expression::mul(vec![
    ///     Expression::integer(-1),
    ///     Expression::function("cos", vec![Expression::symbol(x)]),
    /// ]);
    /// assert_eq!(result, expected);
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
    /// let result = FunctionIntegrals::integrate_simple_function("sin", x.clone());
    ///
    /// let expected = Expression::mul(vec![
    ///     Expression::integer(-1),
    ///     Expression::function("cos", vec![Expression::symbol(x)]),
    /// ]);
    /// assert_eq!(result, expected);
    /// ```
    pub fn integrate_simple_function(name: &str, variable: Symbol) -> Expression {
        let registry = get_universal_registry();

        if let Some(props) = registry.get_properties(name) {
            if let Some(rule) = props.get_antiderivative_rule() {
                return Self::apply_antiderivative_rule(rule, name, variable);
            }
        }

        Expression::integral(
            Expression::function(name, vec![Expression::symbol(variable.clone())]),
            variable,
        )
    }

    /// Apply antiderivative rule from registry to compute integral
    ///
    /// Takes a rule from the function intelligence registry and constructs
    /// the corresponding antiderivative expression.
    ///
    /// # Arguments
    ///
    /// * `rule` - The antiderivative rule from function intelligence registry
    /// * `function_name` - Original function name (for error messages and fallback)
    /// * `variable` - Integration variable
    ///
    /// # Returns
    ///
    /// The antiderivative expression. For unknown rule types, returns symbolic integral.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::symbol;
    /// ```
    fn apply_antiderivative_rule(
        rule: &AntiderivativeRule,
        function_name: &str,
        variable: Symbol,
    ) -> Expression {
        match &rule.rule_type {
            AntiderivativeRuleType::Simple {
                antiderivative_fn,
                coefficient,
            } => {
                // ∫f(x)dx = c * F(x)
                Expression::mul(vec![
                    coefficient.clone(),
                    Expression::function(antiderivative_fn, vec![Expression::symbol(variable)]),
                ])
            }

            AntiderivativeRuleType::Custom { builder } => {
                // Builder constructs the expression directly
                builder(variable)
            }

            AntiderivativeRuleType::LinearSubstitution { .. } => {
                // Future implementation
                Expression::integral(
                    Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                    variable,
                )
            }

            AntiderivativeRuleType::TrigSubstitution { .. } => {
                // Future implementation
                Expression::integral(
                    Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                    variable,
                )
            }

            AntiderivativeRuleType::PartialFractions { .. } => {
                // Future implementation
                Expression::integral(
                    Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                    variable,
                )
            }
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
    /// let inner = Expression::mul(vec![
    ///     Expression::integer(2),
    ///     Expression::symbol(x.clone()),
    /// ]);
    /// let result = FunctionIntegrals::integrate_composite_function("sin", &inner, x.clone());
    ///
    /// let expected = Expression::mul(vec![
    ///     Expression::pow(Expression::integer(2), Expression::integer(-1)),
    ///     Expression::mul(vec![
    ///         Expression::integer(-1),
    ///         Expression::function("cos", vec![
    ///             Expression::mul(vec![
    ///                 Expression::integer(2),
    ///                 Expression::symbol(x),
    ///             ])
    ///         ]),
    ///     ]),
    /// ]);
    /// assert_eq!(result, expected);
    /// ```
    pub fn integrate_composite_function(
        name: &str,
        inner: &Expression,
        variable: Symbol,
    ) -> Expression {
        let registry = get_universal_registry();

        if let Some(props) = registry.get_properties(name) {
            if let Some(_rule) = props.get_antiderivative_rule() {
                if let Expression::Mul(factors) = inner {
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
                }
            }
        }

        Expression::integral(Expression::function(name, vec![inner.clone()]), variable)
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
    /// let result = FunctionIntegrals::integrate_linear_substitution("sin", &a, x.clone());
    ///
    /// let expected = Expression::mul(vec![
    ///     Expression::pow(Expression::integer(3), Expression::integer(-1)),
    ///     Expression::mul(vec![
    ///         Expression::integer(-1),
    ///         Expression::function("cos", vec![
    ///             Expression::mul(vec![
    ///                 Expression::integer(3),
    ///                 Expression::symbol(x),
    ///             ])
    ///         ]),
    ///     ]),
    /// ]);
    /// assert_eq!(result, expected);
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

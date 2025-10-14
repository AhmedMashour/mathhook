//! Chain rule implementation for function derivatives

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
use crate::MathConstant;

/// Chain rule implementation for function derivatives
pub struct ChainRule;

impl ChainRule {
    /// Handle derivative of function expressions using chain rule
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let result = expr.derivative(x.clone());
    /// ```
    pub fn handle_function(name: &str, args: &[Expression], variable: Symbol) -> Expression {
        if args.len() != 1 {
            return Expression::derivative(Expression::function(name, args.to_vec()), variable, 1);
        }

        Self::apply(name, &args[0], variable)
    }

    /// Apply chain rule for function derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::ChainRule;
    ///
    /// let x = symbol!(x);
    /// let arg = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    /// let result = ChainRule::apply("sin", &arg, x.clone());
    /// ```
    pub fn apply(function_name: &str, arg: &Expression, variable: Symbol) -> Expression {
        let arg_derivative = arg.derivative(variable.clone());
        let function_derivative = FunctionDerivatives::get(function_name, arg, variable);

        Expression::mul(vec![function_derivative, arg_derivative]).simplify()
    }
}

/// Function derivative lookup
pub struct FunctionDerivatives;

impl FunctionDerivatives {
    /// Get the derivative of a specific function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::FunctionDerivatives;
    ///
    /// let x = symbol!(x);
    /// let arg = Expression::symbol(x.clone());
    /// let result = FunctionDerivatives::get("sin", &arg, x.clone());
    /// ```
    pub fn get(name: &str, arg: &Expression, variable: Symbol) -> Expression {
        match name {
            // Basic trigonometric functions
            "sin" => Expression::function("cos", vec![arg.clone()]),
            "cos" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("sin", vec![arg.clone()]),
            ]),
            "tan" => Expression::pow(
                Expression::function("sec", vec![arg.clone()]),
                Expression::integer(2),
            ),
            "sec" => Expression::mul(vec![
                Expression::function("sec", vec![arg.clone()]),
                Expression::function("tan", vec![arg.clone()]),
            ]),
            "csc" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("csc", vec![arg.clone()]),
                Expression::function("cot", vec![arg.clone()]),
            ]),
            "cot" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::function("csc", vec![arg.clone()]),
                    Expression::integer(2),
                ),
            ]),

            // Inverse trigonometric functions
            "arcsin" => Expression::pow(
                Expression::add(vec![
                    Expression::integer(1),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(arg.clone(), Expression::integer(2)),
                    ]),
                ]),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::pow(Expression::integer(2), Expression::integer(-1)),
                ]),
            ),
            "arccos" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::add(vec![
                        Expression::integer(1),
                        Expression::mul(vec![
                            Expression::integer(-1),
                            Expression::pow(arg.clone(), Expression::integer(2)),
                        ]),
                    ]),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(Expression::integer(2), Expression::integer(-1)),
                    ]),
                ),
            ]),
            "arctan" => Expression::pow(
                Expression::add(vec![
                    Expression::integer(1),
                    Expression::pow(arg.clone(), Expression::integer(2)),
                ]),
                Expression::integer(-1),
            ),
            "arcsec" => Expression::mul(vec![Expression::pow(
                Expression::mul(vec![
                    Expression::function("abs", vec![arg.clone()]),
                    Expression::function(
                        "sqrt",
                        vec![Expression::add(vec![
                            Expression::pow(arg.clone(), Expression::integer(2)),
                            Expression::integer(-1),
                        ])],
                    ),
                ]),
                Expression::integer(-1),
            )]),
            "arccsc" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::mul(vec![
                        Expression::function("abs", vec![arg.clone()]),
                        Expression::function(
                            "sqrt",
                            vec![Expression::add(vec![
                                Expression::pow(arg.clone(), Expression::integer(2)),
                                Expression::integer(-1),
                            ])],
                        ),
                    ]),
                    Expression::integer(-1),
                ),
            ]),
            "arccot" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::add(vec![
                        Expression::integer(1),
                        Expression::pow(arg.clone(), Expression::integer(2)),
                    ]),
                    Expression::integer(-1),
                ),
            ]),

            // Hyperbolic functions
            "sinh" => Expression::function("cosh", vec![arg.clone()]),
            "cosh" => Expression::function("sinh", vec![arg.clone()]),
            "tanh" => Expression::add(vec![
                Expression::integer(1),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::pow(
                        Expression::function("tanh", vec![arg.clone()]),
                        Expression::integer(2),
                    ),
                ]),
            ]),
            "sech" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("sech", vec![arg.clone()]),
                Expression::function("tanh", vec![arg.clone()]),
            ]),
            "csch" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("csch", vec![arg.clone()]),
                Expression::function("coth", vec![arg.clone()]),
            ]),
            "coth" => Expression::add(vec![
                Expression::integer(1),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::pow(
                        Expression::function("csch", vec![arg.clone()]),
                        Expression::integer(2),
                    ),
                ]),
            ]),

            // Inverse hyperbolic functions
            "asinh" => Expression::pow(
                Expression::function(
                    "sqrt",
                    vec![Expression::add(vec![
                        Expression::pow(arg.clone(), Expression::integer(2)),
                        Expression::integer(1),
                    ])],
                ),
                Expression::integer(-1),
            ),
            "acosh" => Expression::pow(
                Expression::function(
                    "sqrt",
                    vec![Expression::add(vec![
                        Expression::pow(arg.clone(), Expression::integer(2)),
                        Expression::integer(-1),
                    ])],
                ),
                Expression::integer(-1),
            ),
            "atanh" => Expression::pow(
                Expression::add(vec![
                    Expression::integer(1),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(arg.clone(), Expression::integer(2)),
                    ]),
                ]),
                Expression::integer(-1),
            ),

            // Exponential and logarithmic functions
            "exp" => Expression::function("exp", vec![arg.clone()]),
            "ln" => Expression::pow(arg.clone(), Expression::integer(-1)),
            "log" => Expression::mul(vec![
                Expression::pow(arg.clone(), Expression::integer(-1)),
                Expression::pow(
                    Expression::function("ln", vec![Expression::integer(10)]),
                    Expression::integer(-1),
                ),
            ]),
            "log2" => Expression::mul(vec![
                Expression::pow(arg.clone(), Expression::integer(-1)),
                Expression::pow(
                    Expression::function("ln", vec![Expression::integer(2)]),
                    Expression::integer(-1),
                ),
            ]),

            // Power and root functions
            "sqrt" => Expression::mul(vec![
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
                Expression::pow(
                    arg.clone(),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(Expression::integer(2), Expression::integer(-1)),
                    ]),
                ),
            ]),
            "cbrt" => Expression::mul(vec![
                Expression::pow(Expression::integer(3), Expression::integer(-1)),
                Expression::pow(
                    arg.clone(),
                    Expression::mul(vec![
                        Expression::integer(-2),
                        Expression::pow(Expression::integer(3), Expression::integer(-1)),
                    ]),
                ),
            ]),

            // Special functions (basic derivatives)
            "abs" => Expression::function("sign", vec![arg.clone()]),
            "sign" => Expression::integer(0), // Derivative is 0 except at discontinuities
            "floor" | "ceil" => Expression::integer(0), // Derivative is 0 except at integers

            // Error functions
            "erf" => Expression::mul(vec![
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pow(
                        Expression::function("sqrt", vec![Expression::constant(MathConstant::Pi)]),
                        Expression::integer(-1),
                    ),
                ]),
                Expression::function(
                    "exp",
                    vec![Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(arg.clone(), Expression::integer(2)),
                    ])],
                ),
            ]),
            "erfc" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pow(
                        Expression::function("sqrt", vec![Expression::constant(MathConstant::Pi)]),
                        Expression::integer(-1),
                    ),
                ]),
                Expression::function(
                    "exp",
                    vec![Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::pow(arg.clone(), Expression::integer(2)),
                    ])],
                ),
            ]),

            // Gamma function
            "gamma" => Expression::mul(vec![
                Expression::function("gamma", vec![arg.clone()]),
                Expression::function("digamma", vec![arg.clone()]),
            ]),
            "lgamma" => Expression::function("digamma", vec![arg.clone()]),

            // Fall back to symbolic representation
            _ => Expression::derivative(Expression::function(name, vec![arg.clone()]), variable, 1),
        }
    }
}

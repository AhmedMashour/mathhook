//! Chain rule implementation for function derivatives

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::functions::intelligence::get_universal_registry;
use crate::simplify::Simplify;

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
    /// Get the derivative of a specific function using UniversalFunctionRegistry
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
        let registry = get_universal_registry();

        if let Some(props) = registry.get_properties(name) {
            if let Some(deriv_expr) = props.get_derivative_expression(arg) {
                return deriv_expr;
            }
        }

        Expression::derivative(Expression::function(name, vec![arg.clone()]), variable, 1)
    }
}

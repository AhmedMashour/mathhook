//! Derivative computation and symbolic differentiation
//!
//! Provides symbolic differentiation capabilities including basic derivatives,
//! chain rule, product rule, quotient rule, and higher-order derivatives.
//! Utilizes the existing Expression::Calculus infrastructure.

mod advanced_differentiation;
mod basic;
mod chain_rule;
mod checker;
pub mod educational;
mod higher_order;
mod partial;
mod power_rule;
mod product_rule;

use crate::core::{Expression, Symbol};
pub use advanced_differentiation::{
    AdvancedDifferentiation, ImplicitCurveAnalysis, ImplicitDifferentiation,
    ParametricCurveAnalysis, ParametricDifferentiation, VectorValuedDifferentiation,
};
pub use basic::BasicDerivatives;
pub use chain_rule::{ChainRule, FunctionDerivatives};
pub use checker::DifferentiabilityChecker;
pub use educational::DerivativeWithSteps;
pub use higher_order::HigherOrderDerivatives;
pub use partial::{
    ConservativeFields, DirectionalDerivatives, FluidDynamicsOperations, GradientOperations,
    HessianOperations, JacobianDeterminant, JacobianOperations, MatrixUtils, PartialDerivatives,
    PartialUtils, VectorFieldOperations,
};
pub use power_rule::PowerRule;
pub use product_rule::{GeneralProductRule, ProductRule};

/// Trait for derivative operations
pub trait Derivative {
    /// Compute the derivative with respect to a variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = expr!(x ^ 2);
    /// let result = expr.derivative(x);
    /// ```
    fn derivative(&self, variable: Symbol) -> Expression;

    /// Compute higher-order derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = expr!(x ^ 4);
    /// let second_derivative = expr.nth_derivative(x, 2);
    /// ```
    fn nth_derivative(&self, variable: Symbol, order: u32) -> Expression;

    /// Check if expression is differentiable with respect to variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = expr!(sin(x));
    /// let is_diff = expr.is_differentiable(x);
    /// ```
    fn is_differentiable(&self, variable: Symbol) -> bool;
}

impl Derivative for Expression {
    fn derivative(&self, variable: Symbol) -> Expression {
        match self {
            Expression::Calculus(data) => BasicDerivatives::handle_calculus(self, data, variable),
            Expression::Number(_) | Expression::Constant(_) => Expression::integer(0),
            Expression::Symbol(sym) => BasicDerivatives::handle_symbol(sym, &variable),
            Expression::Add(terms) => BasicDerivatives::handle_sum(terms, variable),
            Expression::Mul(factors) => ProductRule::handle_product(factors, variable),
            Expression::Pow(base, exponent) => PowerRule::apply(base, exponent, variable),
            Expression::Function { name, args } => ChainRule::handle_function(name, args, variable),
            _ => Expression::derivative(self.clone(), variable, 1),
        }
    }

    fn nth_derivative(&self, variable: Symbol, order: u32) -> Expression {
        HigherOrderDerivatives::compute(self, variable, order)
    }

    fn is_differentiable(&self, variable: Symbol) -> bool {
        DifferentiabilityChecker::check(self, variable)
    }
}

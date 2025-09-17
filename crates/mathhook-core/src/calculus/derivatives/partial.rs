//! Partial derivative operations for multivariate calculus
//!
//! Provides comprehensive partial differentiation capabilities organized
//! into focused modules for better maintainability and performance.
mod gradient;
mod hessian;
mod jacobian;
mod utils;
mod vector_fields;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
pub use gradient::{DirectionalDerivatives, GradientOperations};
pub use hessian::HessianOperations;
pub use jacobian::{JacobianDeterminant, JacobianOperations};
pub use utils::{MatrixUtils, PartialUtils};
pub use vector_fields::{ConservativeFields, FluidDynamicsOperations, VectorFieldOperations};
/// Main partial derivatives interface
pub struct PartialDerivatives;
impl PartialDerivatives {
    /// Compute mixed partial derivative ∂ⁿf/∂x₁∂x₂...∂xₙ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::PartialDerivatives;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::mul(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(3))
    /// ]);
    /// let mixed = PartialDerivatives::mixed_partial(&expr, vec![x, y]);
    /// ```
    pub fn mixed_partial(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let mut result = expr.clone();
        for var in variables {
            result = result.derivative(var);
        }
        result.simplify()
    }
    /// Compute gradient vector - delegates to specialized module
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::PartialDerivatives;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let grad = PartialDerivatives::gradient(&expr, vec![x, y]);
    /// ```
    pub fn gradient(expr: &Expression, variables: Vec<Symbol>) -> Vec<Expression> {
        GradientOperations::compute(expr, variables)
    }
    /// Compute Hessian matrix - delegates to specialized module
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::PartialDerivatives;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let hessian = PartialDerivatives::hessian(&expr, vec![x, y]);
    /// ```
    pub fn hessian(expr: &Expression, variables: Vec<Symbol>) -> Vec<Vec<Expression>> {
        HessianOperations::compute(expr, &variables)
    }
    /// Compute directional derivative - delegates to specialized module
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::PartialDerivatives;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let direction = vec![Expression::integer(1), Expression::integer(1)];
    /// let dir_deriv = PartialDerivatives::directional_derivative(&expr, vec![x, y], direction);
    /// ```
    pub fn directional_derivative(
        expr: &Expression,
        variables: Vec<Symbol>,
        direction: Vec<Expression>,
    ) -> Expression {
        DirectionalDerivatives::compute(expr, variables, direction)
    }
    /// Compute Jacobian matrix - delegates to specialized module
    pub fn jacobian(functions: &[Expression], variables: &[Symbol]) -> Vec<Vec<Expression>> {
        JacobianOperations::compute(functions, variables)
    }
    /// Compute Jacobian determinant - delegates to specialized module
    pub fn jacobian_determinant(functions: &[Expression], variables: &[Symbol]) -> Expression {
        JacobianDeterminant::compute(functions, variables)
    }
    /// Compute divergence - delegates to specialized module
    pub fn divergence(vector_field: &[Expression], variables: Vec<Symbol>) -> Expression {
        VectorFieldOperations::divergence(vector_field, variables)
    }
    /// Compute curl - delegates to specialized module
    pub fn curl(vector_field: &[Expression], variables: Vec<Symbol>) -> Vec<Expression> {
        VectorFieldOperations::curl(vector_field, &variables)
    }
    /// Compute Laplacian - delegates to specialized module
    pub fn laplacian(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        VectorFieldOperations::laplacian(expr, variables)
    }
    /// Check if vector field is conservative - delegates to specialized module
    pub fn is_conservative(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        ConservativeFields::is_conservative(vector_field, variables)
    }
    /// Find potential function - delegates to specialized module
    pub fn find_potential(
        vector_field: &[Expression],
        variables: Vec<Symbol>,
    ) -> Option<Expression> {
        ConservativeFields::find_potential(vector_field, &variables)
    }
}

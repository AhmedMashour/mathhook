//! Advanced differentiation techniques
//!
//! Provides specialized differentiation methods for implicit functions,
//! parametric curves, and vector-valued functions with optimized performance
//! and memory usage.

mod implicit;
mod parametric;
mod vector_valued;

use crate::core::{Expression, Symbol};

pub use implicit::{ImplicitCurveAnalysis, ImplicitDifferentiation};
pub use parametric::{ParametricCurveAnalysis, ParametricDifferentiation};
pub use vector_valued::VectorValuedDifferentiation;

/// Advanced differentiation methods with performance optimization
pub struct AdvancedDifferentiation;

impl AdvancedDifferentiation {
    /// Compute implicit derivative dy/dx for F(x,y) = 0
    ///
    /// Uses the formula: dy/dx = -∂F/∂x / ∂F/∂y
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::AdvancedDifferentiation;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let equation = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let dy_dx = AdvancedDifferentiation::implicit(&equation, x, y);
    /// ```
    pub fn implicit(equation: &Expression, x_var: Symbol, y_var: Symbol) -> Expression {
        ImplicitDifferentiation::compute(equation, x_var, y_var)
    }

    /// Compute parametric derivative dy/dx for x = f(t), y = g(t)
    ///
    /// Uses formula: dy/dx = (dy/dt) / (dx/dt)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::AdvancedDifferentiation;
    ///
    /// let t = symbol!(t);
    /// let x_param = Expression::function("cos", vec![Expression::symbol(t.clone())]);
    /// let y_param = Expression::function("sin", vec![Expression::symbol(t.clone())]);
    /// let dy_dx = AdvancedDifferentiation::parametric(&x_param, &y_param, t);
    /// ```
    pub fn parametric(x_param: &Expression, y_param: &Expression, parameter: Symbol) -> Expression {
        ParametricDifferentiation::first_derivative(x_param, y_param, parameter)
    }

    /// Compute derivative of vector-valued function r'(t)
    ///
    /// Returns velocity vector for position vector r(t) = [x(t), y(t), z(t)]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::AdvancedDifferentiation;
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    ///     Expression::symbol(t.clone())
    /// ];
    /// let velocity = AdvancedDifferentiation::vector_valued(&components, t);
    /// ```
    pub fn vector_valued(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        VectorValuedDifferentiation::derivative(components, parameter)
    }

    /// Compute curvature for parametric curves
    ///
    /// Uses formula: κ = |x'y'' - y'x''| / (x'² + y'²)^(3/2)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::AdvancedDifferentiation;
    ///
    /// let t = symbol!(t);
    /// let x_param = Expression::function("cos", vec![Expression::symbol(t.clone())]);
    /// let y_param = Expression::function("sin", vec![Expression::symbol(t.clone())]);
    /// let curvature = AdvancedDifferentiation::parametric_curvature(&x_param, &y_param, t);
    /// ```
    pub fn parametric_curvature(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> Expression {
        ParametricDifferentiation::curvature(x_param, y_param, parameter)
    }

    /// Compute curvature for vector-valued functions (space curves)
    ///
    /// Uses formula: κ = |r' × r''| / |r'|³
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::AdvancedDifferentiation;
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::symbol(t.clone()),
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(3))
    /// ];
    /// let curvature = AdvancedDifferentiation::vector_curvature(&components, t);
    /// ```
    pub fn vector_curvature(components: &[Expression], parameter: Symbol) -> Expression {
        VectorValuedDifferentiation::curvature(components, parameter)
    }
}

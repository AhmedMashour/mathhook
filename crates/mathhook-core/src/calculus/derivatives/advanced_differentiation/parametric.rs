//! Parametric differentiation for parametrically defined curves
//!
//! Handles differentiation of curves defined parametrically as x = f(t), y = g(t)
//! using the chain rule: dy/dx = (dy/dt) / (dx/dt)

use crate::algebra::simplify::Simplify;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};

/// Parametric differentiation operations
pub struct ParametricDifferentiation;

impl ParametricDifferentiation {
    /// Compute dy/dx for parametric curve with cached derivatives
    ///
    /// Uses formula: dy/dx = (dy/dt) / (dx/dt)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let x_param = Expression::function("cos", vec![Expression::symbol(t.clone())]);
    /// let y_param = Expression::function("sin", vec![Expression::symbol(t.clone())]);
    /// let dy_dx = ParametricDifferentiation::first_derivative(&x_param, &y_param, t);
    /// ```
    pub fn first_derivative(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> Expression {
        let derivatives = Self::compute_first_derivatives(x_param, y_param, parameter);
        Self::create_division_inline(&derivatives.dy_dt, &derivatives.dx_dt).simplify()
    }

    /// Compute d²y/dx² for parametric curves
    ///
    /// Uses formula: d²y/dx² = (d/dt(dy/dx)) / (dx/dt)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let x_param = Expression::symbol(t.clone());
    /// let y_param = Expression::pow(Expression::symbol(t.clone()), Expression::integer(2));
    /// let d2y_dx2 = ParametricDifferentiation::second_derivative(&x_param, &y_param, t);
    /// ```
    pub fn second_derivative(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> Expression {
        let first_derivs = Self::compute_first_derivatives(x_param, y_param, parameter.clone());

        let dy_dx = Self::create_division_inline(&first_derivs.dy_dt, &first_derivs.dx_dt);

        let d_dt_dy_dx = dy_dx.derivative(parameter);

        Self::create_division_inline(&d_dt_dy_dx, &first_derivs.dx_dt).simplify()
    }

    /// Compute arc length differential
    ///
    /// Formula: ds = √((dx/dt)² + (dy/dt)²)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let x_param = Expression::symbol(t.clone());
    /// let y_param = Expression::pow(Expression::symbol(t.clone()), Expression::integer(2));
    /// let arc_length_diff = ParametricDifferentiation::arc_length_differential(&x_param, &y_param, t);
    /// ```
    pub fn arc_length_differential(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> Expression {
        let derivatives = Self::compute_first_derivatives(x_param, y_param, parameter);

        let sum_of_squares = Expression::add(vec![
            Expression::pow(derivatives.dx_dt, Expression::integer(2)),
            Expression::pow(derivatives.dy_dt, Expression::integer(2)),
        ]);

        Expression::function("sqrt", vec![sum_of_squares]).simplify()
    }

    /// Compute curvature for parametric curves
    ///
    /// Formula: κ = |x'y'' - y'x''| / (x'² + y'²)^(3/2)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let x_param = Expression::function("cos", vec![Expression::symbol(t.clone())]);
    /// let y_param = Expression::function("sin", vec![Expression::symbol(t.clone())]);
    /// let curvature = ParametricDifferentiation::curvature(&x_param, &y_param, t);
    /// ```
    pub fn curvature(x_param: &Expression, y_param: &Expression, parameter: Symbol) -> Expression {
        let derivs = Self::compute_all_derivatives(x_param, y_param, parameter);

        let cross_term = Expression::add(vec![
            Expression::mul(vec![derivs.dx_dt.clone(), derivs.d2y_dt2]),
            Expression::mul(vec![
                Expression::integer(-1),
                derivs.dy_dt.clone(),
                derivs.d2x_dt2,
            ]),
        ]);
        let numerator = Expression::function("abs", vec![cross_term]);

        let sum_of_squares = Expression::add(vec![derivs.dx_dt_sq, derivs.dy_dt_sq]);
        let denominator = Expression::pow(
            sum_of_squares,
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
            ]),
        );

        Self::create_division_inline(&numerator, &denominator).simplify()
    }

    /// Compute first derivatives once
    fn compute_first_derivatives(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> FirstDerivatives {
        FirstDerivatives {
            dx_dt: x_param.derivative(parameter.clone()),
            dy_dt: y_param.derivative(parameter),
        }
    }

    /// Compute all derivatives needed for curvature
    fn compute_all_derivatives(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> AllDerivatives {
        let dx_dt = x_param.derivative(parameter.clone());
        let dy_dt = y_param.derivative(parameter.clone());

        let d2x_dt2 = dx_dt.derivative(parameter.clone());
        let d2y_dt2 = dy_dt.derivative(parameter);

        let dx_dt_sq = Expression::pow(dx_dt.clone(), Expression::integer(2));
        let dy_dt_sq = Expression::pow(dy_dt.clone(), Expression::integer(2));

        AllDerivatives {
            dx_dt,
            dy_dt,
            d2x_dt2,
            d2y_dt2,
            dx_dt_sq,
            dy_dt_sq,
        }
    }

    /// Inlined division creation
    #[inline(always)]
    fn create_division_inline(numerator: &Expression, denominator: &Expression) -> Expression {
        Expression::mul(vec![
            numerator.clone(),
            Expression::pow(denominator.clone(), Expression::integer(-1)),
        ])
    }
}

/// Cached first derivatives
struct FirstDerivatives {
    dx_dt: Expression,
    dy_dt: Expression,
}

/// Cached all derivatives for curvature computation
struct AllDerivatives {
    dx_dt: Expression,
    dy_dt: Expression,
    d2x_dt2: Expression,
    d2y_dt2: Expression,
    dx_dt_sq: Expression,
    dy_dt_sq: Expression,
}

/// Parametric curve analysis
pub struct ParametricCurveAnalysis;

impl ParametricCurveAnalysis {
    /// Find critical points where dy/dx = 0 or undefined
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let x_param = Expression::pow(Expression::symbol(t.clone()), Expression::integer(2));
    /// let y_param = Expression::pow(Expression::symbol(t.clone()), Expression::integer(3));
    /// let critical_t = ParametricCurveAnalysis::critical_points(&x_param, &y_param, t);
    /// ```
    pub fn critical_points(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> Vec<Expression> {
        let derivatives =
            ParametricDifferentiation::compute_first_derivatives(x_param, y_param, parameter);
        vec![derivatives.dy_dt, derivatives.dx_dt]
    }

    /// Analyze tangent vector at parameter value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let x_param = Expression::symbol(t.clone());
    /// let y_param = Expression::pow(Expression::symbol(t.clone()), Expression::integer(2));
    /// let tangent = ParametricCurveAnalysis::tangent_vector(&x_param, &y_param, t);
    /// ```
    pub fn tangent_vector(
        x_param: &Expression,
        y_param: &Expression,
        parameter: Symbol,
    ) -> (Expression, Expression) {
        let derivatives =
            ParametricDifferentiation::compute_first_derivatives(x_param, y_param, parameter);
        (derivatives.dx_dt, derivatives.dy_dt)
    }
}

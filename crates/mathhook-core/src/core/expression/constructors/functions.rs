//! Function and calculus expression constructors

use crate::core::expression::{CalculusData, Expression};
use crate::core::Symbol;

impl Expression {
    /// Create a function expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let expression = Expression::function("sin", vec![expr!(x)]);
    /// ```
    pub fn function<S: Into<String>>(name: S, args: Vec<Expression>) -> Self {
        Self::Function {
            name: name.into(),
            args: Box::new(args),
        }
    }

    /// Create a square root expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let sqrt_2 = Expression::sqrt(Expression::integer(2));
    /// ```
    pub fn sqrt(arg: Expression) -> Self {
        Self::function("sqrt", vec![arg])
    }

    /// Create a derivative expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let expr = Expression::derivative(
    ///     Expression::pow(expr!(x), Expression::integer(2)),
    ///     symbol!(x),
    ///     1,
    /// );
    /// ```
    pub fn derivative(expression: Expression, variable: Symbol, order: u32) -> Self {
        Self::Calculus(Box::new(CalculusData::Derivative {
            expression,
            variable,
            order,
        }))
    }

    /// Create an integral expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let expr = Expression::integral(
    ///     expr!(x),
    ///     symbol!(x),
    /// );
    /// ```
    pub fn integral(integrand: Expression, variable: Symbol) -> Self {
        Self::Calculus(Box::new(CalculusData::Integral {
            integrand,
            variable,
            bounds: None,
        }))
    }

    /// Create a definite integral expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let expr = Expression::definite_integral(
    ///     expr!(x),
    ///     symbol!(x),
    ///     Expression::integer(0),
    ///     Expression::integer(1),
    /// );
    /// ```
    pub fn definite_integral(
        integrand: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(CalculusData::Integral {
            integrand,
            variable,
            bounds: Some((start, end)),
        }))
    }

    /// Create a limit expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let expr = Expression::limit(
    ///     expr!(x),
    ///     symbol!(x),
    ///     Expression::integer(0),
    /// );
    /// ```
    pub fn limit(expression: Expression, variable: Symbol, point: Expression) -> Self {
        use crate::core::expression::LimitDirection;
        Self::Calculus(Box::new(CalculusData::Limit {
            expression,
            variable,
            point,
            direction: LimitDirection::Both,
        }))
    }

    /// Create a sum expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let expr = Expression::sum(
    ///     expr!(i),
    ///     symbol!(i),
    ///     Expression::integer(1),
    ///     Expression::integer(10),
    /// );
    /// ```
    pub fn sum(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(CalculusData::Sum {
            expression,
            variable,
            start,
            end,
        }))
    }

    /// Create a product expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let expr = Expression::product(
    ///     expr!(i),
    ///     symbol!(i),
    ///     Expression::integer(1),
    ///     Expression::integer(10),
    /// );
    /// ```
    pub fn product(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(CalculusData::Product {
            expression,
            variable,
            start,
            end,
        }))
    }
}

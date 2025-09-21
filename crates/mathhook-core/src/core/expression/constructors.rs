//! Expression constructor methods

use super::{
    CalculusData, ComplexData, Expression, IntervalData, MatrixData, PiecewiseData, RelationData,
    RelationType,
};
use crate::core::{MathConstant, Number, Symbol};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

impl Expression {
    /// Create a number expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::number(42);
    /// let expr = Expression::number(3.14);
    /// ```
    pub fn number<T: Into<Number>>(value: T) -> Self {
        Self::Number(value.into())
    }

    /// Create an integer expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::integer(42);
    /// ```
    pub fn integer(value: i64) -> Self {
        Self::Number(Number::integer(value))
    }

    /// Create an integer expression from BigInt
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use num_bigint::BigInt;
    ///
    /// let big_val = BigInt::from(42);
    /// let expr = Expression::big_integer(big_val);
    /// ```
    pub fn big_integer(value: BigInt) -> Self {
        if let Some(small_val) = value.to_i64() {
            Self::Number(Number::integer(small_val))
        } else {
            Self::Number(Number::BigInteger(Box::new(value)))
        }
    }

    /// Create a symbol expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::symbol(Symbol::new("x"));
    /// let expr = Expression::symbol("x");
    /// ```
    pub fn symbol<T: Into<Symbol>>(symbol: T) -> Self {
        Self::Symbol(symbol.into())
    }

    /// Create an addition expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::add(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    /// ]);
    /// ```
    pub fn add(terms: Vec<Expression>) -> Self {
        Self::Add(Box::new(terms))
    }

    /// Create a multiplication expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::mul(vec![
    ///     Expression::integer(2),
    ///     Expression::symbol("x"),
    /// ]);
    ///
    /// let multi = Expression::mul(vec![
    ///     Expression::integer(2),
    ///     Expression::symbol("x"),
    ///     Expression::integer(3),
    /// ]);
    /// ```
    pub fn mul(factors: Vec<Expression>) -> Self {
        Self::Mul(Box::new(factors))
    }

    /// Create a power expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::pow(
    ///     Expression::symbol("x"),
    ///     Expression::integer(2),
    /// );
    /// ```
    pub fn pow(base: Expression, exponent: Expression) -> Self {
        Self::Pow(Box::new(base), Box::new(exponent))
    }

    /// Create a function expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::function("sin", vec![Expression::symbol("x")]);
    /// ```
    pub fn function<S: Into<String>>(name: S, args: Vec<Expression>) -> Self {
        Self::Function {
            name: name.into(),
            args: Box::new(args),
        }
    }

    /// Create a constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MathConstant};
    ///
    /// let expr = Expression::constant(MathConstant::Pi);
    /// ```
    pub fn constant(constant: MathConstant) -> Self {
        Self::Constant(constant)
    }

    /// Create a pi constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let pi = Expression::pi();
    /// ```
    pub fn pi() -> Self {
        Self::Constant(MathConstant::Pi)
    }

    /// Create an e constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let e = Expression::e();
    /// ```
    pub fn e() -> Self {
        Self::Constant(MathConstant::E)
    }

    /// Create an imaginary unit expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let i = Expression::i();
    /// ```
    pub fn i() -> Self {
        Self::Constant(MathConstant::I)
    }

    /// Create an infinity expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let inf = Expression::infinity();
    /// ```
    pub fn infinity() -> Self {
        Self::Constant(MathConstant::Infinity)
    }

    /// Create a matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)],
    /// ]);
    /// ```
    pub fn matrix(rows: Vec<Vec<Expression>>) -> Self {
        Self::Matrix(Box::new(MatrixData { rows }))
    }

    /// Create a complex number expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::complex(
    ///     Expression::integer(3),
    ///     Expression::integer(4),
    /// );
    /// ```
    pub fn complex(real: Expression, imag: Expression) -> Self {
        Self::Complex(Box::new(ComplexData { real, imag }))
    }

    /// Create an equation (equality relation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::equation(
    ///     Expression::symbol("x"),
    ///     Expression::integer(5),
    /// );
    /// ```
    pub fn equation(left: Expression, right: Expression) -> Self {
        Self::Relation(Box::new(RelationData {
            left,
            right,
            relation_type: RelationType::Equal,
        }))
    }

    /// Create a derivative expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::derivative(
    ///     Expression::pow(Expression::symbol("x"), Expression::integer(2)),
    ///     Symbol::new("x"),
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::integral(
    ///     Expression::symbol("x"),
    ///     Symbol::new("x"),
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::definite_integral(
    ///     Expression::symbol("x"),
    ///     Symbol::new("x"),
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::limit(
    ///     Expression::symbol("x"),
    ///     Symbol::new("x"),
    ///     Expression::integer(0),
    /// );
    /// ```
    pub fn limit(expression: Expression, variable: Symbol, point: Expression) -> Self {
        Self::Calculus(Box::new(CalculusData::Limit {
            expression,
            variable,
            point,
            direction: super::LimitDirection::Both,
        }))
    }

    /// Create a sum expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::sum(
    ///     Expression::symbol("i"),
    ///     Symbol::new("i"),
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::product(
    ///     Expression::symbol("i"),
    ///     Symbol::new("i"),
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

    /// Create a set expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let set = Expression::set(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// ]);
    /// ```
    pub fn set(elements: Vec<Expression>) -> Self {
        Self::Set(Box::new(elements))
    }

    /// Create an interval expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let interval = Expression::interval(
    ///     Expression::integer(0),
    ///     Expression::integer(10),
    ///     true,
    ///     false,
    /// );
    /// ```
    pub fn interval(
        start: Expression,
        end: Expression,
        start_inclusive: bool,
        end_inclusive: bool,
    ) -> Self {
        Self::Interval(Box::new(IntervalData {
            start,
            end,
            start_inclusive,
            end_inclusive,
        }))
    }

    /// Create a piecewise function expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let piecewise = Expression::piecewise(
    ///     vec![(Expression::symbol("x"), Expression::integer(1))],
    ///     Some(Expression::integer(0)),
    /// );
    /// ```
    pub fn piecewise(pieces: Vec<(Expression, Expression)>, default: Option<Expression>) -> Self {
        Self::Piecewise(Box::new(PiecewiseData { pieces, default }))
    }

    /// Create a relation expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::core::expression::RelationType;
    ///
    /// let relation = Expression::relation(
    ///     Expression::symbol("x"),
    ///     Expression::integer(5),
    ///     RelationType::Greater,
    /// );
    /// ```
    pub fn relation(left: Expression, right: Expression, relation_type: RelationType) -> Self {
        Self::Relation(Box::new(RelationData {
            left,
            right,
            relation_type,
        }))
    }
}

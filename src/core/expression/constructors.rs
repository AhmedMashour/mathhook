//! Expression constructor methods
//!
//! All methods for creating Expression instances in an ergonomic way.

use super::Expression;
use crate::core::{LimitDirection, MathConstant, Number, RelationType, Symbol};

impl Expression {
    /// Create a new number expression
    #[inline(always)]
    pub fn number<T: Into<Number>>(value: T) -> Self {
        Self::Number(value.into())
    }

    /// Create a new integer expression (optimized path for small integers)
    #[inline(always)]
    pub fn integer<T: Into<num_bigint::BigInt>>(value: T) -> Self {
        let big_int = value.into();
        // Fast path for small integers
        if let Ok(small_int) = i64::try_from(&big_int) {
            Self::Number(Number::SmallInt(small_int))
        } else {
            Self::Number(Number::BigInteger(Box::new(big_int)))
        }
    }

    /// Create a new symbol expression
    #[inline]
    pub fn symbol<T: Into<Symbol>>(symbol: T) -> Self {
        Self::Symbol(symbol.into())
    }

    /// Create an addition expression (optimized)
    #[inline(always)]
    pub fn add(terms: Vec<Expression>) -> Self {
        if terms.is_empty() {
            return Self::integer(0);
        }
        if terms.len() == 1 {
            return terms.into_iter().next().unwrap();
        }
        Self::Add(Box::new(terms))
    }

    /// Create a multiplication expression (optimized)
    #[inline(always)]
    pub fn mul(factors: Vec<Expression>) -> Self {
        if factors.is_empty() {
            return Self::integer(1);
        }
        if factors.len() == 1 {
            return factors.into_iter().next().unwrap();
        }
        Self::Mul(Box::new(factors))
    }

    /// Create a power expression
    #[inline]
    pub fn pow(base: Expression, exponent: Expression) -> Self {
        Self::Pow(Box::new(base), Box::new(exponent))
    }

    /// Create a function call expression (optimized)
    #[inline(always)]
    pub fn function<S: Into<String>>(name: S, args: Vec<Expression>) -> Self {
        Self::Function {
            name: name.into(),
            args: Box::new(args),
        }
    }

    /// Create a complex number expression
    #[inline]
    pub fn complex(real: Expression, imag: Expression) -> Self {
        Self::Complex(Box::new(super::ComplexData { real, imag }))
    }

    /// Create a matrix expression
    #[inline]
    pub fn matrix(rows: Vec<Vec<Expression>>) -> Self {
        Self::Matrix(Box::new(super::MatrixData { rows }))
    }

    /// Create a mathematical constant
    #[inline(always)]
    pub fn constant(constant: MathConstant) -> Self {
        Self::Constant(constant)
    }

    /// Create pi constant
    #[inline(always)]
    pub fn pi() -> Self {
        Self::Constant(MathConstant::Pi)
    }

    /// Create e constant
    #[inline(always)]
    pub fn e() -> Self {
        Self::Constant(MathConstant::E)
    }

    /// Create imaginary unit i
    #[inline(always)]
    pub fn i() -> Self {
        Self::Constant(MathConstant::I)
    }

    /// Create infinity
    #[inline(always)]
    pub fn infinity() -> Self {
        Self::Constant(MathConstant::Infinity)
    }

    /// Create an equation (relation)
    #[inline]
    pub fn equation(left: Expression, right: Expression) -> Self {
        Self::Relation(Box::new(super::RelationData {
            left,
            right,
            relation_type: RelationType::Equal,
        }))
    }

    /// Create a relation with specific type
    #[inline]
    pub fn relation(left: Expression, right: Expression, relation_type: RelationType) -> Self {
        Self::Relation(Box::new(super::RelationData {
            left,
            right,
            relation_type,
        }))
    }

    /// Create a set
    #[inline]
    pub fn set(elements: Vec<Expression>) -> Self {
        Self::Set(Box::new(elements))
    }

    /// Create an interval
    #[inline]
    pub fn interval(
        start: Expression,
        end: Expression,
        start_inclusive: bool,
        end_inclusive: bool,
    ) -> Self {
        Self::Interval(Box::new(super::IntervalData {
            start,
            end,
            start_inclusive,
            end_inclusive,
        }))
    }

    /// Create a piecewise function
    #[inline]
    pub fn piecewise(cases: Vec<(Expression, Expression)>, default: Option<Expression>) -> Self {
        Self::Piecewise(Box::new(super::PiecewiseData { cases, default }))
    }

    /// Create a derivative
    #[inline]
    pub fn derivative(expression: Expression, variable: Symbol, order: u32) -> Self {
        Self::Calculus(Box::new(super::CalculusData::Derivative {
            expression,
            variable,
            order,
        }))
    }

    /// Create an indefinite integral
    #[inline]
    pub fn integral(integrand: Expression, variable: Symbol) -> Self {
        Self::Calculus(Box::new(super::CalculusData::Integral {
            integrand,
            variable,
            bounds: None,
        }))
    }

    /// Create a definite integral
    #[inline]
    pub fn definite_integral(
        integrand: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(super::CalculusData::Integral {
            integrand,
            variable,
            bounds: Some((start, end)),
        }))
    }

    /// Create a limit
    #[inline]
    pub fn limit(expression: Expression, variable: Symbol, approach: Expression) -> Self {
        Self::Calculus(Box::new(super::CalculusData::Limit {
            expression,
            variable,
            approach,
            direction: LimitDirection::Both,
        }))
    }

    /// Create a summation
    #[inline]
    pub fn sum(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(super::CalculusData::Sum {
            expression,
            variable,
            start,
            end,
        }))
    }

    /// Create a product
    #[inline]
    pub fn product(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(super::CalculusData::Product {
            expression,
            variable,
            start,
            end,
        }))
    }
}

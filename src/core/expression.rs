//! Expression representation - the heart of the algebra system

use crate::core::{Number, Symbol};
// Simplify trait not used in this module
use serde::{Deserialize, Serialize};
use std::fmt;

/// Mathematical constants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MathConstant {
    /// Pi (π)
    Pi,
    /// Euler's number (e)
    E,
    /// Imaginary unit (i)
    I,
    /// Positive infinity (∞)
    Infinity,
    /// Negative infinity (-∞)
    NegInfinity,
    /// Undefined/NaN
    Undefined,
    /// Golden ratio (φ)
    GoldenRatio,
    /// Euler-Mascheroni constant (γ)
    EulerGamma,
}

/// Relation types for equations and inequalities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationType {
    /// Equal (=)
    Equal,
    /// Not equal (≠)
    NotEqual,
    /// Less than (<)
    Less,
    /// Greater than (>)
    Greater,
    /// Less than or equal (≤)
    LessEqual,
    /// Greater than or equal (≥)
    GreaterEqual,
    /// Approximately equal (≈)
    Approximately,
}

/// Expression with 32-byte optimization
/// Memory-optimized with boxed vectors for cache
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Optimized number representation
    Number(Number),
    /// Symbol (variable)
    Symbol(Symbol),
    /// Addition with boxed vector for memory
    Add(Box<Vec<Expression>>),
    /// Multiplication with boxed vector for memory
    Mul(Box<Vec<Expression>>),
    /// Power operation with boxed expressions
    Pow(Box<Expression>, Box<Expression>),
    /// Function call with boxed arguments
    Function {
        name: String,
        args: Box<Vec<Expression>>,
    },
    /// Complex number representation
    Complex {
        real: Box<Expression>,
        imag: Box<Expression>,
    },
    /// Matrix representation
    Matrix(Box<Vec<Vec<Expression>>>),
    /// Mathematical constants
    Constant(MathConstant),
    /// Equations and relations
    Relation {
        left: Box<Expression>,
        right: Box<Expression>,
        relation_type: RelationType,
    },
    /// Piecewise functions
    Piecewise {
        cases: Box<Vec<(Expression, Expression)>>, // (condition, value)
        default: Option<Box<Expression>>,
    },
    /// Set representation
    Set(Box<Vec<Expression>>),
    /// Interval notation
    Interval {
        start: Box<Expression>,
        end: Box<Expression>,
        start_inclusive: bool,
        end_inclusive: bool,
    },
    /// Derivative
    Derivative {
        expression: Box<Expression>,
        variable: Symbol,
        order: u32,
    },
    /// Integral
    Integral {
        integrand: Box<Expression>,
        variable: Symbol,
        bounds: Option<(Box<Expression>, Box<Expression>)>, // None = indefinite, Some = definite
    },
    /// Limit
    Limit {
        expression: Box<Expression>,
        variable: Symbol,
        approach: Box<Expression>,
        direction: LimitDirection,
    },
    /// Summation
    Sum {
        expression: Box<Expression>,
        variable: Symbol,
        start: Box<Expression>,
        end: Box<Expression>,
    },
    /// Product
    Product {
        expression: Box<Expression>,
        variable: Symbol,
        start: Box<Expression>,
        end: Box<Expression>,
    },
}

/// Direction for limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LimitDirection {
    /// Approach from both sides
    Both,
    /// Approach from the left
    Left,
    /// Approach from the right
    Right,
}

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
        Self::Complex {
            real: Box::new(real),
            imag: Box::new(imag),
        }
    }

    /// Create a matrix expression
    #[inline]
    pub fn matrix(rows: Vec<Vec<Expression>>) -> Self {
        Self::Matrix(Box::new(rows))
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
        Self::Relation {
            left: Box::new(left),
            right: Box::new(right),
            relation_type: RelationType::Equal,
        }
    }

    /// Create a relation with specific type
    #[inline]
    pub fn relation(left: Expression, right: Expression, relation_type: RelationType) -> Self {
        Self::Relation {
            left: Box::new(left),
            right: Box::new(right),
            relation_type,
        }
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
        Self::Interval {
            start: Box::new(start),
            end: Box::new(end),
            start_inclusive,
            end_inclusive,
        }
    }

    /// Create a piecewise function
    #[inline]
    pub fn piecewise(cases: Vec<(Expression, Expression)>, default: Option<Expression>) -> Self {
        Self::Piecewise {
            cases: Box::new(cases),
            default: default.map(Box::new),
        }
    }

    /// Create a derivative
    #[inline]
    pub fn derivative(expression: Expression, variable: Symbol, order: u32) -> Self {
        Self::Derivative {
            expression: Box::new(expression),
            variable,
            order,
        }
    }

    /// Create an indefinite integral
    #[inline]
    pub fn integral(integrand: Expression, variable: Symbol) -> Self {
        Self::Integral {
            integrand: Box::new(integrand),
            variable,
            bounds: None,
        }
    }

    /// Create a definite integral
    #[inline]
    pub fn definite_integral(
        integrand: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Integral {
            integrand: Box::new(integrand),
            variable,
            bounds: Some((Box::new(start), Box::new(end))),
        }
    }

    /// Create a limit
    #[inline]
    pub fn limit(expression: Expression, variable: Symbol, approach: Expression) -> Self {
        Self::Limit {
            expression: Box::new(expression),
            variable,
            approach: Box::new(approach),
            direction: LimitDirection::Both,
        }
    }

    /// Create a summation
    #[inline]
    pub fn sum(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Sum {
            expression: Box::new(expression),
            variable,
            start: Box::new(start),
            end: Box::new(end),
        }
    }

    /// Create a product
    #[inline]
    pub fn product(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Product {
            expression: Box::new(expression),
            variable,
            start: Box::new(start),
            end: Box::new(end),
        }
    }

    /// Check if the expression is zero (optimized)
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_zero(),
            _ => false,
        }
    }

    /// Check if the expression is one (optimized)
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_one(),
            _ => false,
        }
    }

    /// Get the numeric coefficient if this is a simple numeric expression
    #[inline]
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Expression::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Get the symbol if this is a simple symbol expression
    #[inline]
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Expression::Symbol(s) => Some(s),
            _ => None,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Add(terms) => {
                if terms.is_empty() {
                    write!(f, "0")
                } else {
                    write!(f, "(")?;
                    for (i, term) in terms.iter().enumerate() {
                        if i > 0 {
                            write!(f, " + ")?;
                        }
                        write!(f, "{}", term)?;
                    }
                    write!(f, ")")
                }
            }
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    write!(f, "1")
                } else {
                    write!(f, "(")?;
                    for (i, factor) in factors.iter().enumerate() {
                        if i > 0 {
                            write!(f, " * ")?;
                        }
                        write!(f, "{}", factor)?;
                    }
                    write!(f, ")")
                }
            }
            Expression::Pow(base, exp) => {
                write!(f, "({})^({})", base, exp)
            }
            Expression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Complex { real, imag } => {
                write!(f, "({} + {}i)", real, imag)
            }
            Expression::Matrix(rows) => {
                write!(f, "[")?;
                for (i, row) in rows.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "[")?;
                    for (j, elem) in row.iter().enumerate() {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", elem)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            Expression::Constant(c) => match c {
                MathConstant::Pi => write!(f, "π"),
                MathConstant::E => write!(f, "e"),
                MathConstant::I => write!(f, "i"),
                MathConstant::Infinity => write!(f, "∞"),
                MathConstant::NegInfinity => write!(f, "-∞"),
                MathConstant::Undefined => write!(f, "undefined"),
                MathConstant::GoldenRatio => write!(f, "φ"),
                MathConstant::EulerGamma => write!(f, "γ"),
            },
            Expression::Relation {
                left,
                right,
                relation_type,
            } => {
                let symbol = match relation_type {
                    RelationType::Equal => "=",
                    RelationType::NotEqual => "≠",
                    RelationType::Less => "<",
                    RelationType::Greater => ">",
                    RelationType::LessEqual => "≤",
                    RelationType::GreaterEqual => "≥",
                    RelationType::Approximately => "≈",
                };
                write!(f, "{} {} {}", left, symbol, right)
            }
            Expression::Set(elements) => {
                write!(f, "{{")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "}}")
            }
            Expression::Interval {
                start,
                end,
                start_inclusive,
                end_inclusive,
            } => {
                let start_bracket = if *start_inclusive { "[" } else { "(" };
                let end_bracket = if *end_inclusive { "]" } else { ")" };
                write!(f, "{}{}, {}{}", start_bracket, start, end, end_bracket)
            }
            Expression::Piecewise { cases, default } => {
                write!(f, "piecewise(")?;
                for (i, (condition, value)) in cases.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} if {}", value, condition)?;
                }
                if let Some(def) = default {
                    write!(f, ", {} otherwise", def)?;
                }
                write!(f, ")")
            }
            Expression::Derivative {
                expression,
                variable,
                order,
            } => {
                if *order == 1 {
                    write!(f, "d/d{} ({})", variable.name(), expression)
                } else {
                    write!(
                        f,
                        "d^{}/d{}^{} ({})",
                        order,
                        variable.name(),
                        order,
                        expression
                    )
                }
            }
            Expression::Integral {
                integrand,
                variable,
                bounds,
            } => match bounds {
                None => write!(f, "∫ {} d{}", integrand, variable.name()),
                Some((start, end)) => write!(
                    f,
                    "∫[{} to {}] {} d{}",
                    start,
                    end,
                    integrand,
                    variable.name()
                ),
            },
            Expression::Limit {
                expression,
                variable,
                approach,
                direction,
            } => {
                let dir_str = match direction {
                    LimitDirection::Both => "",
                    LimitDirection::Left => "⁻",
                    LimitDirection::Right => "⁺",
                };
                write!(
                    f,
                    "lim({} → {}{}) {}",
                    variable.name(),
                    approach,
                    dir_str,
                    expression
                )
            }
            Expression::Sum {
                expression,
                variable,
                start,
                end,
            } => {
                write!(
                    f,
                    "Σ({}={} to {}) {}",
                    variable.name(),
                    start,
                    end,
                    expression
                )
            }
            Expression::Product {
                expression,
                variable,
                start,
                end,
            } => {
                write!(
                    f,
                    "Π({}={} to {}) {}",
                    variable.name(),
                    start,
                    end,
                    expression
                )
            }
        }
    }
}

// Conversion implementations
impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Self::integer(value)
    }
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Self::integer(value)
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Self::Number(Number::float(value))
    }
}

impl From<Symbol> for Expression {
    fn from(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }
}

impl From<&str> for Expression {
    fn from(name: &str) -> Self {
        Self::Symbol(Symbol::new(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_creation() {
        let num_expr = Expression::integer(42);
        let sym_expr = Expression::symbol(Symbol::new("x"));
        let add_expr = Expression::add(vec![num_expr.clone(), sym_expr.clone()]);

        assert!(matches!(num_expr, Expression::Number(_)));
        assert!(matches!(sym_expr, Expression::Symbol(_)));
        assert!(matches!(add_expr, Expression::Add(_)));
    }

    #[test]
    fn test_zero_and_one_detection() {
        let zero = Expression::integer(0);
        let one = Expression::integer(1);
        let x = Expression::symbol(Symbol::new("x"));

        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!(!x.is_zero());
        assert!(!x.is_one());
    }

    #[test]
    fn test_display() {
        let x = Expression::symbol(Symbol::new("x"));
        let two = Expression::integer(2);
        let sum = Expression::add(vec![x.clone(), two.clone()]);

        assert_eq!(format!("{}", x), "x");
        assert_eq!(format!("{}", two), "2");
        assert_eq!(format!("{}", sum), "(x + 2)");
    }
}

//! Clean parse and stringify API for mathematical expressions

use crate::core::expression::LimitDirection;
use crate::core::{Expression, MathConstant, Number, Symbol};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json;

/// Mathematical expression serialization API
pub struct MathSerializer;

impl MathSerializer {
    /// 🎯 PARSE - Convert serialized data to Expression
    pub fn parse(data_str: &str) -> Result<Expression, SerializationError> {
        let expr_data: ExpressionData = serde_json::from_str(data_str)
            .map_err(|e| SerializationError::InvalidFormat(e.to_string()))?;
        Self::data_to_expression(expr_data)
    }

    /// 🎯 STRINGIFY - Convert Expression to serialized format
    pub fn stringify(expr: &Expression) -> Result<String, SerializationError> {
        let expr_data = Self::expression_to_data(expr);
        serde_json::to_string_pretty(&expr_data)
            .map_err(|e| SerializationError::SerializationError(e.to_string()))
    }

    /// 🎯 STRINGIFY COMPACT - Compact format
    pub fn stringify_compact(expr: &Expression) -> Result<String, SerializationError> {
        let expr_data = Self::expression_to_data(expr);
        serde_json::to_string(&expr_data)
            .map_err(|e| SerializationError::SerializationError(e.to_string()))
    }

    /// Convert ExpressionData to Expression
    fn data_to_expression(expr_data: ExpressionData) -> Result<Expression, SerializationError> {
        match expr_data {
            ExpressionData::Number { value } => Ok(Expression::integer(value)),
            ExpressionData::Float { value } => Ok(Expression::number(Number::float(value))),
            ExpressionData::Symbol { name } => Ok(Expression::symbol(Symbol::new(&name))),

            ExpressionData::Add { terms } => {
                let expr_terms: Result<Vec<Expression>, SerializationError> =
                    terms.into_iter().map(Self::data_to_expression).collect();
                Ok(Expression::add(expr_terms?))
            }

            ExpressionData::Mul { factors } => {
                let expr_factors: Result<Vec<Expression>, SerializationError> =
                    factors.into_iter().map(Self::data_to_expression).collect();
                Ok(Expression::mul(expr_factors?))
            }

            ExpressionData::Pow { base, exponent } => {
                let base_expr = Self::data_to_expression(*base)?;
                let exp_expr = Self::data_to_expression(*exponent)?;
                Ok(Expression::pow(base_expr, exp_expr))
            }

            ExpressionData::Function { name, args } => {
                let expr_args: Result<Vec<Expression>, SerializationError> =
                    args.into_iter().map(Self::data_to_expression).collect();
                Ok(Expression::function(name, expr_args?))
            }

            ExpressionData::Complex { real, imag } => {
                let real_expr = Self::data_to_expression(*real)?;
                let imag_expr = Self::data_to_expression(*imag)?;
                Ok(Expression::complex(real_expr, imag_expr))
            }

            ExpressionData::Constant { constant } => Ok(Expression::constant(constant)),

            ExpressionData::Derivative {
                expression,
                variable,
                order,
            } => {
                let expr = Self::data_to_expression(*expression)?;
                Ok(Expression::derivative(expr, Symbol::new(&variable), order))
            }

            ExpressionData::Integral {
                integrand,
                variable,
                bounds,
            } => {
                let integrand_expr = Self::data_to_expression(*integrand)?;
                let var_symbol = Symbol::new(&variable);

                match bounds {
                    None => Ok(Expression::integral(integrand_expr, var_symbol)),
                    Some((start, end)) => {
                        let start_expr = Self::data_to_expression(*start)?;
                        let end_expr = Self::data_to_expression(*end)?;
                        Ok(Expression::definite_integral(
                            integrand_expr,
                            var_symbol,
                            start_expr,
                            end_expr,
                        ))
                    }
                }
            }

            ExpressionData::Limit {
                expression,
                variable,
                approach,
                direction: _,
            } => {
                let expr = Self::data_to_expression(*expression)?;
                let approach_expr = Self::data_to_expression(*approach)?;
                Ok(Expression::limit(
                    expr,
                    Symbol::new(&variable),
                    approach_expr,
                ))
            }
        }
    }

    /// Convert Expression to ExpressionData
    fn expression_to_data(expr: &Expression) -> ExpressionData {
        match expr {
            Expression::Number(Number::Integer(n)) => ExpressionData::Number { value: *n },
            Expression::Number(Number::BigInteger(n)) => ExpressionData::Number {
                value: n.to_string().parse().unwrap_or(0),
            },
            Expression::Number(Number::Float(f)) => ExpressionData::Float { value: *f },
            Expression::Number(Number::Rational(r)) => {
                let float_val =
                    r.numer().to_f64().unwrap_or(0.0) / r.denom().to_f64().unwrap_or(1.0);
                ExpressionData::Float { value: float_val }
            }

            Expression::Symbol(s) => ExpressionData::Symbol {
                name: s.name().to_string(),
            },

            Expression::Add(terms) => ExpressionData::Add {
                terms: terms.iter().map(Self::expression_to_data).collect(),
            },

            Expression::Mul(factors) => ExpressionData::Mul {
                factors: factors.iter().map(Self::expression_to_data).collect(),
            },

            Expression::Pow(base, exp) => ExpressionData::Pow {
                base: Box::new(Self::expression_to_data(base)),
                exponent: Box::new(Self::expression_to_data(exp)),
            },

            Expression::Function { name, args } => ExpressionData::Function {
                name: name.clone(),
                args: args.iter().map(Self::expression_to_data).collect(),
            },

            Expression::Complex(complex_data) => ExpressionData::Complex {
                real: Box::new(Self::expression_to_data(&complex_data.real)),
                imag: Box::new(Self::expression_to_data(&complex_data.imag)),
            },

            Expression::Constant(c) => ExpressionData::Constant {
                constant: c.clone(),
            },

            Expression::Calculus(calculus_data) => {
                use crate::core::expression::CalculusData;
                match calculus_data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => ExpressionData::Derivative {
                        expression: Box::new(Self::expression_to_data(expression)),
                        variable: variable.name().to_string(),
                        order: *order,
                    },
                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => ExpressionData::Integral {
                        integrand: Box::new(Self::expression_to_data(integrand)),
                        variable: variable.name().to_string(),
                        bounds: bounds.as_ref().map(|(start, end)| {
                            (
                                Box::new(Self::expression_to_data(start)),
                                Box::new(Self::expression_to_data(end)),
                            )
                        }),
                    },
                    _ => ExpressionData::Function {
                        name: "calculus_operation".to_string(),
                        args: vec![],
                    },
                }
            }

            // For other types, use placeholder for now
            _ => ExpressionData::Symbol {
                name: "placeholder".to_string(),
            },
        }
    }
}

/// Serializable representation of mathematical expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExpressionData {
    Number {
        value: i64,
    },
    Float {
        value: f64,
    },
    Symbol {
        name: String,
    },
    Add {
        terms: Vec<ExpressionData>,
    },
    Mul {
        factors: Vec<ExpressionData>,
    },
    Pow {
        base: Box<ExpressionData>,
        exponent: Box<ExpressionData>,
    },
    Function {
        name: String,
        args: Vec<ExpressionData>,
    },
    Complex {
        real: Box<ExpressionData>,
        imag: Box<ExpressionData>,
    },
    Constant {
        constant: MathConstant,
    },
    Derivative {
        expression: Box<ExpressionData>,
        variable: String,
        order: u32,
    },
    Integral {
        integrand: Box<ExpressionData>,
        variable: String,
        bounds: Option<(Box<ExpressionData>, Box<ExpressionData>)>,
    },
    Limit {
        expression: Box<ExpressionData>,
        variable: String,
        approach: Box<ExpressionData>,
        direction: LimitDirection,
    },
}

/// Errors for serialization operations
#[derive(Debug, Clone)]
pub enum SerializationError {
    InvalidFormat(String),
    SerializationError(String),
    ParseError(String),
    UnsupportedType(String),
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            SerializationError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            SerializationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SerializationError::UnsupportedType(msg) => write!(f, "Unsupported type: {}", msg),
        }
    }
}

impl std::error::Error for SerializationError {}

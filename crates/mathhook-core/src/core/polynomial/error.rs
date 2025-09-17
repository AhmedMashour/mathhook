//! Polynomial Error Types
//!
//! Comprehensive error handling for polynomial operations.

use crate::core::{Expression, Symbol};
use std::fmt;

/// Errors that can occur during polynomial operations
#[derive(Debug, Clone, PartialEq)]
pub enum PolynomialError {
    /// Division by zero polynomial
    DivisionByZero,

    /// Division did not yield exact result (non-zero remainder)
    DivisionNotExact {
        dividend: Expression,
        divisor: Expression,
    },

    /// Not a valid polynomial (contains transcendental functions, negative powers, etc.)
    NotPolynomial {
        expression: Expression,
        reason: String,
    },

    /// Polynomial has wrong number of variables for the operation
    WrongVariableCount {
        expected: usize,
        got: usize,
        operation: &'static str,
    },

    /// Variable not found in polynomial
    VariableNotFound { var: Symbol },

    /// Degree computation failed (expression too complex or not polynomial)
    DegreeComputationFailed { expression: Expression },

    /// GCD computation failed
    GcdComputationFailed { reason: String },

    /// Factorization failed
    FactorizationFailed { reason: String },

    /// Numeric overflow during computation
    NumericOverflow { operation: &'static str },

    /// Algorithm reached maximum iterations without converging
    MaxIterationsExceeded {
        operation: &'static str,
        limit: usize,
    },
}

impl fmt::Display for PolynomialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolynomialError::DivisionByZero => {
                write!(f, "division by zero polynomial")
            }
            PolynomialError::DivisionNotExact { dividend, divisor } => {
                write!(
                    f,
                    "division of {} by {} does not yield exact result",
                    dividend, divisor
                )
            }
            PolynomialError::NotPolynomial { expression, reason } => {
                write!(f, "not a polynomial: {} ({})", expression, reason)
            }
            PolynomialError::WrongVariableCount {
                expected,
                got,
                operation,
            } => {
                write!(
                    f,
                    "{} requires {} variable(s), got {}",
                    operation, expected, got
                )
            }
            PolynomialError::VariableNotFound { var } => {
                write!(f, "variable {:?} not found in polynomial", var)
            }
            PolynomialError::DegreeComputationFailed { expression } => {
                write!(f, "failed to compute degree of {}", expression)
            }
            PolynomialError::GcdComputationFailed { reason } => {
                write!(f, "GCD computation failed: {}", reason)
            }
            PolynomialError::FactorizationFailed { reason } => {
                write!(f, "factorization failed: {}", reason)
            }
            PolynomialError::NumericOverflow { operation } => {
                write!(f, "numeric overflow during {}", operation)
            }
            PolynomialError::MaxIterationsExceeded { operation, limit } => {
                write!(f, "{} exceeded maximum iterations ({})", operation, limit)
            }
        }
    }
}

impl std::error::Error for PolynomialError {}

/// Result type for polynomial operations
#[allow(dead_code)]
pub type PolynomialResult<T> = Result<T, PolynomialError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PolynomialError::DivisionByZero;
        assert_eq!(format!("{}", err), "division by zero polynomial");
    }

    #[test]
    fn test_wrong_variable_count() {
        let err = PolynomialError::WrongVariableCount {
            expected: 1,
            got: 3,
            operation: "univariate division",
        };
        assert!(format!("{}", err).contains("1 variable(s)"));
        assert!(format!("{}", err).contains("got 3"));
    }
}

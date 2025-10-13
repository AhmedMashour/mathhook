//! Mathematical error types for domain violations and computation failures
//!
//! This module defines comprehensive error types for mathematical operations
//! that can fail due to domain restrictions, undefined behavior, or other
//! mathematical constraints.

use crate::core::Expression;
use std::fmt;

/// Comprehensive mathematical error type
///
/// Represents all possible error conditions that can occur during mathematical
/// operations, including domain violations, singularities, and undefined behavior.
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Domain error - operation not valid for given input
    ///
    /// # Examples
    ///
    /// - sqrt(-1) in real domain
    /// - arcsin(2) in real domain (domain is [-1, 1])
    DomainError {
        operation: String,
        value: Expression,
        reason: String,
    },

    /// Division by zero
    ///
    /// # Examples
    ///
    /// - 1/0
    /// - 0^(-1) = 1/0
    DivisionByZero,

    /// Undefined expression
    ///
    /// # Examples
    ///
    /// - 0^0 (indeterminate form)
    /// - 0/0 (indeterminate form)
    Undefined {
        expression: Expression,
        reason: String,
    },

    /// Numeric overflow during computation
    NumericOverflow {
        operation: String,
    },

    /// Feature not yet implemented
    NotImplemented {
        feature: String,
    },

    /// Pole singularity - function approaches infinity
    ///
    /// # Examples
    ///
    /// - tan(π/2)
    /// - log(0)
    /// - 1/0
    Pole {
        function: String,
        at: Expression,
    },

    /// Branch cut issue for multi-valued functions
    ///
    /// # Examples
    ///
    /// - log(-1) in real domain (requires complex domain)
    /// - sqrt(-1) in real domain (branch cut on negative real axis)
    BranchCut {
        function: String,
        value: Expression,
    },
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DomainError { operation, value, reason } => {
                write!(f, "Domain error in {}: {} ({})", operation, value, reason)
            }
            MathError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            MathError::Undefined { expression, reason } => {
                write!(f, "Undefined: {} ({})", expression, reason)
            }
            MathError::NumericOverflow { operation } => {
                write!(f, "Numeric overflow in {}", operation)
            }
            MathError::NotImplemented { feature } => {
                write!(f, "Not yet implemented: {}", feature)
            }
            MathError::Pole { function, at } => {
                write!(f, "Pole singularity: {}({}) is undefined", function, at)
            }
            MathError::BranchCut { function, value } => {
                write!(f, "Branch cut: {}({}) requires domain specification (use complex domain or specify branch)", function, value)
            }
        }
    }
}

impl std::error::Error for MathError {}

/// Type alias for mathematical operations that can fail
pub type MathResult<T> = Result<T, MathError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Expression;

    #[test]
    fn test_error_display() {
        let err = MathError::DivisionByZero;
        assert_eq!(err.to_string(), "Division by zero");

        let err = MathError::DomainError {
            operation: "sqrt".to_string(),
            value: Expression::integer(-1),
            reason: "sqrt requires non-negative input in real domain".to_string(),
        };
        assert!(err.to_string().contains("Domain error in sqrt"));

        let err = MathError::Pole {
            function: "log".to_string(),
            at: Expression::integer(0),
        };
        assert!(err.to_string().contains("Pole singularity"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = MathError::DivisionByZero;
        let err2 = MathError::DivisionByZero;
        assert_eq!(err1, err2);

        let err3 = MathError::NotImplemented {
            feature: "groebner bases".to_string(),
        };
        assert_ne!(err1, err3);
    }
}

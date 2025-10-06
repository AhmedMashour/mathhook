//! Mathematical operations and queries for expressions
//!
//! Methods for checking properties and extracting information from expressions.

use super::Expression;
use crate::core::{Number, Symbol};
use crate::matrix::unified::CoreMatrixOps;

impl Expression {
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

    /// Evaluate method calls on expressions
    ///
    /// This handles method calls like matrix.det(), matrix.trace(), etc.
    /// by calling the appropriate methods on the underlying objects.
    pub fn evaluate_method_call(&self) -> Expression {
        match self {
            Expression::MethodCall(method_data) => {
                let object = &method_data.object;
                let method_name = &method_data.method_name;
                let _args = &method_data.args;

                // Handle matrix method calls
                if let Expression::Matrix(matrix) = object {
                    match method_name.as_str() {
                        "det" | "determinant" => matrix.determinant(),
                        "trace" => matrix.trace(),
                        "transpose" => Expression::Matrix(Box::new(matrix.transpose())),
                        "inverse" => Expression::Matrix(Box::new(matrix.inverse())),
                        _ => {
                            // Unknown method - return as is
                            self.clone()
                        }
                    }
                } else {
                    // For non-matrix objects, we might need to evaluate the object first
                    // and then try the method call again
                    let evaluated_object = object.evaluate_method_call();
                    if let Expression::Matrix(matrix) = &evaluated_object {
                        match method_name.as_str() {
                            "det" | "determinant" => matrix.determinant(),
                            "trace" => matrix.trace(),
                            "transpose" => Expression::Matrix(Box::new(matrix.transpose())),
                            "inverse" => Expression::Matrix(Box::new(matrix.inverse())),
                            _ => self.clone(),
                        }
                    } else {
                        // Not a matrix - return as is
                        self.clone()
                    }
                }
            }
            _ => self.clone(),
        }
    }
}

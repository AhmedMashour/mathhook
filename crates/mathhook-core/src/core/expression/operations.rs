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

    /// Check if this expression represents a mathematical function
    ///
    /// Returns true for expressions like sin(x), cos(x), etc.
    /// Now integrated with Universal Function Intelligence System
    pub fn is_function(&self) -> bool {
        match self {
            Expression::Function { name, .. } => {
                // Check if function has intelligence in the registry
                use crate::functions::intelligence::UNIVERSAL_REGISTRY;
                UNIVERSAL_REGISTRY.has_intelligence(name)
            }
            _ => false,
        }
    }

    /// Get function intelligence properties if available
    ///
    /// Seamless integration between core expressions and function intelligence
    pub fn get_function_intelligence(&self) -> Option<&crate::functions::FunctionProperties> {
        if let Expression::Function { name, .. } = self {
            use crate::functions::intelligence::UNIVERSAL_REGISTRY;
            UNIVERSAL_REGISTRY.get_properties(name)
        } else {
            None
        }
    }

    /// Generate educational explanation for function expressions
    ///
    /// Perfect integration with the educational system
    pub fn explain_function(&self) -> Vec<crate::educational::step_by_step::Step> {
        if let Expression::Function { name, args } = self {
            use crate::functions::intelligence::UNIVERSAL_REGISTRY;
            UNIVERSAL_REGISTRY.explain_function(name, args)
        } else {
            vec![crate::educational::step_by_step::Step::new(
                "Expression Type",
                "This is not a function expression".to_string(),
            )]
        }
    }

    /// High-performance function evaluation using SIMD when beneficial
    ///
    /// Seamless integration with the evaluation system
    pub fn evaluate_function_simd(&self, values: &[f64]) -> Option<Vec<f64>> {
        if let Expression::Function { name, .. } = self {
            use crate::functions::FunctionEvaluator;
            let evaluator = FunctionEvaluator::new();
            evaluator.evaluate_bulk_f64(name, values)
        } else {
            None
        }
    }
}

//! Mathematical operations and queries for expressions
//!
//! Methods for checking properties and extracting information from expressions.

use super::Expression;
use crate::core::{Number, Symbol};
use crate::matrices::unified::CoreMatrixOps;
use crate::simplify::Simplify;
use num_traits::Signed;
use std::sync::Arc;

impl Expression {
    /// Check if the expression is zero (robust version with simplification)
    ///
    /// This method simplifies the expression first before checking if it equals zero.
    /// It correctly detects zero for expressions like:
    /// - Literal zeros: `0`, `0.0`, `0/1`
    /// - Symbolic zeros: `x - x`, `0 * y`, `sin(0)`
    /// - Simplified zeros: `(x + 1) - (x + 1)`
    ///
    /// # Performance Note
    ///
    /// This method calls `simplify()`, which may be expensive for complex expressions.
    /// For performance-critical code where you only need to check literal zeros,
    /// use `is_zero_fast()` instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::Expression;
    ///
    /// // Literal zero
    /// assert!(Expression::integer(0).is_zero());
    ///
    /// // Symbolic zero after simplification
    /// let expr = Expression::mul(vec![Expression::integer(0), Expression::integer(5)]);
    /// assert!(expr.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_zero(),
            _ => {
                let simplified = self.simplify();
                matches!(simplified, Expression::Number(n) if n.is_zero())
            }
        }
    }

    /// Fast literal zero check without simplification
    ///
    /// This is a performance-optimized version that only checks if the expression
    /// is literally `Number(0)`. It does NOT simplify the expression first.
    ///
    /// Use this in performance-critical loops where you know the expression
    /// is already in simplified form, or where you specifically want to check
    /// for literal zeros only.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Detects literal zero
    /// assert!(Expression::integer(0).is_zero_fast());
    ///
    /// // Mul constructor auto-simplifies, so this IS detected
    /// let expr = Expression::mul(vec![Expression::integer(0), Expression::integer(5)]);
    /// assert!(expr.is_zero_fast()); // Simplified to 0 by constructor
    ///
    /// // Example of what is_zero_fast() does NOT detect (without simplification):
    /// // If we had a raw unsimplified Mul expression, is_zero_fast() wouldn't detect it
    /// ```
    #[inline(always)]
    pub fn is_zero_fast(&self) -> bool {
        matches!(self, Expression::Number(n) if n.is_zero())
    }

    /// Check if the expression is one (robust version with simplification)
    ///
    /// This method simplifies the expression first before checking if it equals one.
    /// It correctly detects one for expressions like:
    /// - Literal ones: `1`, `1.0`, `2/2`
    /// - Symbolic ones: `x / x`, `x^0`, `cos(0)`
    /// - Simplified ones: `(x + 1) / (x + 1)`
    ///
    /// # Performance Note
    ///
    /// This method calls `simplify()`, which may be expensive for complex expressions.
    /// For performance-critical code where you only need to check literal ones,
    /// use `is_one_fast()` instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::Expression;
    ///
    /// // Literal one
    /// assert!(Expression::integer(1).is_one());
    ///
    /// // Symbolic one after simplification
    /// let expr = Expression::pow(Expression::integer(5), Expression::integer(0));
    /// assert!(expr.is_one());
    /// ```
    pub fn is_one(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_one(),
            _ => {
                let simplified = self.simplify();
                matches!(simplified, Expression::Number(n) if n.is_one())
            }
        }
    }

    /// Fast literal one check without simplification
    ///
    /// This is a performance-optimized version that only checks if the expression
    /// is literally `Number(1)`. It does NOT simplify the expression first.
    ///
    /// Use this in performance-critical loops where you know the expression
    /// is already in simplified form, or where you specifically want to check
    /// for literal ones only.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Detects literal one
    /// assert!(Expression::integer(1).is_one_fast());
    ///
    /// // Pow constructor auto-simplifies, so x^0 = 1 IS detected
    /// let expr = Expression::pow(Expression::integer(5), Expression::integer(0));
    /// assert!(expr.is_one_fast()); // Simplified to 1 by constructor
    ///
    /// // is_one_fast() checks ONLY for literal Number(1)
    /// // It does not simplify complex expressions first
    /// ```
    #[inline(always)]
    pub fn is_one_fast(&self) -> bool {
        matches!(self, Expression::Number(n) if n.is_one())
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

    /// Check if this expression is a negative number
    ///
    /// Returns true if the expression is a negative integer, rational, or float.
    /// Returns false for symbolic expressions (even if they might evaluate to negative).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// assert!(Expression::integer(-5).is_negative_number());
    /// assert!(Expression::rational(-1, 2).is_negative_number());
    /// assert!(!Expression::integer(5).is_negative_number());
    /// assert!(!Expression::symbol("x").is_negative_number()); // Symbolic, not a number
    /// ```
    #[inline]
    pub fn is_negative_number(&self) -> bool {
        match self {
            Expression::Number(Number::Integer(i)) => *i < 0,
            Expression::Number(Number::Rational(r)) => r.is_negative(),
            Expression::Number(Number::Float(f)) => *f < 0.0,
            _ => false,
        }
    }

    /// Check if this expression is a positive number
    ///
    /// Returns true if the expression is a positive integer, rational, or float.
    /// Returns false for symbolic expressions (even if they might evaluate to positive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// assert!(Expression::integer(5).is_positive_number());
    /// assert!(Expression::rational(1, 2).is_positive_number());
    /// assert!(!Expression::integer(-5).is_positive_number());
    /// assert!(!Expression::symbol("x").is_positive_number()); // Symbolic, not a number
    /// ```
    #[inline]
    pub fn is_positive_number(&self) -> bool {
        match self {
            Expression::Number(Number::Integer(i)) => *i > 0,
            Expression::Number(Number::Rational(r)) => r.is_positive(),
            Expression::Number(Number::Float(f)) => *f > 0.0,
            _ => false,
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

                if let Expression::Matrix(matrix) = object {
                    match method_name.as_ref() {
                        "det" | "determinant" => matrix
                            .determinant()
                            .unwrap_or_else(|_| Expression::function("undefined", vec![])),
                        "trace" => matrix.trace(),
                        "transpose" => Expression::Matrix(Arc::new(matrix.transpose())),
                        "inverse" => Expression::Matrix(Arc::new(matrix.inverse())),
                        _ => self.clone(),
                    }
                } else {
                    let evaluated_object = object.evaluate_method_call();
                    if let Expression::Matrix(matrix) = &evaluated_object {
                        match method_name.as_ref() {
                            "det" | "determinant" => matrix
                                .determinant()
                                .unwrap_or_else(|_| Expression::function("undefined", vec![])),
                            "trace" => matrix.trace(),
                            "transpose" => Expression::Matrix(Arc::new(matrix.transpose())),
                            "inverse" => Expression::Matrix(Arc::new(matrix.inverse())),
                            _ => self.clone(),
                        }
                    } else {
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
                "This is not a function expression".to_owned(),
            )]
        }
    }
}

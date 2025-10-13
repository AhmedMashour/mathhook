//! Mathematical operations and queries for expressions
//!
//! Methods for checking properties and extracting information from expressions.

use super::Expression;
use crate::core::{Number, Symbol};
use crate::matrix::unified::CoreMatrixOps;
use crate::simplify::Simplify;

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

    /// Evaluate expression with domain checking
    ///
    /// Returns a Result that contains either the evaluated expression or a MathError
    /// when domain violations occur. This method checks mathematical constraints like:
    /// - sqrt(x) requires x >= 0 in real domain
    /// - log(x) requires x > 0 (pole at 0)
    /// - tan(x) has poles at π/2 + nπ
    /// - arcsin/arccos require |x| <= 1 in real domain
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MathError};
    ///
    /// // Valid sqrt
    /// let expr = Expression::function("sqrt".to_string(), vec![Expression::integer(4)]);
    /// assert!(expr.evaluate().is_ok());
    ///
    /// // Invalid sqrt in real domain
    /// let expr = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);
    /// assert!(matches!(expr.evaluate(), Err(MathError::DomainError { .. })));
    /// ```
    pub fn evaluate(&self) -> Result<Expression, crate::MathError> {
        use crate::MathError;
        use std::f64::consts::PI;

        match self {
            Expression::Function { name, args } => {
                // Check for undefined marker function (produced during simplification)
                if name == "undefined" {
                    return Err(MathError::DivisionByZero);
                }

                // First, recursively evaluate arguments
                let evaluated_args: Result<Vec<Expression>, MathError> =
                    args.iter().map(|arg| arg.evaluate()).collect();
                let evaluated_args = evaluated_args?;

                // Check domain restrictions for specific functions
                match name.as_str() {
                    "sqrt" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            // Check if argument is negative in real domain
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if value < 0.0 {
                                    return Err(MathError::DomainError {
                                        operation: "sqrt".to_string(),
                                        value: arg.clone(),
                                        reason: "sqrt requires non-negative input in real domain"
                                            .to_string(),
                                    });
                                }
                            }
                        }
                    }
                    "log" | "ln" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if value == 0.0 {
                                    return Err(MathError::Pole {
                                        function: name.clone(),
                                        at: arg.clone(),
                                    });
                                } else if value < 0.0 {
                                    return Err(MathError::BranchCut {
                                        function: name.clone(),
                                        value: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "tan" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                // Check for poles at π/2 + nπ
                                // tan is undefined when cos(x) = 0
                                let normalized = value.rem_euclid(PI);
                                if (normalized - PI / 2.0).abs() < 1e-10 {
                                    return Err(MathError::Pole {
                                        function: "tan".to_string(),
                                        at: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "arcsin" | "asin" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if value < -1.0 || value > 1.0 {
                                    return Err(MathError::DomainError {
                                        operation: "arcsin".to_string(),
                                        value: arg.clone(),
                                        reason: "arcsin requires input in [-1, 1] in real domain"
                                            .to_string(),
                                    });
                                }
                            }
                        }
                    }
                    "arccos" | "acos" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if value < -1.0 || value > 1.0 {
                                    return Err(MathError::DomainError {
                                        operation: "arccos".to_string(),
                                        value: arg.clone(),
                                        reason: "arccos requires input in [-1, 1] in real domain"
                                            .to_string(),
                                    });
                                }
                            }
                        }
                    }
                    "csc" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                // csc(x) = 1/sin(x), undefined when sin(x) = 0 (at nπ)
                                let normalized = value.rem_euclid(PI);
                                if normalized.abs() < 1e-10 {
                                    return Err(MathError::Pole {
                                        function: "csc".to_string(),
                                        at: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "sec" => {
                        if let Some(arg) = evaluated_args.get(0) {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                // sec(x) = 1/cos(x), undefined when cos(x) = 0 (at π/2 + nπ)
                                let normalized = value.rem_euclid(PI);
                                if (normalized - PI / 2.0).abs() < 1e-10 {
                                    return Err(MathError::Pole {
                                        function: "sec".to_string(),
                                        at: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }

                // If no domain error, simplify the function
                Ok(Expression::function(name.clone(), evaluated_args).simplify())
            }
            Expression::Pow(base, exp) => {
                let eval_base = base.evaluate()?;
                let eval_exp = exp.evaluate()?;

                // Check for division by zero: 0^(-n)
                if eval_base.is_zero_fast() {
                    if let Some(exp_value) = Self::try_extract_numeric_value(&eval_exp) {
                        if exp_value < 0.0 {
                            return Err(MathError::DivisionByZero);
                        }
                    }
                }

                Ok(Expression::pow(eval_base, eval_exp).simplify())
            }
            Expression::Mul(factors) => {
                let evaluated_factors: Result<Vec<Expression>, MathError> =
                    factors.iter().map(|f| f.evaluate()).collect();
                Ok(Expression::mul(evaluated_factors?).simplify())
            }
            Expression::Add(terms) => {
                let evaluated_terms: Result<Vec<Expression>, MathError> =
                    terms.iter().map(|t| t.evaluate()).collect();
                Ok(Expression::add(evaluated_terms?).simplify())
            }
            _ => Ok(self.simplify()),
        }
    }

    /// Helper to extract numeric value from an expression
    ///
    /// Returns Some(f64) if the expression is a Number, None otherwise
    fn try_extract_numeric_value(expr: &Expression) -> Option<f64> {
        match expr {
            Expression::Number(Number::Integer(i)) => Some(*i as f64),
            Expression::Number(Number::Float(f)) => Some(*f),
            Expression::Number(Number::Rational(r)) => {
                let num_float = r.numer().to_string().parse::<f64>().ok()?;
                let denom_float = r.denom().to_string().parse::<f64>().ok()?;
                Some(num_float / denom_float)
            }
            _ => None,
        }
    }
}

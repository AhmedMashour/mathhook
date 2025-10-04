//! Implicit multiplication post-processor
//!
//! Analyzes parsed AST and inserts implicit multiplication operators
//! between adjacent terms that should be multiplied.

use crate::core::Expression;

/// Post-processor for implicit multiplication
pub struct ImplicitMultiplicationProcessor {
    /// Enable different implicit multiplication patterns
    pub enable_number_variable: bool, // 2x -> 2*x
    pub enable_variable_variable: bool,    // xy -> x*y
    pub enable_number_function: bool,      // 2sin(x) -> 2*sin(x)
    pub enable_function_function: bool,    // sin(x)cos(y) -> sin(x)*cos(y)
    pub enable_number_parentheses: bool,   // 2(x+1) -> 2*(x+1)
    pub enable_parentheses_variable: bool, // (x+1)y -> (x+1)*y
}

impl Default for ImplicitMultiplicationProcessor {
    fn default() -> Self {
        Self {
            enable_number_variable: true,
            enable_variable_variable: true,
            enable_number_function: true,
            enable_function_function: true,
            enable_number_parentheses: true,
            enable_parentheses_variable: true,
        }
    }
}

impl ImplicitMultiplicationProcessor {
    /// Create a new processor with all patterns enabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a conservative processor (only obvious cases)
    pub fn conservative() -> Self {
        Self {
            enable_number_variable: true,
            enable_variable_variable: false, // More conservative
            enable_number_function: true,
            enable_function_function: false, // More conservative
            enable_number_parentheses: true,
            enable_parentheses_variable: false, // More conservative
        }
    }

    /// Process an expression to insert implicit multiplication
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, parser::implicit_multiplication::ImplicitMultiplicationProcessor};
    ///
    /// let processor = ImplicitMultiplicationProcessor::new();
    ///
    /// // This would work if you had a sequence like [2, x] parsed
    /// // let expr = Expression::parse("2 x").unwrap(); // Hypothetical
    /// // let result = processor.process(expr);
    /// // assert_eq!(result.to_string(), "2 * x");
    /// ```
    pub fn process(&self, expr: Expression) -> Expression {
        self.process_recursive(expr)
    }

    /// Recursive processing of expression tree
    fn process_recursive(&self, expr: Expression) -> Expression {
        match expr {
            // Process addition terms - look for implicit multiplication within terms
            Expression::Add(terms) => {
                let processed_terms: Vec<Expression> = terms
                    .iter()
                    .map(|term| self.process_recursive(term.clone()))
                    .collect();

                // Look for adjacent terms that should be implicitly multiplied
                let with_implicit_mul =
                    self.insert_implicit_multiplication_in_sequence(processed_terms);
                Expression::Add(Box::new(with_implicit_mul))
            }

            // Process multiplication factors
            Expression::Mul(factors) => {
                let processed_factors: Vec<Expression> = factors
                    .iter()
                    .map(|factor| self.process_recursive(factor.clone()))
                    .collect();
                Expression::Mul(Box::new(processed_factors))
            }

            // Process power base and exponent
            Expression::Pow(base, exp) => Expression::Pow(
                Box::new(self.process_recursive(*base)),
                Box::new(self.process_recursive(*exp)),
            ),

            // Process function arguments
            Expression::Function { name, args } => {
                let processed_args: Vec<Expression> = args
                    .iter()
                    .map(|arg| self.process_recursive(arg.clone()))
                    .collect();
                Expression::Function {
                    name,
                    args: Box::new(processed_args),
                }
            }

            // Process set elements
            Expression::Set(elements) => {
                let processed_elements: Vec<Expression> = elements
                    .iter()
                    .map(|elem| self.process_recursive(elem.clone()))
                    .collect();
                Expression::Set(Box::new(processed_elements))
            }

            // Process complex numbers
            Expression::Complex(data) => {
                Expression::Complex(Box::new(crate::core::expression::ComplexData {
                    real: self.process_recursive(data.real.clone()),
                    imag: self.process_recursive(data.imag.clone()),
                }))
            }

            // Process relations
            Expression::Relation(data) => {
                Expression::Relation(Box::new(crate::core::expression::RelationData {
                    left: self.process_recursive(data.left.clone()),
                    right: self.process_recursive(data.right.clone()),
                    relation_type: data.relation_type,
                }))
            }

            // Process intervals
            Expression::Interval(data) => {
                Expression::Interval(Box::new(crate::core::expression::IntervalData {
                    start: self.process_recursive(data.start.clone()),
                    end: self.process_recursive(data.end.clone()),
                    start_inclusive: data.start_inclusive,
                    end_inclusive: data.end_inclusive,
                }))
            }

            // Base cases - no further processing needed
            Expression::Number(_)
            | Expression::Symbol(_)
            | Expression::Constant(_)
            | Expression::Matrix(_)
            | Expression::Piecewise(_)
            | Expression::Calculus(_) => expr,
        }
    }

    /// Insert implicit multiplication between adjacent terms in a sequence
    fn insert_implicit_multiplication_in_sequence(
        &self,
        terms: Vec<Expression>,
    ) -> Vec<Expression> {
        if terms.len() <= 1 {
            return terms;
        }

        let mut result = Vec::new();
        result.push(terms[0].clone());

        for i in 1..terms.len() {
            let prev = &terms[i - 1];
            let curr = &terms[i];

            // Check if we should insert implicit multiplication
            if self.should_insert_implicit_multiplication(prev, curr) {
                // Convert the sequence into a multiplication
                let last = result.pop().unwrap();
                result.push(Expression::Mul(Box::new(vec![last, curr.clone()])));
            } else {
                result.push(curr.clone());
            }
        }

        result
    }

    /// Determine if implicit multiplication should be inserted between two expressions
    pub fn should_insert_implicit_multiplication(
        &self,
        left: &Expression,
        right: &Expression,
    ) -> bool {
        match (left, right) {
            // 2x -> 2*x
            (Expression::Number(_), Expression::Symbol(_)) => self.enable_number_variable,

            // xy -> x*y
            (Expression::Symbol(_), Expression::Symbol(_)) => self.enable_variable_variable,

            // 2sin(x) -> 2*sin(x)
            (Expression::Number(_), Expression::Function { .. }) => self.enable_number_function,

            // sin(x)cos(y) -> sin(x)*cos(y)
            (Expression::Function { .. }, Expression::Function { .. }) => {
                self.enable_function_function
            }

            // 2(x+1) -> 2*(x+1)
            (Expression::Number(_), Expression::Add(_)) => self.enable_number_parentheses,

            // (x+1)y -> (x+1)*y
            (Expression::Add(_), Expression::Symbol(_)) => self.enable_parentheses_variable,

            // x^2 y -> (x^2)*y
            (Expression::Pow(_, _), Expression::Symbol(_)) => self.enable_variable_variable,

            // 2x^2 -> 2*(x^2) - but this should be parsed as 2*(x^2) already
            (Expression::Number(_), Expression::Pow(_, _)) => self.enable_number_variable,

            // sin(x)^2 cos(y) -> (sin(x)^2)*cos(y)
            (Expression::Pow(base, _), Expression::Function { .. })
                if matches!(**base, Expression::Function { .. }) =>
            {
                self.enable_function_function
            }

            // More complex cases...
            _ => false,
        }
    }
}

/// Extension trait to add implicit multiplication processing to Expression
pub trait ImplicitMultiplicationExt {
    /// Process this expression to insert implicit multiplication
    fn with_implicit_multiplication(self) -> Self;

    /// Process with custom processor settings
    fn with_implicit_multiplication_custom(
        self,
        processor: &ImplicitMultiplicationProcessor,
    ) -> Self;
}

impl ImplicitMultiplicationExt for Expression {
    fn with_implicit_multiplication(self) -> Self {
        let processor = ImplicitMultiplicationProcessor::new();
        processor.process(self)
    }

    fn with_implicit_multiplication_custom(
        self,
        processor: &ImplicitMultiplicationProcessor,
    ) -> Self {
        processor.process(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_variable_multiplication() {
        let processor = ImplicitMultiplicationProcessor::new();

        // Create a hypothetical sequence [2, x] that would need implicit multiplication
        // This would come from your parser detecting adjacent terms
        let two = Expression::integer(2);
        let x = Expression::Symbol(crate::core::Symbol::new("x"));

        // Test the logic
        assert!(processor.should_insert_implicit_multiplication(&two, &x));
    }

    #[test]
    fn test_function_function_multiplication() {
        let processor = ImplicitMultiplicationProcessor::new();

        let sin_x = Expression::Function {
            name: "sin".to_string(),
            args: Box::new(vec![Expression::Symbol(crate::core::Symbol::new("x"))]),
        };
        let cos_y = Expression::Function {
            name: "cos".to_string(),
            args: Box::new(vec![Expression::Symbol(crate::core::Symbol::new("y"))]),
        };

        assert!(processor.should_insert_implicit_multiplication(&sin_x, &cos_y));
    }

    #[test]
    fn test_conservative_mode() {
        let processor = ImplicitMultiplicationProcessor::conservative();

        let x = Expression::Symbol(crate::core::Symbol::new("x"));
        let y = Expression::Symbol(crate::core::Symbol::new("y"));

        // Conservative mode should NOT enable variable*variable
        assert!(!processor.should_insert_implicit_multiplication(&x, &y));

        // But should still enable number*variable
        let two = Expression::integer(2);
        assert!(processor.should_insert_implicit_multiplication(&two, &x));
    }
}

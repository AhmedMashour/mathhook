//! Sequence detection for implicit multiplication
//!
//! This module provides utilities to detect sequences of adjacent terms
//! during parsing that should be implicitly multiplied.

use crate::core::Expression;
use crate::parser::implicit_multiplication::ImplicitMultiplicationProcessor;

/// A sequence of adjacent mathematical terms
#[derive(Debug, Clone)]
pub struct TermSequence {
    pub terms: Vec<Expression>,
    pub positions: Vec<usize>, // Original positions in input for error reporting
}

impl TermSequence {
    /// Create a new term sequence
    pub fn new() -> Self {
        Self {
            terms: Vec::new(),
            positions: Vec::new(),
        }
    }

    /// Add a term to the sequence
    pub fn push(&mut self, term: Expression, position: usize) {
        self.terms.push(term);
        self.positions.push(position);
    }

    /// Convert sequence to expression with implicit multiplication
    pub fn to_expression_with_implicit_mul(
        self,
        processor: &ImplicitMultiplicationProcessor,
    ) -> Expression {
        if self.terms.is_empty() {
            return Expression::integer(0);
        }

        if self.terms.len() == 1 {
            return self.terms.into_iter().next().unwrap();
        }

        // Process the sequence to insert implicit multiplication
        self.insert_implicit_multiplication(processor)
    }

    /// Insert implicit multiplication between adjacent terms
    fn insert_implicit_multiplication(
        self,
        processor: &ImplicitMultiplicationProcessor,
    ) -> Expression {
        let mut result_terms = Vec::new();

        for (i, term) in self.terms.into_iter().enumerate() {
            if i == 0 {
                result_terms.push(term);
            } else {
                let prev = &result_terms[result_terms.len() - 1];

                if processor.should_insert_implicit_multiplication(prev, &term) {
                    // Create multiplication of previous term and current term
                    let prev_term = result_terms.pop().unwrap();
                    result_terms.push(Expression::Mul(Box::new(vec![prev_term, term])));
                } else {
                    result_terms.push(term);
                }
            }
        }

        // If we have multiple terms left, they form an addition
        if result_terms.len() == 1 {
            result_terms.into_iter().next().unwrap()
        } else {
            Expression::Add(Box::new(result_terms))
        }
    }
}

/// Helper trait for building sequences during parsing
pub trait SequenceBuilder {
    /// Start building a new sequence
    fn start_sequence() -> TermSequence {
        TermSequence::new()
    }

    /// Add term to sequence if it should be implicitly multiplied
    fn maybe_add_to_sequence(sequence: &mut TermSequence, term: Expression, position: usize) {
        sequence.push(term, position);
    }
}

impl SequenceBuilder for Expression {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_number_variable_sequence() {
        let mut seq = TermSequence::new();
        seq.push(Expression::integer(2), 0);
        seq.push(Expression::Symbol(Symbol::new("x")), 1);

        let processor = ImplicitMultiplicationProcessor::new();
        let result = seq.to_expression_with_implicit_mul(&processor);

        // Should create 2*x
        match result {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert!(matches!(factors[0], Expression::Number(_)));
                assert!(matches!(factors[1], Expression::Symbol(_)));
            }
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_function_sequence() {
        let mut seq = TermSequence::new();

        let sin_x = Expression::Function {
            name: "sin".to_string(),
            args: Box::new(vec![Expression::Symbol(Symbol::new("x"))]),
        };
        let cos_y = Expression::Function {
            name: "cos".to_string(),
            args: Box::new(vec![Expression::Symbol(Symbol::new("y"))]),
        };

        seq.push(sin_x, 0);
        seq.push(cos_y, 1);

        let processor = ImplicitMultiplicationProcessor::new();
        let result = seq.to_expression_with_implicit_mul(&processor);

        // Should create sin(x)*cos(y)
        match result {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert!(matches!(factors[0], Expression::Function { .. }));
                assert!(matches!(factors[1], Expression::Function { .. }));
            }
            _ => panic!("Expected multiplication"),
        }
    }
}

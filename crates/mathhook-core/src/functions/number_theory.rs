//! Number Theory Function Intelligence
//!
//! Complete mathematical intelligence for number theory functions:
//! gcd, lcm, mod, prime operations with existing algorithm integration.

use crate::core::Expression;
use crate::functions::properties::*;
use crate::symbol;
use std::collections::HashMap;

/// Number Theory Function Intelligence
///
/// Dedicated intelligence system for number theory functions
/// with integration to existing mathematical algorithms.
pub struct NumberTheoryIntelligence {
    /// Function properties for each number theory function
    properties: HashMap<String, FunctionProperties>,
}

impl NumberTheoryIntelligence {
    /// Create new number theory intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(8),
        };

        intelligence.initialize_gcd_lcm();
        intelligence.initialize_modular_arithmetic();
        intelligence.initialize_prime_functions();

        intelligence
    }

    /// Get all number theory function properties
    pub fn get_all_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function has number theory intelligence
    pub fn has_intelligence(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    fn initialize_gcd_lcm(&mut self) {
        self.properties.insert(
            "gcd".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 2 {
                        args[0].gcd(&args[1])
                    } else {
                        Expression::function("gcd", args.to_vec())
                    }
                },

                derivative_rule: None, // GCD is not differentiable
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "0".to_string(),
                    output: Expression::symbol(symbol!(b)),
                    latex_explanation: "\\gcd(0, b) = |b|".to_string(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Integer,
                    range: Range::PositiveInteger,
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: None, // Uses existing symbolic implementation
            })),
        );

        // LCM function - integrates with existing Expression::lcm implementation
        self.properties.insert(
            "lcm".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 2 {
                        args[0].lcm(&args[1])
                    } else {
                        Expression::function("lcm", args.to_vec())
                    }
                },

                derivative_rule: None, // LCM is not differentiable
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "1".to_string(),
                    output: Expression::symbol(symbol!(b)),
                    latex_explanation: "\\text{lcm}(1, b) = |b|".to_string(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Integer,
                    range: Range::PositiveInteger,
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: None, // Uses existing symbolic implementation
            })),
        );
    }

    /// Initialize modular arithmetic functions
    fn initialize_modular_arithmetic(&mut self) {
        // MOD function for modular reduction
        self.properties.insert(
            "mod".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| Expression::function("mod", args.to_vec()),
                derivative_rule: None,
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "0".to_string(),
                    output: Expression::integer(0),
                    latex_explanation: "a \\bmod m = 0 \\text{ when } a = 0".to_string(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Integer,
                    range: Range::Integer,
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: None, // Uses existing modular arithmetic
            })),
        );
    }

    /// Initialize prime-related functions
    fn initialize_prime_functions(&mut self) {
        // IS_PRIME function
        self.properties.insert(
            "is_prime".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| Expression::function("is_prime", args.to_vec()),
                derivative_rule: None,
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "2".to_string(),
                    output: Expression::integer(1), // Use 1 for true, 0 for false
                    latex_explanation: "2 \\text{ is prime}".to_string(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::PositiveInteger,
                    range: Range::Integer,
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: None, // Uses existing prime testing algorithms
            })),
        );
    }
}

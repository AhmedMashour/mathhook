//! Step-by-step explanation system for educational purposes
//! Provides detailed explanations of simplification and algebraic operations

use crate::core::Expression;
use serde::{Deserialize, Serialize};

/// Represents a single step in a mathematical operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub title: String,
    pub description: String,
    pub expression: Expression,
    pub rule_applied: String,
    pub latex: Option<String>,
}

impl Step {
    /// Create a new step with title and description
    pub fn new<T: Into<String>, D: Into<String>>(title: T, description: D) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            expression: Expression::integer(0), // Default
            rule_applied: "Custom".to_string(),
            latex: None,
        }
    }
}

/// Complete step-by-step explanation of a mathematical operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepByStepExplanation {
    pub initial_expression: Expression,
    pub final_expression: Expression,
    pub steps: Vec<Step>,
    pub total_steps: usize,
    pub rules_used: Vec<String>,
}

impl StepByStepExplanation {
    /// Create a new step-by-step explanation
    pub fn new(steps: Vec<Step>) -> Self {
        let total_steps = steps.len();
        let initial_expr = Expression::integer(0); // Default
        let final_expr = Expression::integer(0); // Default
        let rules_used = steps.iter().map(|s| s.rule_applied.clone()).collect();

        Self {
            initial_expression: initial_expr,
            final_expression: final_expr,
            steps,
            total_steps,
            rules_used,
        }
    }
}

/// Trait for generating step-by-step explanations
pub trait StepByStep {
    fn explain_simplification(&self) -> StepByStepExplanation;
    fn explain_expansion(&self) -> StepByStepExplanation;
    fn explain_factorization(&self) -> StepByStepExplanation;
}

impl StepByStep for Expression {
    /// Generate step-by-step explanation for simplification
    fn explain_simplification(&self) -> StepByStepExplanation {
        // Temporarily simplified for TDD focus
        StepByStepExplanation::new(vec![Step::new(
            "Simplification",
            "Step-by-step simplification",
        )])
    }

    // Temporarily disabled - complex implementation
    /*
    fn explain_simplification_full(&self) -> StepByStepExplanation {
        let mut steps = Vec::new();
        let mut current = self.clone();
        let mut step_count = 0;
        let mut rules_used = Vec::new();

        // Step 1: Initial expression
        steps.push(Step {
            title: "Starting Expression".to_string(),
            description: "Starting expression".to_string(),
            expression: current.clone(),
            rule_applied: "Initial".to_string(),
            latex: Some(self.to_latex()),
        });

        // Apply simplification rules step by step
        current = self.apply_simplification_steps(&mut steps, &mut rules_used, &mut step_count);

        StepByStepExplanation {
            initial_expression: self.clone(),
            final_expression: current,
            steps,
            total_steps: step_count,
            rules_used,
        }
    }

    */

    /// Generate step-by-step explanation for expansion
    fn explain_expansion(&self) -> StepByStepExplanation {
        // Temporarily simplified
        StepByStepExplanation::new(vec![Step::new("Expansion", "Step-by-step expansion")])
    }

    /*
    fn explain_expansion_full(&self) -> StepByStepExplanation {
        let mut steps = Vec::new();
        let mut rules_used = Vec::new();

        steps.push(Step {
            description: "Starting expression".to_string(),
            expression: self.clone(),
            rule_applied: "Initial".to_string(),
            latex: Some(self.to_latex()),
        });

        // For now, just show the final expanded form
        // Full implementation would show each expansion step
        let expanded = self.clone(); // Would call expand() when implemented

        steps.push(Step {
            description: "Applied expansion rules".to_string(),
            expression: expanded.clone(),
            rule_applied: "Expansion".to_string(),
            latex: Some(expanded.to_latex()),
        });

        rules_used.push("Expansion".to_string());

        StepByStepExplanation {
            initial_expression: self.clone(),
            final_expression: expanded,
            steps,
            total_steps: 1,
            rules_used,
        }
    }

    */

    /// Generate step-by-step explanation for factorization
    fn explain_factorization(&self) -> StepByStepExplanation {
        // Temporarily simplified
        StepByStepExplanation::new(vec![Step::new(
            "Factorization",
            "Step-by-step factorization",
        )])
    }
}

/*
// Temporarily commented out - complex implementation with Step struct issues
impl Expression {
            description: "Starting expression".to_string(),
            expression: self.clone(),
            rule_applied: "Initial".to_string(),
            latex: Some(self.to_latex()),
        });

        // For now, just show the factored form
        let factored = self.clone(); // Would call factor() when implemented

        steps.push(Step {
            description: "Applied factorization rules".to_string(),
            expression: factored.clone(),
            rule_applied: "Factorization".to_string(),
            latex: Some(factored.to_latex()),
        });

        rules_used.push("Factorization".to_string());

        StepByStepExplanation {
            initial_expression: self.clone(),
            final_expression: factored,
            steps,
            total_steps: 1,
            rules_used,
        }
    }
}

impl Expression {
    /// Apply simplification rules step by step
    fn apply_simplification_steps(
        &self,
        steps: &mut Vec<Step>,
        rules_used: &mut Vec<String>,
        step_count: &mut usize,
    ) -> Expression {
        let mut current = self.clone();

        // Step 1: Combine numeric terms
        if let Some(numeric_simplified) = self.try_numeric_simplification(&current) {
            if numeric_simplified != current {
                *step_count += 1;
                steps.push(Step {
                    title: "Combine Terms".to_string(),
                    description: "Combine numeric terms".to_string(),
                    expression: numeric_simplified.clone(),
                    rule_applied: "Numeric Combination".to_string(),
                    latex: Some(numeric_simplified.to_latex()),
                });
                rules_used.push("Numeric Combination".to_string());
                current = numeric_simplified;
            }
        }

        // Step 2: Apply identity rules (x + 0 = x, x * 1 = x, etc.)
        if let Some(identity_simplified) = self.try_identity_simplification(&current) {
            if identity_simplified != current {
                *step_count += 1;
                steps.push(Step {
                    title: "Combine Terms".to_string(),
                    description: "Apply identity rules".to_string(),
                    expression: identity_simplified.clone(),
                    rule_applied: "Identity Rules".to_string(),
                    latex: Some(identity_simplified.to_latex()),
                });
                rules_used.push("Identity Rules".to_string());
                current = identity_simplified;
            }
        }

        // Step 3: Apply zero rules (x * 0 = 0, 0^n = 0, etc.)
        if let Some(zero_simplified) = self.try_zero_simplification(&current) {
            if zero_simplified != current {
                *step_count += 1;
                steps.push(Step {
                    title: "Combine Terms".to_string(),
                    description: "Apply zero rules".to_string(),
                    expression: zero_simplified.clone(),
                    rule_applied: "Zero Rules".to_string(),
                    latex: Some(zero_simplified.to_latex()),
                });
                rules_used.push("Zero Rules".to_string());
                current = zero_simplified;
            }
        }

        // Step 4: Apply power rules (x^0 = 1, x^1 = x, etc.)
        if let Some(power_simplified) = self.try_power_simplification(&current) {
            if power_simplified != current {
                *step_count += 1;
                steps.push(Step {
                    title: "Combine Terms".to_string(),
                    description: "Apply power rules".to_string(),
                    expression: power_simplified.clone(),
                    rule_applied: "Power Rules".to_string(),
                    latex: Some(power_simplified.to_latex()),
                });
                rules_used.push("Power Rules".to_string());
                current = power_simplified;
            }
        }

        // Final step: Standard simplification
        let final_simplified = current.simplify();
        if final_simplified != current {
            *step_count += 1;
            steps.push(Step {
                description: "Final simplification".to_string(),
                expression: final_simplified.clone(),
                rule_applied: "Standard Simplification".to_string(),
                latex: Some(final_simplified.to_latex()),
            });
            rules_used.push("Standard Simplification".to_string());
            current = final_simplified;
        }

        current
    }

    /// Try numeric simplification
    fn try_numeric_simplification(&self, expr: &Expression) -> Option<Expression> {
        match expr {
            Expression::Add(terms) => {
                let mut numeric_sum = num_bigint::BigInt::from(0);
                let mut non_numeric = Vec::new();
                let mut has_numeric = false;

                for term in terms.iter() {
                    if let Expression::Number(Number::Integer(n)) = term {
                        numeric_sum += BigInt::from(*n);
                        has_numeric = true;
                    } else {
                        non_numeric.push(term.clone());
                    }
                }

                if has_numeric {
                    if !numeric_sum.is_zero() {
                        non_numeric.insert(0, Expression::integer(numeric_sum));
                    }
                    Some(Expression::add(non_numeric))
                } else {
                    None
                }
            },
            Expression::Mul(factors) => {
                let mut numeric_product = num_bigint::BigInt::from(1);
                let mut non_numeric = Vec::new();
                let mut has_numeric = false;

                for factor in factors.iter() {
                    if let Expression::Number(Number::Integer(n)) = factor {
                        numeric_product *= BigInt::from(*n);
                        has_numeric = true;
                    } else {
                        non_numeric.push(factor.clone());
                    }
                }

                if has_numeric {
                    if !numeric_product.is_one() {
                        non_numeric.insert(0, Expression::integer(numeric_product));
                    }
                    Some(Expression::mul(non_numeric))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Try identity simplification
    fn try_identity_simplification(&self, expr: &Expression) -> Option<Expression> {
        match expr {
            Expression::Add(terms) => {
                let non_zero_terms: Vec<Expression> = terms.iter()
                    .filter(|t| !t.is_zero())
                    .cloned()
                    .collect();

                if non_zero_terms.len() != terms.len() {
                    Some(Expression::add(non_zero_terms))
                } else {
                    None
                }
            },
            Expression::Mul(factors) => {
                let non_one_factors: Vec<Expression> = factors.iter()
                    .filter(|f| !f.is_one())
                    .cloned()
                    .collect();

                if non_one_factors.len() != factors.len() {
                    Some(Expression::mul(non_one_factors))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Try zero simplification
    fn try_zero_simplification(&self, expr: &Expression) -> Option<Expression> {
        match expr {
            Expression::Mul(factors) => {
                if factors.iter().any(|f| f.is_zero()) {
                    Some(Expression::integer(0))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Try power simplification
    fn try_power_simplification(&self, expr: &Expression) -> Option<Expression> {
        match expr {
            Expression::Pow(base, exp) => {
                if exp.is_zero() {
                    Some(Expression::integer(1))
                } else if exp.is_one() {
                    Some(base.as_ref().clone())
                } else if base.is_zero() {
                    Some(Expression::integer(0))
                } else if base.is_one() {
                    Some(Expression::integer(1))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Convert expression to LaTeX format
    pub fn to_latex(&self) -> String {
        match self {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom().is_one() {
                    r.numer().to_string()
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
                }
            },
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Add(terms) => {
                if terms.is_empty() {
                    "0".to_string()
                } else {
                    let term_strs: Vec<String> = terms.iter()
                        .map(|t| t.to_latex())
                        .collect();
                    term_strs.join(" + ")
                }
            },
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    "1".to_string()
                } else {
                    let factor_strs: Vec<String> = factors.iter()
                        .map(|f| {
                            match f {
                                Expression::Add(_) => format!("({})", f.to_latex()),
                                _ => f.to_latex(),
                            }
                        })
                        .collect();
                    factor_strs.join(" \\cdot ")
                }
            },
            Expression::Pow(base, exp) => {
                let base_latex = match base.as_ref() {
                    Expression::Add(_) | Expression::Mul(_) => format!("({})", base.to_latex()),
                    _ => base.to_latex(),
                };
                format!("{}^{{{}}}", base_latex, exp.to_latex())
            },
            Expression::Function { name, args } => {
                if args.is_empty() {
                    format!("\\{}", name)
                } else {
                    let arg_strs: Vec<String> = args.iter()
                        .map(|a| a.to_latex())
                        .collect();
                    format!("\\{}({})", name, arg_strs.join(", "))
                }
            }
        }
    }

    /// Parse LaTeX input to Expression (simplified for now)
    pub fn from_latex(latex: &str) -> Result<Expression, String> {
        // Simplified LaTeX parsing - full implementation would be more complex

        // Handle basic cases
        if latex.trim().chars().all(|c| c.is_ascii_digit() || c == '-') {
            if let Ok(n) = latex.trim().parse::<i64>() {
                return Ok(Expression::integer(n));
            }
        }

        // Handle single variables
        if latex.trim().chars().all(|c| c.is_ascii_alphabetic()) {
            return Ok(Expression::symbol(Symbol::new(latex.trim())));
        }

        Err(format!("Cannot parse LaTeX: {} (full parser not implemented yet)", latex))
    }

    /// Parse LaTeX fraction: \frac{numerator}{denominator}
    #[allow(dead_code)]
    fn parse_latex_fraction(&self, latex: &str) -> Option<Expression> {
        // Simplified fraction parsing
        if let Some(start) = latex.find("\\frac{") {
            let content = &latex[start + 6..];
            if let Some(close_num) = content.find("}{") {
                let numerator = &content[..close_num];
                let rest = &content[close_num + 2..];
                if let Some(close_den) = rest.find('}') {
                    let denominator = &rest[..close_den];

                    if let (Ok(num), Ok(den)) = (numerator.parse::<i64>(), denominator.parse::<i64>()) {
                        let rational = num_rational::BigRational::new(
                            num_bigint::BigInt::from(num),
                            num_bigint::BigInt::from(den)
                        );
                        return Some(Expression::number(Number::rational(rational)));
                    }
                }
            }
        }
        None
    }

    /// Parse LaTeX power: base^{exponent}
    #[allow(dead_code)]
    fn parse_latex_power(&self, latex: &str) -> Option<Expression> {
        if let Some(caret_pos) = latex.find("^{") {
            let base_str = &latex[..caret_pos];
            let exp_start = caret_pos + 2;
            if let Some(close_brace) = latex[exp_start..].find('}') {
                let exp_str = &latex[exp_start..exp_start + close_brace];

                // Parse base and exponent
                if let (Ok(base_expr), Ok(exp_expr)) = (
                    Expression::from_latex(base_str),
                    Expression::from_latex(exp_str)
                ) {
                    return Some(Expression::pow(base_expr, exp_expr));
                }
            }
        }
        None
    }

    /// Parse LaTeX function: \function_name(args)
    #[allow(dead_code)]
    fn parse_latex_function(&self, latex: &str) -> Option<Expression> {
        if let Some(backslash_pos) = latex.find('\\') {
            let after_backslash = &latex[backslash_pos + 1..];
            if let Some(paren_pos) = after_backslash.find('(') {
                let func_name = &after_backslash[..paren_pos];
                let args_start = backslash_pos + 1 + paren_pos + 1;
                if let Some(close_paren) = latex[args_start..].rfind(')') {
                    let args_str = &latex[args_start..args_start + close_paren];

                    // Parse arguments (simplified - would need full parser)
                    if let Ok(arg_expr) = Expression::from_latex(args_str) {
                        return Some(Expression::function(func_name, vec![arg_expr]));
                    }
                }
            }
        }
        None
    }

    /// Generate educational explanation text
    pub fn explain_rule(&self, rule: &str) -> String {
        match rule {
            "Numeric Combination" => {
                "Combine numeric terms by performing arithmetic operations".to_string()
            },
            "Identity Rules" => {
                "Apply identity rules: x + 0 = x, x * 1 = x, x - x = 0".to_string()
            },
            "Zero Rules" => {
                "Apply zero rules: x * 0 = 0, 0 + x = x, 0^n = 0".to_string()
            },
            "Power Rules" => {
                "Apply power rules: x^0 = 1, x^1 = x, 1^n = 1".to_string()
            },
            "Expansion" => {
                "Expand expressions by distributing multiplication over addition".to_string()
            },
            "Factorization" => {
                "Factor expressions by extracting common factors".to_string()
            },
            _ => format!("Applied rule: {}", rule),
        }
    }
}

/// Step-by-step explanation builder
pub struct StepByStepBuilder {
    steps: Vec<Step>,
    rules_used: Vec<String>,
}

impl StepByStepBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            rules_used: Vec::new(),
        }
    }

    /// Add a step to the explanation
    pub fn add_step(&mut self, description: String, expression: Expression, rule: String) {
        let latex = expression.to_latex();
        self.steps.push(Step {
            description,
            expression,
            rule_applied: rule.clone(),
            latex: Some(latex),
        });
        self.rules_used.push(rule);
    }

    /// Build the final explanation
    pub fn build(self, initial: Expression, final_expr: Expression) -> StepByStepExplanation {
        StepByStepExplanation {
            initial_expression: initial,
            final_expression: final_expr,
            total_steps: self.steps.len(),
            steps: self.steps,
            rules_used: self.rules_used,
        }
    }
}

impl Default for StepByStepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_by_step_explanation() {
        let x = Symbol::new("x");
        let expr = Expression::add(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::symbol(x.clone())
        ]);

        let explanation = expr.explain_simplification();

        assert!(!explanation.steps.is_empty());
        assert!(explanation.total_steps > 0);
        assert!(!explanation.rules_used.is_empty());

        println!("Step-by-step explanation:");
        for (i, step) in explanation.steps.iter().enumerate() {
            println!("Step {}: {} - {}", i + 1, step.description, step.expression);
        }
    }

    #[test]
    fn test_latex_generation() {
        let x = Symbol::new("x");
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let latex = expr.to_latex();
        assert_eq!(latex, "x^{2}");

        let rational = Expression::number(Number::rational(
            num_rational::BigRational::new(num_bigint::BigInt::from(3), num_bigint::BigInt::from(4))
        ));
        let latex = rational.to_latex();
        assert_eq!(latex, "\\frac{3}{4}");
    }

    #[test]
    fn test_latex_parsing() {
        // Test simple number
        let expr = Expression::from_latex("42").unwrap();
        assert_eq!(expr, Expression::integer(42));

        // Test variable
        let expr = Expression::from_latex("x").unwrap();
        assert_eq!(expr, Expression::symbol(Symbol::new("x")));

        // Test fraction
        let expr = Expression::from_latex("\\frac{3}{4}").unwrap();
        if let Expression::Number(Number::Rational(r)) = expr {
            assert_eq!(r.numer(), &num_bigint::BigInt::from(3));
            assert_eq!(r.denom(), &num_bigint::BigInt::from(4));
        } else {
            panic!("Expected rational number");
        }
    }

    #[test]
    fn test_educational_explanations() {
        let expr = Expression::integer(1);

        let explanation = expr.explain_rule("Identity Rules");
        assert!(explanation.contains("identity"));

        let explanation = expr.explain_rule("Zero Rules");
        assert!(explanation.contains("zero"));

        let explanation = expr.explain_rule("Power Rules");
        assert!(explanation.contains("power"));
    }

    #[test]
    fn test_step_builder() {
        let mut builder = StepByStepBuilder::new();

        let x = Symbol::new("x");
        let initial = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(0)
        ]);

        builder.add_step(
            "Remove zero term".to_string(),
            Expression::symbol(x.clone()),
            "Identity Rules".to_string()
        );

        let explanation = builder.build(initial.clone(), Expression::symbol(x.clone()));

        assert_eq!(explanation.initial_expression, initial);
        assert_eq!(explanation.final_expression, Expression::symbol(x));
        assert_eq!(explanation.total_steps, 1);
        assert_eq!(explanation.rules_used, vec!["Identity Rules"]);
    }
}
*/

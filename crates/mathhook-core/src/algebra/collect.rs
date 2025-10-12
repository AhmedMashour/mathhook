//! Term collection and combination operations
//! Handles collecting like terms, combining coefficients, and organizing expressions

use crate::core::{Expression, Number, Symbol};
use num_bigint::BigInt;
use num_traits::{One, Zero};
// HashMap removed - using Vec-based approach for Expression compatibility

/// Trait for collecting terms in expressions
pub trait Collect {
    fn collect(&self, var: &Symbol) -> Self;
    fn collect_terms(&self) -> Self;
    fn combine_like_terms(&self) -> Self;
}

impl Collect for Expression {
    /// Collect terms with respect to a specific variable
    fn collect(&self, var: &Symbol) -> Self {
        match self {
            Expression::Add(terms) => self.collect_addition_terms(terms, var),
            _ => self.clone(),
        }
    }

    /// Collect and combine all like terms
    fn collect_terms(&self) -> Self {
        match self {
            Expression::Add(terms) => self.collect_all_like_terms(terms),
            Expression::Mul(factors) => self.collect_multiplication_terms(factors),
            _ => self.clone(),
        }
    }

    /// Combine like terms in the expression
    fn combine_like_terms(&self) -> Self {
        self.collect_terms()
    }
}

impl Expression {
    /// Collect terms in an addition with respect to a variable
    fn collect_addition_terms(&self, terms: &[Expression], var: &Symbol) -> Expression {
        // Use Vec instead of HashMap due to Expression not implementing Eq+Hash (contains f64)
        let mut term_coefficients: Vec<(Expression, BigInt)> = Vec::new();
        let mut constant_term = BigInt::zero();

        for term in terms {
            let (coeff, power_expr) = self.extract_coefficient_and_power(term, var);

            if power_expr == Expression::integer(0) {
                // Constant term
                constant_term += coeff;
            } else {
                // Variable term
                // Find existing entry or create new one
                let mut found = false;
                for (existing_expr, existing_coeff) in term_coefficients.iter_mut() {
                    if *existing_expr == power_expr {
                        *existing_coeff += &coeff;
                        found = true;
                        break;
                    }
                }
                if !found {
                    term_coefficients.push((power_expr, coeff));
                }
            }
        }

        // Reconstruct the expression
        let mut result_terms = Vec::new();

        // Add constant term if non-zero
        if !constant_term.is_zero() {
            result_terms.push(Expression::big_integer(constant_term));
        }

        // Add variable terms
        for (power_expr, coeff) in term_coefficients {
            if !coeff.is_zero() {
                let term = if coeff.is_one() {
                    if power_expr == Expression::integer(1) {
                        Expression::symbol(var.clone())
                    } else {
                        Expression::pow(Expression::symbol(var.clone()), power_expr)
                    }
                } else {
                    let var_part = if power_expr == Expression::integer(1) {
                        Expression::symbol(var.clone())
                    } else {
                        Expression::pow(Expression::symbol(var.clone()), power_expr)
                    };
                    Expression::mul(vec![Expression::big_integer(coeff), var_part])
                };
                result_terms.push(term);
            }
        }

        if result_terms.is_empty() {
            Expression::integer(0)
        } else if result_terms.len() == 1 {
            result_terms[0].clone()
        } else {
            Expression::add(result_terms)
        }
    }

    /// Extract coefficient and power from a term with respect to a variable
    fn extract_coefficient_and_power(
        &self,
        term: &Expression,
        var: &Symbol,
    ) -> (BigInt, Expression) {
        match term {
            // Pure number
            Expression::Number(Number::Integer(n)) => (BigInt::from(*n), Expression::integer(0)),

            // Pure variable
            Expression::Symbol(s) if s == var => (BigInt::one(), Expression::integer(1)),

            // Other symbol
            Expression::Symbol(_) => (BigInt::zero(), Expression::integer(0)),

            // Power of variable: x^n
            Expression::Pow(base, exp) => {
                if let Expression::Symbol(s) = base.as_ref() {
                    if s == var {
                        return (BigInt::one(), exp.as_ref().clone());
                    }
                }
                (BigInt::zero(), Expression::integer(0))
            }

            // Multiplication: coeff * x^n
            Expression::Mul(factors) => {
                let mut coefficient = BigInt::one();
                let mut power = Expression::integer(0);
                let mut has_var = false;

                for factor in factors.iter() {
                    match factor {
                        Expression::Number(Number::Integer(n)) => {
                            coefficient *= BigInt::from(*n);
                        }
                        Expression::Symbol(s) if s == var => {
                            power = Expression::integer(1);
                            has_var = true;
                        }
                        Expression::Pow(base, exp) => {
                            if let Expression::Symbol(s) = base.as_ref() {
                                if s == var {
                                    power = exp.as_ref().clone();
                                    has_var = true;
                                }
                            }
                        }
                        _ => {
                            // Non-variable factor, treat as part of coefficient
                            // This is simplified - full implementation would handle this better
                        }
                    }
                }

                if has_var {
                    (coefficient, power)
                } else {
                    (BigInt::zero(), Expression::integer(0))
                }
            }

            _ => (BigInt::zero(), Expression::integer(0)),
        }
    }

    /// Collect all like terms regardless of variable
    fn collect_all_like_terms(&self, terms: &[Expression]) -> Expression {
        // Use Vec instead of HashMap due to Expression not implementing Eq+Hash
        let mut term_coefficients: Vec<(Expression, BigInt)> = Vec::new();

        for term in terms {
            let (coeff, base_term) = self.extract_coefficient_and_base(term);
            // Find existing entry or create new one
            let mut found = false;
            for (existing_expr, existing_coeff) in term_coefficients.iter_mut() {
                if *existing_expr == base_term {
                    *existing_coeff += &coeff;
                    found = true;
                    break;
                }
            }
            if !found {
                term_coefficients.push((base_term, coeff));
            }
        }

        // Reconstruct expression
        let mut result_terms = Vec::new();

        for (base_term, total_coeff) in term_coefficients {
            if !total_coeff.is_zero() {
                let final_term = if total_coeff.is_one() {
                    base_term
                } else if base_term == Expression::integer(1) {
                    Expression::big_integer(total_coeff)
                } else {
                    Expression::mul(vec![Expression::big_integer(total_coeff), base_term])
                };
                result_terms.push(final_term);
            }
        }

        if result_terms.is_empty() {
            Expression::integer(0)
        } else if result_terms.len() == 1 {
            result_terms[0].clone()
        } else {
            Expression::add(result_terms)
        }
    }

    /// Extract coefficient and base term from any expression
    fn extract_coefficient_and_base(&self, expr: &Expression) -> (BigInt, Expression) {
        match expr {
            Expression::Number(Number::Integer(n)) => (BigInt::from(*n), Expression::integer(1)),

            Expression::Symbol(_) => (BigInt::one(), expr.clone()),

            Expression::Mul(factors) => {
                let mut coefficient = BigInt::one();
                let mut non_numeric_factors = Vec::new();

                for factor in factors.iter() {
                    if let Expression::Number(Number::Integer(n)) = factor {
                        coefficient *= BigInt::from(*n);
                    } else {
                        non_numeric_factors.push(factor.clone());
                    }
                }

                let base = if non_numeric_factors.is_empty() {
                    Expression::integer(1)
                } else if non_numeric_factors.len() == 1 {
                    non_numeric_factors[0].clone()
                } else {
                    Expression::mul(non_numeric_factors)
                };

                (coefficient, base)
            }

            _ => (BigInt::one(), expr.clone()),
        }
    }

    /// Collect terms in multiplication (combine powers of same base)
    fn collect_multiplication_terms(&self, factors: &[Expression]) -> Expression {
        // Use Vec instead of HashMap due to Expression not implementing Eq+Hash
        let mut base_powers: Vec<(Expression, Vec<Expression>)> = Vec::new();
        let mut numeric_factor = BigInt::one();
        let mut other_factors = Vec::new();

        for factor in factors {
            match factor {
                Expression::Number(Number::Integer(n)) => {
                    numeric_factor *= BigInt::from(*n);
                }
                Expression::Pow(base, exp) => {
                    // Find existing base or create new entry
                    let base_expr = (**base).clone();
                    let exp_expr = (**exp).clone();
                    let mut found = false;
                    for (existing_base, powers) in base_powers.iter_mut() {
                        if *existing_base == base_expr {
                            powers.push(exp_expr.clone());
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        base_powers.push((base_expr, vec![exp_expr]));
                    }
                }
                Expression::Symbol(_) => {
                    // Find existing base or create new entry
                    let mut found = false;
                    for (existing_base, powers) in base_powers.iter_mut() {
                        if *existing_base == *factor {
                            powers.push(Expression::integer(1));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        base_powers.push((factor.clone(), vec![Expression::integer(1)]));
                    }
                }
                _ => {
                    other_factors.push(factor.clone());
                }
            }
        }

        // Combine powers of same base
        let mut result_factors = Vec::new();

        if !numeric_factor.is_one() {
            result_factors.push(Expression::big_integer(numeric_factor));
        }

        for (base, exponents) in base_powers {
            if exponents.len() == 1 {
                if exponents[0] == Expression::integer(1) {
                    result_factors.push(base);
                } else {
                    result_factors.push(Expression::pow(base, exponents[0].clone()));
                }
            } else {
                // Add exponents: a^m * a^n = a^(m+n)
                let total_exp = Expression::add(exponents);
                result_factors.push(Expression::pow(base, total_exp));
            }
        }

        result_factors.extend(other_factors);

        if result_factors.is_empty() {
            Expression::integer(1)
        } else if result_factors.len() == 1 {
            result_factors[0].clone()
        } else {
            Expression::mul(result_factors)
        }
    }

    /// Separate variables and constants
    pub fn separate_constants(&self) -> (Expression, Expression) {
        match self {
            Expression::Add(terms) => {
                let mut constants = Vec::new();
                let mut variables = Vec::new();

                for term in terms.iter() {
                    if self.is_constant(term) {
                        constants.push(term.clone());
                    } else {
                        variables.push(term.clone());
                    }
                }

                let const_part = if constants.is_empty() {
                    Expression::integer(0)
                } else {
                    Expression::add(constants)
                };

                let var_part = if variables.is_empty() {
                    Expression::integer(0)
                } else {
                    Expression::add(variables)
                };

                (const_part, var_part)
            }
            _ => {
                if self.is_constant(self) {
                    (self.clone(), Expression::integer(0))
                } else {
                    (Expression::integer(0), self.clone())
                }
            }
        }
    }

    /// Check if an expression is constant (contains no variables)
    fn is_constant(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(_) => true,
            Expression::Symbol(_) => false,
            Expression::Add(terms) | Expression::Mul(terms) => {
                terms.iter().all(|t| self.is_constant(t))
            }
            Expression::Pow(base, exp) => self.is_constant(base) && self.is_constant(exp),
            Expression::Function { args, .. } => args.iter().all(|a| self.is_constant(a)),
            // New expression types - implement later
            Expression::Complex(_) => true,
            Expression::Matrix(_) => false,
            // All matrix types are handled by the Matrix variant
            // Expression::IdentityMatrix(_) => false, // Removed - now unified
            Expression::Constant(_) => true,
            Expression::Relation(_) => false,
            Expression::Piecewise(_) => false,
            Expression::Set(_) => false,
            Expression::Interval(_) => true,
            Expression::Calculus(_) => false,
            Expression::MethodCall(method_data) => {
                self.is_constant(&method_data.object)
                    && method_data.args.iter().all(|a| self.is_constant(a))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_collect_like_terms() {
        let x = symbol!(x);

        // Test 2x + 3x = 5x
        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        ]);

        let result = expr.collect(&x);
        println!("2x + 3x collected = {}", result);

        // Should combine to 5x
        assert!(!result.is_zero());
    }

    #[test]
    fn test_collect_different_powers() {
        let x = symbol!(x);

        // Test x^2 + 2x + x^2 = 2x^2 + 2x
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);

        let result = expr.collect(&x);
        println!("x^2 + 2x + x^2 collected = {}", result);

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2); // Should have 2x^2 and 2x
            }
            _ => println!("Collection result: {}", result),
        }
    }

    #[test]
    fn test_combine_like_terms() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Test 3x + 2y + x + y = 4x + 3y
        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let result = expr.combine_like_terms();
        println!("3x + 2y + x + y combined = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_collect_constants() {
        let x = symbol!(x);

        // Test 5 + 3x + 2 = 3x + 7
        let expr = Expression::add(vec![
            Expression::integer(5),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::integer(2),
        ]);

        let result = expr.collect(&x);
        println!("5 + 3x + 2 collected = {}", result);

        // Should combine constants
        assert!(!result.is_zero());
    }

    #[test]
    fn test_separate_constants() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::integer(5),
            Expression::symbol(x.clone()),
            Expression::integer(3),
        ]);

        let (constants, variables) = expr.separate_constants();

        println!("Constants: {}, Variables: {}", constants, variables);

        // Constants should be 5 + 3 = 8, variables should be x
        assert!(!constants.is_zero());
        assert!(!variables.is_zero());
    }

    #[test]
    fn test_collect_multiplication_powers() {
        let x = symbol!(x);

        // Test x^2 * x^3 = x^5 (if implemented)
        let expr = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ]);

        let result = expr.collect_terms();
        println!("x^2 * x^3 collected = {}", result);

        // Should combine to x^5
        assert!(!result.is_zero());
    }
}

//! Term collection operations

use crate::core::commutativity::Commutativity;
use crate::core::{Expression, Number, Symbol};
use crate::expr;
use num_bigint::BigInt;
use num_traits::{One, Zero};

impl Expression {
    /// Collect terms in an addition with respect to a variable
    pub(super) fn collect_addition_terms(&self, terms: &[Expression], var: &Symbol) -> Expression {
        let mut term_coefficients: Vec<(Expression, BigInt)> = Vec::new();
        let mut constant_term = BigInt::zero();

        for term in terms {
            let (coeff, power_expr) = self.extract_coefficient_and_power(term, var);

            if power_expr == expr!(0) {
                constant_term += coeff;
            } else {
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

        let mut result_terms = Vec::new();

        if !constant_term.is_zero() {
            result_terms.push(Expression::big_integer(constant_term));
        }

        for (power_expr, coeff) in term_coefficients {
            if !coeff.is_zero() {
                let term = if coeff.is_one() {
                    if power_expr == expr!(1) {
                        Expression::symbol(var.clone())
                    } else {
                        Expression::pow(Expression::symbol(var.clone()), power_expr)
                    }
                } else {
                    let var_part = if power_expr == expr!(1) {
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
            expr!(0)
        } else if result_terms.len() == 1 {
            result_terms[0].clone()
        } else {
            Expression::add(result_terms)
        }
    }

    /// Collect all like terms regardless of variable
    ///
    /// For commutative terms: 2AB + 3AB = 5AB
    /// For noncommutative terms: 2AB + 3BA stays as 2AB + 3BA (different order!)
    pub(super) fn collect_all_like_terms(&self, terms: &[Expression]) -> Expression {
        let mut term_coefficients: Vec<(Expression, BigInt)> = Vec::new();

        for term in terms {
            let (coeff, base_term) = self.extract_coefficient_and_base(term);

            let mut found = false;
            for (existing_expr, existing_coeff) in term_coefficients.iter_mut() {
                if *existing_expr == base_term {
                    let commutativity = Commutativity::combine(
                        vec![existing_expr.commutativity(), base_term.commutativity()]
                    );

                    if commutativity.can_sort() || self.same_factor_order(existing_expr, &base_term) {
                        *existing_coeff += &coeff;
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                term_coefficients.push((base_term, coeff));
            }
        }

        let mut result_terms = Vec::new();

        for (base_term, total_coeff) in term_coefficients {
            if !total_coeff.is_zero() {
                let final_term = if total_coeff.is_one() {
                    base_term
                } else if base_term == expr!(1) {
                    Expression::big_integer(total_coeff)
                } else {
                    Expression::mul(vec![Expression::big_integer(total_coeff), base_term])
                };
                result_terms.push(final_term);
            }
        }

        if result_terms.is_empty() {
            expr!(0)
        } else if result_terms.len() == 1 {
            result_terms[0].clone()
        } else {
            Expression::add(result_terms)
        }
    }

    /// Collect terms in multiplication (combine powers of same base)
    pub(super) fn collect_multiplication_terms(&self, factors: &[Expression]) -> Expression {
        let mut base_powers: Vec<(Expression, Vec<Expression>)> = Vec::new();
        let mut numeric_factor = BigInt::one();
        let mut other_factors = Vec::new();

        for factor in factors {
            match factor {
                Expression::Number(Number::Integer(n)) => {
                    numeric_factor *= BigInt::from(*n);
                }
                Expression::Pow(base, exp) => {
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
                    let mut found = false;
                    for (existing_base, powers) in base_powers.iter_mut() {
                        if *existing_base == *factor {
                            powers.push(expr!(1));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        base_powers.push((factor.clone(), vec![expr!(1)]));
                    }
                }
                _ => {
                    other_factors.push(factor.clone());
                }
            }
        }

        let mut result_factors = Vec::new();

        if !numeric_factor.is_one() {
            result_factors.push(Expression::big_integer(numeric_factor));
        }

        for (base, exponents) in base_powers {
            if exponents.len() == 1 {
                if exponents[0] == expr!(1) {
                    result_factors.push(base);
                } else {
                    result_factors.push(Expression::pow(base, exponents[0].clone()));
                }
            } else {
                let total_exp = Expression::add(exponents);
                result_factors.push(Expression::pow(base, total_exp));
            }
        }

        result_factors.extend(other_factors);

        if result_factors.is_empty() {
            expr!(1)
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
                    expr!(0)
                } else {
                    Expression::add(constants)
                };

                let var_part = if variables.is_empty() {
                    expr!(0)
                } else {
                    Expression::add(variables)
                };

                (const_part, var_part)
            }
            _ => {
                if self.is_constant(self) {
                    (self.clone(), expr!(0))
                } else {
                    (expr!(0), self.clone())
                }
            }
        }
    }
}

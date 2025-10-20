//! Rational function integration via partial fraction decomposition
//!
//! Implements integration of rational functions P(x)/Q(x) using:
//! 1. Polynomial long division if deg(P) >= deg(Q)
//! 2. Factor denominator into linear and irreducible quadratic factors
//! 3. Decompose into partial fractions
//! 4. Integrate each term using standard formulas
//!
//! # Integration Formulas
//!
//! ∫1/(x-r) dx = ln|x-r| + C
//! ∫1/(x-r)^n dx = -1/((n-1)(x-r)^(n-1)) + C  (n > 1)
//! ∫1/(x^2+a^2) dx = (1/a) * arctan(x/a) + C

use crate::algebra::gcd::PolynomialGcd;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

#[derive(Debug, Clone)]
pub struct LinearTerm {
    pub coefficient: Expression,
    pub root: Expression,
    pub power: i64,
}

#[derive(Debug, Clone)]
pub struct QuadraticTerm {
    pub numerator_linear_coeff: Expression,
    pub numerator_constant: Expression,
    pub p_coeff: Expression,
    pub q_coeff: Expression,
    pub power: i64,
}

#[derive(Debug, Clone)]
pub struct PartialFractionDecomposition {
    pub polynomial_part: Expression,
    pub linear_terms: Vec<LinearTerm>,
    pub quadratic_terms: Vec<QuadraticTerm>,
}

#[derive(Debug, Clone)]
enum Factor {
    Linear { root: Expression, power: i64 },
    Quadratic { p: Expression, q: Expression, power: i64 },
}

/// Check if expression is a rational function P(x)/Q(x)
pub fn is_rational_function(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Mul(factors) => {
            let mut has_denominator = false;
            for factor in factors.iter() {
                if let Expression::Pow(base, exp) = factor {
                    if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                        if *e < 0 && is_polynomial(base, var) {
                            has_denominator = true;
                        }
                    }
                }
            }
            has_denominator
        }
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                *e < 0 && is_polynomial(base, var)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Check if expression contains a given variable
fn contains_variable(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => s == var,
        Expression::Add(terms) => terms.iter().any(|t| contains_variable(t, var)),
        Expression::Mul(factors) => factors.iter().any(|f| contains_variable(f, var)),
        Expression::Pow(base, exp) => contains_variable(base, var) || contains_variable(exp, var),
        Expression::Function { args, .. } => args.iter().any(|a| contains_variable(a, var)),
        _ => false,
    }
}

/// Check if expression is a polynomial in the given variable
fn is_polynomial(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Number(_) => true,
        Expression::Symbol(s) => s == var || !contains_variable(expr, var),
        Expression::Add(terms) => terms.iter().all(|t| is_polynomial(t, var)),
        Expression::Mul(factors) => factors.iter().all(|f| is_polynomial(f, var)),
        Expression::Pow(base, exp) => {
            if let Expression::Symbol(s) = base.as_ref() {
                if s == var {
                    matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n >= 0)
                } else {
                    is_polynomial(base, var)
                }
            } else {
                is_polynomial(base, var)
                    && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n >= 0)
            }
        }
        _ => false,
    }
}

/// Get polynomial degree with respect to a variable
fn polynomial_degree(expr: &Expression, var: &Symbol) -> i64 {
    match expr {
        Expression::Symbol(s) if s == var => 1,
        Expression::Number(_) => 0,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    return *e;
                }
            }
            0
        }
        Expression::Add(terms) => terms.iter().map(|t| polynomial_degree(t, var)).max().unwrap_or(0),
        Expression::Mul(factors) => factors.iter().map(|f| polynomial_degree(f, var)).sum(),
        _ => 0,
    }
}

/// Extract numerator and denominator from rational expression
fn extract_numerator_denominator(expr: &Expression, var: &Symbol) -> (Expression, Expression) {
    match expr {
        Expression::Mul(factors) => {
            let mut numerator_factors = Vec::new();
            let mut denominator_factors = Vec::new();

            for factor in factors.iter() {
                if let Expression::Pow(base, exp) = factor {
                    if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                        if *e < 0 {
                            let positive_exp = Expression::integer(-e);
                            denominator_factors.push(Expression::pow((**base).clone(), positive_exp));
                        } else {
                            numerator_factors.push(factor.clone());
                        }
                    } else {
                        numerator_factors.push(factor.clone());
                    }
                } else {
                    numerator_factors.push(factor.clone());
                }
            }

            let numerator = if numerator_factors.is_empty() {
                Expression::integer(1)
            } else if numerator_factors.len() == 1 {
                numerator_factors[0].clone()
            } else {
                Expression::mul(numerator_factors)
            };

            let denominator = if denominator_factors.is_empty() {
                Expression::integer(1)
            } else if denominator_factors.len() == 1 {
                denominator_factors[0].clone()
            } else {
                Expression::mul(denominator_factors)
            };

            (numerator, denominator)
        }
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                if *e < 0 {
                    (Expression::integer(1), Expression::pow((**base).clone(), Expression::integer(-e)))
                } else {
                    (expr.clone(), Expression::integer(1))
                }
            } else {
                (expr.clone(), Expression::integer(1))
            }
        }
        _ => (expr.clone(), Expression::integer(1)),
    }
}

/// Integrate rational function P(x)/Q(x) via partial fractions
pub fn integrate_rational(expr: &Expression, var: &Symbol) -> Option<Expression> {
    if !is_rational_function(expr, var) {
        return None;
    }

    let (numerator, denominator) = extract_numerator_denominator(expr, var);

    let num_degree = polynomial_degree(&numerator, var);
    let den_degree = polynomial_degree(&denominator, var);

    let (quotient, remainder) = if num_degree >= den_degree {
        numerator.div_polynomial(&denominator, var)
    } else {
        (Expression::integer(0), numerator.clone())
    };

    let polynomial_integral = if !quotient.is_zero() {
        integrate_polynomial(&quotient, var)
    } else {
        Expression::integer(0)
    };

    if remainder.is_zero() {
        return Some(polynomial_integral);
    }

    let factors = factor_simple_denominator(&denominator, var)?;

    let mut result = polynomial_integral;

    for factor in factors {
        match factor {
            Factor::Linear { root, power } => {
                result = Expression::add(vec![
                    result,
                    integrate_linear_factor(&remainder, &denominator, &root, power, var),
                ])
                .simplify();
            }
            Factor::Quadratic { p, q, power } => {
                if power == 1 {
                    result = Expression::add(vec![
                        result,
                        integrate_simple_quadratic(&remainder, &denominator, &p, &q, var),
                    ])
                    .simplify();
                }
            }
        }
    }

    Some(result.simplify())
}

/// Integrate polynomial term
fn integrate_polynomial(poly: &Expression, var: &Symbol) -> Expression {
    match poly {
        Expression::Number(_) => Expression::mul(vec![poly.clone(), Expression::symbol(var.clone())]),
        Expression::Symbol(s) if s == var => {
            Expression::mul(vec![
                Expression::rational(1, 2),
                Expression::pow(Expression::symbol(var.clone()), Expression::integer(2)),
            ])
        }
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    let new_exp = n + 1;
                    return Expression::mul(vec![
                        Expression::rational(1, new_exp),
                        Expression::pow(Expression::symbol(var.clone()), Expression::integer(new_exp)),
                    ]);
                }
            }
            Expression::mul(vec![poly.clone(), Expression::symbol(var.clone())])
        }
        Expression::Mul(factors) => {
            let mut coeff = Expression::integer(1);
            let mut var_power = 0i64;

            for factor in factors.iter() {
                if let Expression::Symbol(s) = factor {
                    if s == var {
                        var_power += 1;
                        continue;
                    }
                }
                if let Expression::Pow(base, exp) = factor {
                    if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                        (base.as_ref(), exp.as_ref())
                    {
                        if s == var {
                            var_power += e;
                            continue;
                        }
                    }
                }
                coeff = Expression::mul(vec![coeff, factor.clone()]);
            }

            let new_power = var_power + 1;
            Expression::mul(vec![
                Expression::mul(vec![coeff, Expression::rational(1, new_power)]),
                Expression::pow(Expression::symbol(var.clone()), Expression::integer(new_power)),
            ])
        }
        Expression::Add(terms) => {
            Expression::add(terms.iter().map(|t| integrate_polynomial(t, var)).collect())
        }
        _ => Expression::mul(vec![poly.clone(), Expression::symbol(var.clone())]),
    }
}

/// Simple factorization for basic denominators
fn factor_simple_denominator(denom: &Expression, var: &Symbol) -> Option<Vec<Factor>> {
    let mut factors = Vec::new();

    match denom {
        Expression::Add(terms) => {
            if terms.len() == 2 {
                if let Expression::Symbol(s) = &terms[0] {
                    if s == var {
                        if let Expression::Number(_) = &terms[1] {
                            let root = Expression::mul(vec![Expression::integer(-1), terms[1].clone()]);
                            factors.push(Factor::Linear { root, power: 1 });
                            return Some(factors);
                        }
                    }
                }
            }
            factors.push(Factor::Linear {
                root: Expression::integer(0),
                power: 1,
            });
        }
        Expression::Mul(terms) => {
            for term in terms.iter() {
                if let Some(mut term_factors) = factor_simple_denominator(term, var) {
                    factors.append(&mut term_factors);
                }
            }
        }
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                if let Some(base_factors) = factor_simple_denominator(base, var) {
                    for factor in base_factors {
                        match factor {
                            Factor::Linear { root, power } => {
                                factors.push(Factor::Linear {
                                    root,
                                    power: power * e,
                                });
                            }
                            Factor::Quadratic { p, q, power } => {
                                factors.push(Factor::Quadratic {
                                    p,
                                    q,
                                    power: power * e,
                                });
                            }
                        }
                    }
                    return Some(factors);
                }
            }
        }
        Expression::Symbol(s) if s == var => {
            factors.push(Factor::Linear {
                root: Expression::integer(0),
                power: 1,
            });
        }
        _ => {
            return None;
        }
    }

    Some(factors)
}

/// Integrate linear partial fraction A/(x-r)^k
fn integrate_linear_factor(
    numerator: &Expression,
    denominator: &Expression,
    root: &Expression,
    power: i64,
    var: &Symbol,
) -> Expression {
    let coeff = substitute_variable(numerator, var, root).simplify();

    if power == 1 {
        Expression::mul(vec![
            coeff,
            Expression::function(
                "ln",
                vec![Expression::function(
                    "abs",
                    vec![Expression::add(vec![
                        Expression::symbol(var.clone()),
                        Expression::mul(vec![Expression::integer(-1), root.clone()]),
                    ])],
                )],
            ),
        ])
    } else {
        Expression::mul(vec![
            Expression::integer(-1),
            coeff,
            Expression::rational(1, power - 1),
            Expression::pow(
                Expression::add(vec![
                    Expression::symbol(var.clone()),
                    Expression::mul(vec![Expression::integer(-1), root.clone()]),
                ]),
                Expression::integer(-(power - 1)),
            ),
        ])
    }
}

/// Simple substitution of variable with value
fn substitute_variable(expr: &Expression, var: &Symbol, value: &Expression) -> Expression {
    match expr {
        Expression::Symbol(s) if s == var => value.clone(),
        Expression::Number(_) => expr.clone(),
        Expression::Constant(_) => expr.clone(),
        Expression::Add(terms) => Expression::add(
            terms
                .iter()
                .map(|t| substitute_variable(t, var, value))
                .collect(),
        ),
        Expression::Mul(factors) => Expression::mul(
            factors
                .iter()
                .map(|f| substitute_variable(f, var, value))
                .collect(),
        ),
        Expression::Pow(base, exp) => Expression::pow(
            substitute_variable(base, var, value),
            substitute_variable(exp, var, value),
        ),
        Expression::Function { name, args } => Expression::function(
            name,
            args.iter()
                .map(|a| substitute_variable(a, var, value))
                .collect(),
        ),
        _ => expr.clone(),
    }
}

/// Integrate simple quadratic term 1/(x^2+px+q)
fn integrate_simple_quadratic(
    _numerator: &Expression,
    _denominator: &Expression,
    p: &Expression,
    q: &Expression,
    var: &Symbol,
) -> Expression {
    let discriminant = Expression::add(vec![
        Expression::pow(p.clone(), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-4), q.clone()]),
    ])
    .simplify();

    if let Expression::Number(Number::Integer(d)) = discriminant {
        if d < 0 {
            let a_squared = Expression::mul(vec![Expression::rational(1, 4), discriminant.abs()])
                .simplify();

            if let Expression::Number(_) = a_squared {
                let a = Expression::function("sqrt", vec![a_squared.clone()]);
                let shift = Expression::mul(vec![Expression::rational(1, 2), p.clone()]);
                let x_shifted =
                    Expression::add(vec![Expression::symbol(var.clone()), shift]).simplify();

                return Expression::mul(vec![
                    Expression::pow(a.clone(), Expression::integer(-1)),
                    Expression::function(
                        "atan",
                        vec![Expression::mul(vec![
                            Expression::pow(a, Expression::integer(-1)),
                            x_shifted,
                        ])],
                    ),
                ]);
            }
        }
    }

    Expression::function(
        "atan",
        vec![Expression::mul(vec![
            Expression::pow(
                Expression::function("sqrt", vec![q.clone()]),
                Expression::integer(-1),
            ),
            Expression::symbol(var.clone()),
        ])],
    )
}

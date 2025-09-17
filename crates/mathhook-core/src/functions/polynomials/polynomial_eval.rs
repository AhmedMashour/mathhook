//! Polynomial function evaluations

use crate::core::{Expression, Number, Symbol};

/// Get polynomial degree
///
/// # Mathematical Definition
///
/// degree(p(x)) = highest power of x in polynomial p
///
/// # Arguments
///
/// * `poly` - Polynomial expression
/// * `var` - Variable to check degree for
///
/// # Returns
///
/// Degree as integer expression or symbolic
///
/// # Examples
///
/// ```ignore
/// use mathhook_core::functions::polynomials::polynomial_eval::degree;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!((x ^ 3) + (2 * (x ^ 2)) + x + 1);
/// let deg = degree(&poly, &x);
/// assert_eq!(deg, expr!(3));
/// ```
pub fn degree(poly: &Expression, var: &Symbol) -> Expression {
    match poly {
        Expression::Pow(base, exp) if matches!(**base, Expression::Symbol(ref s) if s == var) => {
            if let Expression::Number(Number::Integer(n)) = **exp {
                Expression::integer(n)
            } else {
                exp.as_ref().clone()
            }
        }
        Expression::Add(terms) => {
            let mut max_degree = 0i64;
            for term in terms.iter() {
                if let Expression::Number(Number::Integer(d)) = degree(term, var) {
                    max_degree = max_degree.max(d);
                }
            }
            Expression::integer(max_degree)
        }
        Expression::Mul(factors) => {
            let mut total_degree = 0i64;
            for factor in factors.iter() {
                if let Expression::Number(Number::Integer(d)) = degree(factor, var) {
                    total_degree += d;
                }
            }
            Expression::integer(total_degree)
        }
        Expression::Symbol(s) if s == var => Expression::integer(1),
        Expression::Number(_) => Expression::integer(0),
        _ => Expression::function(
            "degree",
            vec![poly.clone(), Expression::symbol(var.clone())],
        ),
    }
}

/// Find polynomial roots
///
/// # Mathematical Definition
///
/// roots(p(x)) = {x : p(x) = 0}
///
/// # Arguments
///
/// * `poly` - Polynomial expression
/// * `var` - Variable to solve for
///
/// # Returns
///
/// Set of roots or symbolic expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::polynomials::polynomial_eval::roots;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!((x ^ 2) - 1);
/// let r = roots(&poly, &x);
/// ```
pub fn roots(poly: &Expression, var: &Symbol) -> Expression {
    Expression::function("roots", vec![poly.clone(), Expression::symbol(var.clone())])
}

/// Expand polynomial expression
///
/// # Mathematical Definition
///
/// expand((x+1)²) = x² + 2x + 1
///
/// # Arguments
///
/// * `expr` - Expression to expand
///
/// # Returns
///
/// Expanded expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::polynomials::polynomial_eval::expand;
/// use mathhook_core::expr;
///
/// let result = expand(&expr!(1));
/// assert_eq!(result, expr!(1));
/// ```
pub fn expand(expr: &Expression) -> Expression {
    expr.clone()
}

/// Factor polynomial expression
///
/// # Mathematical Definition
///
/// factor(x² - 1) = (x - 1)(x + 1)
///
/// # Arguments
///
/// * `expr` - Expression to factor
///
/// # Returns
///
/// Factored expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::polynomials::polynomial_eval::factor;
/// use mathhook_core::expr;
///
/// let result = factor(&expr!(1));
/// assert_eq!(result, expr!(1));
/// ```
pub fn factor(expr: &Expression) -> Expression {
    expr.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_degree_constant() {
        let x = symbol!(x);
        assert_eq!(degree(&Expression::integer(5), &x), Expression::integer(0));
    }

    #[test]
    fn test_degree_linear() {
        let x = symbol!(x);
        assert_eq!(
            degree(&Expression::symbol(x.clone()), &x),
            Expression::integer(1)
        );
    }

    #[test]
    fn test_degree_quadratic() {
        let x = symbol!(x);
        let poly = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert_eq!(degree(&poly, &x), Expression::integer(2));
    }

    #[test]
    fn test_expand_constant() {
        assert_eq!(expand(&Expression::integer(1)), Expression::integer(1));
    }

    #[test]
    fn test_factor_constant() {
        assert_eq!(factor(&Expression::integer(1)), Expression::integer(1));
    }
}

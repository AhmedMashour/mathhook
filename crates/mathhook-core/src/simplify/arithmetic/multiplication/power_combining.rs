//! Power combining operations for multiplication simplification
//!
//! Implements the power rule: x^a * x^b → x^(a+b) and x * x → x^2

use super::super::addition::simplify_addition;
use crate::core::Expression;

/// Extract the base and exponent from an expression
///
/// - `Pow(base, exp)` → `(base, exp)`
/// - `Symbol/Function/etc.` → `(expression, 1)` (implicit exponent 1)
fn extract_base_and_exponent(expr: &Expression) -> (Expression, Expression) {
    match expr {
        Expression::Pow(base, exp) => ((**base).clone(), (**exp).clone()),
        _ => (expr.clone(), Expression::integer(1)),
    }
}

/// Check if an expression is a "combinable base" (can be raised to a power)
///
/// We want to combine Symbol, Function, Add, Constant, but NOT Number
/// (numbers combine via multiplication: 2 * 3 = 6, not 2^1 * 3^1)
fn is_combinable_base(expr: &Expression) -> bool {
    !matches!(expr, Expression::Number(_))
}

/// Combine like powers in a multiplication
///
/// Groups factors with same base and combines their exponents:
/// - `x^2 * x^3` → `x^(2+3)` = `x^5`
/// - `x * x` → `x^2`
/// - `x * x^2` → `x^3`
/// - `sin(x) * sin(x)` → `sin(x)^2`
///
/// # Arguments
/// * `factors` - List of factors in the multiplication
///
/// # Returns
/// Factors with like powers combined
pub fn combine_like_powers(factors: Vec<Expression>) -> Vec<Expression> {
    let mut grouped_powers: Vec<(Expression, Vec<Expression>)> = Vec::new();
    let mut numeric_factors = Vec::new();

    for factor in factors {
        if !is_combinable_base(&factor) && !matches!(&factor, Expression::Pow(_, _)) {
            numeric_factors.push(factor);
            continue;
        }

        let (base, exp) = extract_base_and_exponent(&factor);

        let mut found = false;
        for (existing_base, exponents) in &mut grouped_powers {
            if existing_base == &base {
                exponents.push(exp.clone());
                found = true;
                break;
            }
        }
        if !found {
            grouped_powers.push((base, vec![exp]));
        }
    }

    let mut result = numeric_factors;

    for (base, exponents) in grouped_powers {
        if exponents.len() == 1 {
            let exp = exponents.into_iter().next().unwrap();
            if exp == Expression::integer(1) {
                result.push(base);
            } else {
                result.push(Expression::pow(base, exp));
            }
        } else {
            let combined_exp = simplify_addition(&exponents);
            if combined_exp == Expression::integer(1) {
                result.push(base);
            } else {
                result.push(Expression::pow(base, combined_exp));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_combine_like_powers() {
        let x = symbol!(x);
        let factors = vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ];

        let result = combine_like_powers(factors);

        assert_eq!(result.len(), 1);
        match &result[0] {
            Expression::Pow(base, exp) => {
                assert_eq!(**base, Expression::symbol(x));
                assert_eq!(**exp, Expression::integer(5));
            }
            _ => panic!("Expected Pow, got {:?}", result[0]),
        }
    }

    #[test]
    fn test_combine_non_powers_preserved() {
        let x = symbol!(x);
        let y = symbol!(y);
        let factors = vec![
            Expression::symbol(x.clone()),
            Expression::integer(2),
            Expression::symbol(y.clone()),
        ];

        let result = combine_like_powers(factors);

        assert_eq!(result.len(), 3);
        assert!(result.contains(&Expression::symbol(x)));
        assert!(result.contains(&Expression::symbol(y)));
        assert!(result.contains(&Expression::integer(2)));
    }

    #[test]
    fn test_combine_x_times_x() {
        let x = symbol!(x);
        let factors = vec![Expression::symbol(x.clone()), Expression::symbol(x.clone())];

        let result = combine_like_powers(factors);

        assert_eq!(result.len(), 1);
        match &result[0] {
            Expression::Pow(base, exp) => {
                assert_eq!(**base, Expression::symbol(x));
                assert_eq!(**exp, Expression::integer(2));
            }
            _ => panic!("Expected Pow(x, 2), got {:?}", result[0]),
        }
    }

    #[test]
    fn test_combine_x_times_x_squared() {
        let x = symbol!(x);
        let factors = vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ];

        let result = combine_like_powers(factors);

        assert_eq!(result.len(), 1);
        match &result[0] {
            Expression::Pow(base, exp) => {
                assert_eq!(**base, Expression::symbol(x));
                assert_eq!(**exp, Expression::integer(3));
            }
            _ => panic!("Expected Pow(x, 3), got {:?}", result[0]),
        }
    }

    #[test]
    fn test_combine_with_numeric_coefficient() {
        let x = symbol!(x);
        let factors = vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
        ];

        let result = combine_like_powers(factors);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&Expression::integer(2)));
        let has_x_squared = result.iter().any(|f| {
            matches!(f, Expression::Pow(base, exp)
                if **base == Expression::symbol(x.clone()) && **exp == Expression::integer(2))
        });
        assert!(has_x_squared, "Expected x^2 in result: {:?}", result);
    }
}

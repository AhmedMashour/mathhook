use std::sync::LazyLock;
use std::collections::HashMap;
use crate::core::expression::Expression;
use crate::core::number::Number;

pub type SpecialValuesMap = HashMap<String, Expression>;

/// Special values for the gamma function
///
/// Mathematical properties:
/// - Γ(1) = 1
/// - Γ(1/2) = √π
/// - Γ(n) = (n-1)! for positive integers n
/// - Γ(n+1) = n·Γ(n) (recurrence relation)
pub static GAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert("1".to_string(), Expression::integer(1));
    map.insert("2".to_string(), Expression::integer(1));
    map.insert("3".to_string(), Expression::integer(2));
    map.insert("4".to_string(), Expression::integer(6));
    map.insert("5".to_string(), Expression::integer(24));

    map.insert("1/2".to_string(), Expression::sqrt(Expression::pi()));
    map.insert(
        "3/2".to_string(),
        Expression::div(
            Expression::sqrt(Expression::pi()),
            Expression::integer(2)
        )
    );

    map
});

/// Special value patterns for gamma function
///
/// Handles both direct lookup and pattern-based special values:
/// - Direct lookup for small positive integers and half-integers
/// - Pattern-based for general positive integers: Γ(n) = (n-1)!
/// - Pattern-based for half-integers: Γ(n+1/2) = (2n-1)!! · √π / 2^n
///
/// # Arguments
///
/// * `arg` - The argument to check for special values
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::expression::Expression;
/// use mathhook_core::core::functions::gamma::data::gamma_special_value;
///
/// let result = gamma_special_value(&Expression::integer(5));
/// assert_eq!(result, Some(Expression::integer(24)));
/// ```
pub fn gamma_special_value(arg: &Expression) -> Option<Expression> {
    if let Some(val) = GAMMA_SPECIAL_VALUES.get(&arg.to_string()) {
        return Some(val.clone());
    }

    match arg {
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            let mut result = 1i64;
            for i in 1..*n {
                result *= i;
            }
            Some(Expression::integer(result))
        }
        Expression::Number(Number::Float(x)) => {
            let twice = x * 2.0;
            if (twice - twice.round()).abs() < 1e-10 {
                let n = (x - 0.5).round() as i64;
                if (x - (n as f64 + 0.5)).abs() < 1e-10 && n >= 0 {
                    let sqrt_pi = Expression::sqrt(Expression::pi());
                    if n == 0 {
                        return Some(sqrt_pi);
                    }
                    let mut double_fact = Expression::integer(1);
                    for k in 0..n {
                        let term = Expression::integer(2 * k + 1);
                        double_fact = Expression::mul(vec![double_fact, term]);
                    }
                    let numerator = Expression::mul(vec![double_fact, sqrt_pi]);
                    let denominator = Expression::pow(Expression::integer(2), Expression::integer(n));
                    return Some(Expression::div(numerator, denominator));
                }
            }
            None
        }
        _ => None
    }
}

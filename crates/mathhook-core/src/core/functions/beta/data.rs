use std::sync::LazyLock;
use std::collections::HashMap;
use crate::core::expression::Expression;

pub type SpecialValuesMap = HashMap<String, Expression>;

/// Special values for the beta function
///
/// Mathematical properties:
/// - β(1,1) = 1
/// - β(a,b) = β(b,a) (symmetry)
/// - β(a,b) = Γ(a)·Γ(b)/Γ(a+b)
/// - β(1,n) = 1/n
/// - β(2,2) = 1/6
pub static BETA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert("(1,1)".to_string(), Expression::integer(1));
    map.insert("(1,2)".to_string(), Expression::rational(1, 2));
    map.insert("(2,1)".to_string(), Expression::rational(1, 2));
    map.insert("(2,2)".to_string(), Expression::rational(1, 6));
    map.insert("(1,3)".to_string(), Expression::rational(1, 3));
    map.insert("(3,1)".to_string(), Expression::rational(1, 3));
    map.insert("(2,3)".to_string(), Expression::rational(1, 12));
    map.insert("(3,2)".to_string(), Expression::rational(1, 12));

    map
});

/// Special value patterns for beta function
///
/// # Arguments
///
/// * `a` - First parameter
/// * `b` - Second parameter
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::expression::Expression;
/// use mathhook_core::core::functions::beta::data::beta_special_value;
///
/// let result = beta_special_value(&Expression::integer(1), &Expression::integer(1));
/// assert_eq!(result, Some(Expression::integer(1)));
/// ```
pub fn beta_special_value(a: &Expression, b: &Expression) -> Option<Expression> {
    let key = format!("({},{})", a.to_string(), b.to_string());
    BETA_SPECIAL_VALUES.get(&key).cloned()
}

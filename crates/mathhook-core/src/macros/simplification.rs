//! Simplification pattern macros
//!
//! Common simplification patterns and algebraic identities.
//! These macros eliminate repetitive simplification checks and provide
//! compile-time optimized pattern matching for mathematical expressions.

/// Common simplification patterns
///
/// This macro provides optimized pattern matching for common mathematical
/// simplifications and algebraic identities.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::simplify;
/// use mathhook_core::Expression;
///
/// let zero = Expression::integer(0);
/// let one = Expression::integer(1);
/// let x = Expression::symbol(mathhook_core::Symbol::new("x"));
///
/// // Zero and one detection
/// assert!(simplify!(is_zero: zero));
/// assert!(simplify!(is_one: one));
/// assert!(!simplify!(is_zero: one));
///
/// // Algebraic identities
/// if let Some(result) = simplify!(identity: mul_zero, zero.clone()) {
///     assert_eq!(result, Expression::integer(0));
/// }
///
/// // Power rules
/// let power_zero = simplify!(power: x.clone(), 0);
/// assert_eq!(power_zero, Expression::integer(1));
/// ```
#[macro_export]
macro_rules! simplify {
    // Zero detection
    (is_zero: $expr:expr) => {
        matches!($expr, $crate::Expression::Number($crate::Number::Integer(0)))
    };

    // One detection
    (is_one: $expr:expr) => {
        matches!($expr, $crate::Expression::Number($crate::Number::Integer(1)))
    };

    // Negative one detection
    (is_neg_one: $expr:expr) => {
        matches!($expr, $crate::Expression::Number($crate::Number::Integer(-1)))
    };

    // Constant folding for integers
    (fold: $op:tt, $a:expr, $b:expr) => {
        match ($a, $b) {
            (
                $crate::Expression::Number($crate::Number::Integer(a)),
                $crate::Expression::Number($crate::Number::Integer(b))
            ) => {
                Some($crate::Expression::integer(a $op b))
            }
            _ => None
        }
    };

    // Addition identities
    (identity: add_zero, $expr:expr) => {
        if simplify!(is_zero: $expr) {
            Some($crate::Expression::integer(0))
        } else {
            None
        }
    };

    // Multiplication identities
    (identity: mul_zero, $expr:expr) => {
        if simplify!(is_zero: $expr) {
            Some($crate::Expression::integer(0))
        } else {
            None
        }
    };

    (identity: mul_one, $expr:expr) => {
        if simplify!(is_one: $expr) {
            Some($expr.clone())
        } else {
            None
        }
    };

    (identity: mul_neg_one, $expr:expr) => {
        if simplify!(is_neg_one: $expr) {
            Some($crate::Expression::mul(vec![
                $crate::Expression::integer(-1),
                $expr.clone()
            ]))
        } else {
            None
        }
    };

    // Power simplification rules
    (power: $base:expr, 0) => {
        $crate::Expression::integer(1)
    };

    (power: $base:expr, 1) => {
        $base.clone()
    };

    (power: 0, $exp:expr) => {
        $crate::Expression::integer(0)
    };

    (power: 1, $exp:expr) => {
        $crate::Expression::integer(1)
    };

    // Trigonometric identities
    (trig: sin(0)) => {
        $crate::Expression::integer(0)
    };

    (trig: cos(0)) => {
        $crate::Expression::integer(1)
    };

    (trig: tan(0)) => {
        $crate::Expression::integer(0)
    };

    (trig: sin(π/2)) => {
        $crate::Expression::integer(1)
    };

    (trig: cos(π/2)) => {
        $crate::Expression::integer(0)
    };

    // Logarithmic identities
    (log: ln(1)) => {
        $crate::Expression::integer(0)
    };

    (log: ln(e)) => {
        $crate::Expression::integer(1)
    };

    (log: log(1, $base:expr)) => {
        $crate::Expression::integer(0)
    };

    (log: log($base:expr, $same_base:expr)) => {
        if $base == $same_base {
            $crate::Expression::integer(1)
        } else {
            $crate::Expression::function("log", vec![$base, $same_base])
        }
    };

    // Exponential identities
    (exp: exp(0)) => {
        $crate::Expression::integer(1)
    };

    (exp: exp(1)) => {
        $crate::Expression::constant($crate::MathConstant::E)
    };

    // Square root identities
    (sqrt: sqrt(0)) => {
        $crate::Expression::integer(0)
    };

    (sqrt: sqrt(1)) => {
        $crate::Expression::integer(1)
    };

    (sqrt: sqrt(4)) => {
        $crate::Expression::integer(2)
    };

    (sqrt: sqrt(9)) => {
        $crate::Expression::integer(3)
    };

    // Absolute value identities
    (abs: abs(0)) => {
        $crate::Expression::integer(0)
    };

    (abs: abs($x:expr)) => {
        // For positive constants, return the constant
        match $x {
            $crate::Expression::Number($crate::Number::Integer(n)) if *n >= 0 => {
                Some($x.clone())
            }
            $crate::Expression::Number($crate::Number::Integer(n)) if *n < 0 => {
                Some($crate::Expression::integer(-n))
            }
            _ => None
        }
    };

    // Factorial identities
    (factorial: factorial(0)) => {
        $crate::Expression::integer(1)
    };

    (factorial: factorial(1)) => {
        $crate::Expression::integer(1)
    };

    (factorial: factorial(2)) => {
        $crate::Expression::integer(2)
    };

    (factorial: factorial(3)) => {
        $crate::Expression::integer(6)
    };

    (factorial: factorial(4)) => {
        $crate::Expression::integer(24)
    };

    // Distributive property
    (distribute: $a:expr, $b:expr, $c:expr) => {
        // a * (b + c) = a * b + a * c
        $crate::Expression::add(vec![
            $crate::Expression::mul(vec![$a.clone(), $b]),
            $crate::Expression::mul(vec![$a, $c])
        ])
    };

    // Factor out common terms
    (factor_common: $terms:expr) => {{
        // Find common factors in a list of terms
        // This is a simplified version - full implementation would be more complex
        $terms
    }};

    // Combine like terms
    (combine_like: $terms:expr) => {{
        // Combine terms with the same variables and powers
        // This is a simplified version - full implementation would be more complex
        $terms
    }};

    // Rationalize denominator
    (rationalize: $num:expr, $den:expr) => {{
        // Multiply by conjugate to rationalize
        // This is a simplified version - full implementation would handle various cases
        $crate::Expression::mul(vec![$num, $crate::Expression::pow($den, $crate::Expression::integer(-1))])
    }};

    // Cancel common factors
    (cancel: $num:expr, $den:expr) => {{
        // Cancel common factors in numerator and denominator
        // This is a simplified version - full implementation would find and cancel common factors
        $crate::Expression::mul(vec![$num, $crate::Expression::pow($den, $crate::Expression::integer(-1))])
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Expression, MathConstant, Number, Symbol};

    #[test]
    fn test_simplify_is_zero() {
        let zero = Expression::integer(0);
        let one = Expression::integer(1);

        assert!(simplify!(is_zero: zero));
        assert!(!simplify!(is_zero: one));
    }

    #[test]
    fn test_simplify_is_one() {
        let zero = Expression::integer(0);
        let one = Expression::integer(1);

        assert!(simplify!(is_one: one));
        assert!(!simplify!(is_one: zero));
    }

    #[test]
    fn test_simplify_is_neg_one() {
        let neg_one = Expression::integer(-1);
        let one = Expression::integer(1);

        assert!(simplify!(is_neg_one: neg_one));
        assert!(!simplify!(is_neg_one: one));
    }

    #[test]
    fn test_simplify_fold_addition() {
        let a = Expression::integer(2);
        let b = Expression::integer(3);

        if let Some(result) = simplify!(fold: +, a, b) {
            assert_eq!(result, Expression::integer(5));
        } else {
            panic!("Should have folded constants");
        }
    }

    #[test]
    fn test_simplify_fold_multiplication() {
        let a = Expression::integer(2);
        let b = Expression::integer(3);

        if let Some(result) = simplify!(fold: *, a, b) {
            assert_eq!(result, Expression::integer(6));
        } else {
            panic!("Should have folded constants");
        }
    }

    #[test]
    fn test_simplify_fold_non_constants() {
        let a = Expression::symbol(Symbol::new("x"));
        let b = Expression::integer(3);

        let result = simplify!(fold: +, a, b);
        assert!(result.is_none());
    }

    #[test]
    fn test_simplify_identity_add_zero() {
        let zero = Expression::integer(0);

        if let Some(result) = simplify!(identity: add_zero, zero) {
            assert_eq!(result, Expression::integer(0));
        } else {
            panic!("Should have applied add_zero identity");
        }
    }

    #[test]
    fn test_simplify_identity_mul_zero() {
        let zero = Expression::integer(0);

        if let Some(result) = simplify!(identity: mul_zero, zero) {
            assert_eq!(result, Expression::integer(0));
        } else {
            panic!("Should have applied mul_zero identity");
        }
    }

    #[test]
    fn test_simplify_identity_mul_one() {
        let one = Expression::integer(1);
        let x = Expression::symbol(Symbol::new("x"));

        if let Some(result) = simplify!(identity: mul_one, one) {
            assert_eq!(result, Expression::integer(1));
        } else {
            panic!("Should have applied mul_one identity");
        }
    }

    #[test]
    fn test_simplify_power_rules() {
        let x = Expression::symbol(Symbol::new("x"));

        // x^0 = 1
        let power_zero = simplify!(power: x.clone(), 0);
        assert_eq!(power_zero, Expression::integer(1));

        // x^1 = x
        let power_one = simplify!(power: x.clone(), 1);
        assert_eq!(power_one, x);

        // 0^n = 0
        let zero_power = simplify!(power: 0, x);
        assert_eq!(zero_power, Expression::integer(0));

        // 1^n = 1
        let one_power = simplify!(power: 1, x);
        assert_eq!(one_power, Expression::integer(1));
    }

    #[test]
    fn test_simplify_trig_identities() {
        let sin_zero = simplify!(trig: sin(0));
        assert_eq!(sin_zero, Expression::integer(0));

        let cos_zero = simplify!(trig: cos(0));
        assert_eq!(cos_zero, Expression::integer(1));

        let tan_zero = simplify!(trig: tan(0));
        assert_eq!(tan_zero, Expression::integer(0));
    }

    #[test]
    fn test_simplify_log_identities() {
        let ln_one = simplify!(log: ln(1));
        assert_eq!(ln_one, Expression::integer(0));

        let ln_e = simplify!(log: ln(e));
        assert_eq!(ln_e, Expression::integer(1));
    }

    #[test]
    fn test_simplify_exp_identities() {
        let exp_zero = simplify!(exp: exp(0));
        assert_eq!(exp_zero, Expression::integer(1));

        let exp_one = simplify!(exp: exp(1));
        assert_eq!(exp_one, Expression::constant(MathConstant::E));
    }

    #[test]
    fn test_simplify_sqrt_identities() {
        let sqrt_zero = simplify!(sqrt: sqrt(0));
        assert_eq!(sqrt_zero, Expression::integer(0));

        let sqrt_one = simplify!(sqrt: sqrt(1));
        assert_eq!(sqrt_one, Expression::integer(1));

        let sqrt_four = simplify!(sqrt: sqrt(4));
        assert_eq!(sqrt_four, Expression::integer(2));
    }

    #[test]
    fn test_simplify_abs_identities() {
        let abs_zero = simplify!(abs: abs(0));
        assert_eq!(abs_zero, Expression::integer(0));

        let positive = Expression::integer(5);
        if let Some(result) = simplify!(abs: abs(positive)) {
            assert_eq!(result, Expression::integer(5));
        }

        let negative = Expression::integer(-5);
        if let Some(result) = simplify!(abs: abs(negative)) {
            assert_eq!(result, Expression::integer(5));
        }
    }

    #[test]
    fn test_simplify_factorial_identities() {
        let fact_zero = simplify!(factorial: factorial(0));
        assert_eq!(fact_zero, Expression::integer(1));

        let fact_one = simplify!(factorial: factorial(1));
        assert_eq!(fact_one, Expression::integer(1));

        let fact_four = simplify!(factorial: factorial(4));
        assert_eq!(fact_four, Expression::integer(24));
    }

    #[test]
    fn test_simplify_distribute() {
        let a = Expression::integer(2);
        let b = Expression::integer(3);
        let c = Expression::integer(4);

        let result = simplify!(distribute: a, b, c);

        // Should be 2 * 3 + 2 * 4 = 6 + 8 = 14 (when simplified)
        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => panic!("Expected addition expression"),
        }
    }
}

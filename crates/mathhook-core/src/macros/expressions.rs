//! Pragmatic expression construction macros
//!
//! Simplified but genuinely useful macros that work within declarative macro limitations.

/// Expression creation with useful shortcuts
///
/// This macro provides genuinely useful syntax without trying to parse full expressions.
/// For complex expressions with mixed operators, use the explicit API or a future
/// procedural macro.
///
/// # What Works Well
///
/// ```rust
/// use mathhook_core::{expr, symbol};
///
/// // Literals
/// let num = expr!(42);
///
/// // Symbols (clean!)
/// let x = expr!(x);
///
/// // Single operations (clean!)
/// let sum = expr!(x + y);
/// let product = expr!(x * y);
/// let power = expr!(x ^ 2);
/// let div = expr!(x / y);
///
/// // Functions (natural!)
/// let sin_x = expr!(sin(x));
/// let log_xy = expr!(log(x, y));
///
/// // Negation
/// let neg = expr!(-x);
///
/// // You can nest with parentheses for clarity
/// let complex = expr!((x + 1) * (y - 1));
/// ```
///
/// # Limitations
///
/// For mixed operations, operator precedence isn't perfect. Use explicit grouping:
/// ```rust
/// // These work as expected:
/// expr!(2 * x)        // ✅ 2*x
/// expr!(x + y)        // ✅ x+y
/// expr!((2*x) + 3)    // ✅ (2*x)+3 with explicit grouping
///
/// // This might not work as expected:
/// expr!(2*x + 3)      // ⚠️  Precedence unclear - use (2*x) + 3
/// ```
#[macro_export]
macro_rules! expr {
    // === Base Cases ===

    // Literal integer
    ($n:literal) => {
        $crate::Expression::integer($n)
    };

    // Symbol or constant
    ($id:ident) => {{
        // Common mathematical constants use dedicated constructors
        match stringify!($id) {
            "pi" => $crate::Expression::pi(),
            "e" => $crate::Expression::e(),
            "i" => $crate::Expression::i(),
            _ => $crate::Expression::symbol($crate::Symbol::new(stringify!($id)))
        }
    }};

    // === Function Calls ===

    // Zero argument function: f()
    ($fname:ident()) => {
        $crate::Expression::function(stringify!($fname), vec![])
    };

    // One argument: f(x)
    ($fname:ident($arg:tt)) => {
        $crate::Expression::function(stringify!($fname), vec![$crate::expr!($arg)])
    };

    // Two arguments: f(x, y)
    ($fname:ident($arg1:tt, $arg2:tt)) => {
        $crate::Expression::function(
            stringify!($fname),
            vec![$crate::expr!($arg1), $crate::expr!($arg2)]
        )
    };

    // Three arguments: f(x, y, z)
    ($fname:ident($arg1:tt, $arg2:tt, $arg3:tt)) => {
        $crate::Expression::function(
            stringify!($fname),
            vec![$crate::expr!($arg1), $crate::expr!($arg2), $crate::expr!($arg3)]
        )
    };

    // === Parenthesized Expressions ===

    // (expr)
    (($($inner:tt)+)) => {
        $crate::expr!($($inner)+)
    };

    // === Unary Operations ===

    // Negation: -x
    (- $e:tt) => {
        $crate::Expression::mul(vec![
            $crate::Expression::integer(-1),
            $crate::expr!($e)
        ])
    };

    // === Binary Operations (Single Operator) ===

    // Addition: a + b
    ($a:tt + $b:tt) => {
        $crate::Expression::add(vec![$crate::expr!($a), $crate::expr!($b)])
    };

    // Subtraction: a - b
    ($a:tt - $b:tt) => {
        $crate::Expression::add(vec![
            $crate::expr!($a),
            $crate::expr!(- $b)
        ])
    };

    // Multiplication: a * b
    ($a:tt * $b:tt) => {
        $crate::Expression::mul(vec![$crate::expr!($a), $crate::expr!($b)])
    };

    // Division: a / b
    ($a:tt / $b:tt) => {
        $crate::Expression::mul(vec![
            $crate::expr!($a),
            $crate::Expression::pow($crate::expr!($b), $crate::Expression::integer(-1))
        ])
    };

    // Exponentiation: a ^ b
    ($a:tt ^ $b:tt) => {
        $crate::Expression::pow($crate::expr!($a), $crate::expr!($b))
    };

    // === Chained Operations (Explicit) ===

    // Multi-term addition with explicit grouping
    (add: $($terms:tt),+ $(,)?) => {
        $crate::Expression::add(vec![$($crate::expr!($terms)),+])
    };

    // Multi-factor multiplication with explicit grouping
    (mul: $($factors:tt),+ $(,)?) => {
        $crate::Expression::mul(vec![$($crate::expr!($factors)),+])
    };
}

/// Symbol creation macro (unchanged - this one is perfect)
#[macro_export]
macro_rules! symbol {
    ($id:ident) => {
        $crate::Symbol::new(stringify!($id))
    };
    ($name:literal) => {
        $crate::Symbol::new($name)
    };
    ($name:expr) => {
        $crate::Symbol::new($name)
    };
}

/// Function expression creation (simplified)
#[macro_export]
macro_rules! function {
    // Zero arguments
    ($name:ident) => {
        $crate::Expression::function(stringify!($name), vec![])
    };

    // Variable arguments
    ($name:ident, $($args:expr),+ $(,)?) => {
        $crate::Expression::function(stringify!($name), vec![$($args),+])
    };
}

#[cfg(test)]
mod tests {
    use crate::{Expression, Symbol};

    #[test]
    fn test_literals() {
        assert_eq!(expr!(42), Expression::integer(42));
        assert_eq!(expr!(0), Expression::integer(0));
    }

    #[test]
    fn test_symbols() {
        assert_eq!(expr!(x), Expression::symbol(Symbol::new("x")));
        assert_eq!(expr!(theta), Expression::symbol(Symbol::new("theta")));
    }

    #[test]
    fn test_constants() {
        assert_eq!(expr!(pi), Expression::pi());
        assert_eq!(expr!(e), Expression::e());
        assert_eq!(expr!(i), Expression::i());
    }

    #[test]
    fn test_addition() {
        let result = expr!(x + y);
        assert_eq!(
            result,
            Expression::add(vec![
                Expression::symbol(Symbol::new("x")),
                Expression::symbol(Symbol::new("y"))
            ])
        );
    }

    #[test]
    fn test_multiplication() {
        let result = expr!(x * y);
        assert_eq!(
            result,
            Expression::mul(vec![
                Expression::symbol(Symbol::new("x")),
                Expression::symbol(Symbol::new("y"))
            ])
        );
    }

    #[test]
    fn test_power() {
        let result = expr!(x ^ 2);
        assert_eq!(
            result,
            Expression::pow(Expression::symbol(Symbol::new("x")), Expression::integer(2))
        );
    }

    #[test]
    fn test_division() {
        let result = expr!(x / y);
        assert_eq!(
            result,
            Expression::mul(vec![
                Expression::symbol(Symbol::new("x")),
                Expression::pow(
                    Expression::symbol(Symbol::new("y")),
                    Expression::integer(-1)
                )
            ])
        );
    }

    #[test]
    fn test_negation() {
        let result = expr!(-x);
        assert_eq!(
            result,
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::symbol(Symbol::new("x"))
            ])
        );
    }

    #[test]
    fn test_function_call_no_args() {
        let result = expr!(gamma());
        assert_eq!(result, Expression::function("gamma", vec![]));
    }

    #[test]
    fn test_function_call_one_arg() {
        let result = expr!(sin(x));
        assert_eq!(
            result,
            Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))])
        );
    }

    #[test]
    fn test_function_call_two_args() {
        let result = expr!(log(x, y));
        assert_eq!(
            result,
            Expression::function(
                "log",
                vec![
                    Expression::symbol(Symbol::new("x")),
                    Expression::symbol(Symbol::new("y"))
                ]
            )
        );
    }

    #[test]
    fn test_parenthesized() {
        let result = expr!((x));
        assert_eq!(result, Expression::symbol(Symbol::new("x")));
    }

    #[test]
    fn test_grouped_operations() {
        let result = expr!((x + y) * (x - y));
        // This creates: (x+y) * (x-y)
        assert!(matches!(result, Expression::Mul(_)));
    }

    #[test]
    fn test_multi_term_addition() {
        let result = expr!(add: x, y, z);
        assert_eq!(
            result,
            Expression::add(vec![
                Expression::symbol(Symbol::new("x")),
                Expression::symbol(Symbol::new("y")),
                Expression::symbol(Symbol::new("z"))
            ])
        );
    }

    #[test]
    fn test_multi_factor_multiplication() {
        let result = expr!(mul: 2, x, y);
        assert_eq!(
            result,
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(Symbol::new("x")),
                Expression::symbol(Symbol::new("y"))
            ])
        );
    }
}

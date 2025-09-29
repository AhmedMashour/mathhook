//! Expression construction macros
//!
//! Universal expression creation with natural mathematical syntax.
//! These macros eliminate verbose Expression::add(vec![...]) constructions
//! and provide compile-time optimized expression building.

/// Universal expression creation with natural syntax
///
/// This macro provides ergonomic expression construction for both internal
/// MathHook development and external library users.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::expr;
/// use mathhook_core::{Expression, Symbol};
///
/// // Numbers
/// let num = expr!(42);
/// assert_eq!(num, Expression::integer(42));
///
/// // Symbols  
/// let x = expr!(x);
/// assert_eq!(x, Expression::symbol(Symbol::new("x")));
///
/// // Multi-term addition
/// let sum = expr!(expr!(1), expr!(2), expr!(3));
/// assert_eq!(sum, Expression::add(vec![
///     Expression::integer(1),
///     Expression::integer(2),
///     Expression::integer(3)
/// ]));
///
/// // Multi-factor multiplication
/// let product = expr!(mul: expr!(2), expr!(x), expr!(3));
/// assert_eq!(product, Expression::mul(vec![
///     Expression::integer(2),
///     Expression::symbol(Symbol::new("x")),
///     Expression::integer(3)
/// ]));
///
/// // Power operations
/// let power = expr!(pow: expr!(x), expr!(2));
/// assert_eq!(power, Expression::pow(
///     Expression::symbol(Symbol::new("x")),
///     Expression::integer(2)
/// ));
/// ```
#[macro_export]
macro_rules! expr {
    // Numbers
    ($n:literal) => {
        $crate::Expression::integer($n)
    };

    // Symbols
    ($id:ident) => {
        $crate::Expression::symbol($crate::Symbol::new(stringify!($id)))
    };

    // Multi-term addition - vectorized construction
    ($($terms:expr),+ $(,)?) => {
        $crate::Expression::add(vec![$($terms),+])
    };

    // Multi-factor multiplication - vectorized construction
    (mul: $($factors:expr),+ $(,)?) => {
        $crate::Expression::mul(vec![$($factors),+])
    };

    // Power operations - optimized for common cases
    (pow: $base:expr, $exp:expr) => {
        $crate::Expression::pow($base, $exp)
    };

    // Function calls - compile-time name validation
    (fn: $name:literal, $($args:expr),* $(,)?) => {
        $crate::Expression::function($name, vec![$($args),*])
    };

    // Rational expressions - common pattern
    (rational: $num:expr, $den:expr) => {
        expr!(mul: $num, expr!(pow: $den, expr!(-1)))
    };
}

/// Symbol creation macro
///
/// This macro provides ergonomic symbol creation for mathematical expressions.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::symbol;
/// use mathhook_core::Symbol;
///
/// // Create a symbol
/// let x = symbol!(x);
/// assert_eq!(x, Symbol::new("x"));
///
/// // Create symbol from string
/// let var = symbol!("variable_name");
/// assert_eq!(var, Symbol::new("variable_name"));
/// ```
#[macro_export]
macro_rules! symbol {
    // Symbol from identifier
    ($id:ident) => {
        $crate::Symbol::new(stringify!($id))
    };

    // Symbol from string literal
    ($name:literal) => {
        $crate::Symbol::new($name)
    };

    // Symbol from expression (runtime)
    ($name:expr) => {
        $crate::Symbol::new($name)
    };
}

/// Function expression creation macro
///
/// This macro provides ergonomic function expression creation with
/// automatic argument handling.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{function, expr};
/// use mathhook_core::Expression;
///
/// // Function with no arguments
/// let pi_func = function!(pi);
///
/// // Function with one argument
/// let sin_x = function!(sin, expr!(x));
///
/// // Function with multiple arguments
/// let log_xy = function!(log, expr!(x), expr!(y));
///
/// // Function with vector of arguments
/// let f_args = function!(f, vec![expr!(a), expr!(b), expr!(c)]);
/// ```
#[macro_export]
macro_rules! function {
    // Function with no arguments (constant function)
    ($name:ident) => {
        $crate::Expression::function(stringify!($name), vec![])
    };

    // Function with no arguments from string
    ($name:literal) => {
        $crate::Expression::function($name, vec![])
    };

    // Function with one argument
    ($name:ident, $arg:expr) => {
        $crate::Expression::function(stringify!($name), vec![$arg])
    };

    // Function with one argument from string
    ($name:literal, $arg:expr) => {
        $crate::Expression::function($name, vec![$arg])
    };

    // Function with two arguments
    ($name:ident, $arg1:expr, $arg2:expr) => {
        $crate::Expression::function(stringify!($name), vec![$arg1, $arg2])
    };

    // Function with two arguments from string
    ($name:literal, $arg1:expr, $arg2:expr) => {
        $crate::Expression::function($name, vec![$arg1, $arg2])
    };

    // Function with three arguments
    ($name:ident, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::Expression::function(stringify!($name), vec![$arg1, $arg2, $arg3])
    };

    // Function with three arguments from string
    ($name:literal, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::Expression::function($name, vec![$arg1, $arg2, $arg3])
    };

    // Function with four arguments
    ($name:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::Expression::function(stringify!($name), vec![$arg1, $arg2, $arg3, $arg4])
    };

    // Function with four arguments from string
    ($name:literal, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::Expression::function($name, vec![$arg1, $arg2, $arg3, $arg4])
    };

    // Function with variable number of arguments (vector)
    ($name:ident, $args:expr) => {
        $crate::Expression::function(stringify!($name), $args)
    };

    // Function with variable number of arguments from string
    ($name:literal, $args:expr) => {
        $crate::Expression::function($name, $args)
    };
}

#[cfg(test)]
mod tests {
    use crate::{Expression, Symbol};

    #[test]
    fn test_expr_numbers() {
        let num = expr!(42);
        assert_eq!(num, Expression::integer(42));
    }

    #[test]
    fn test_expr_symbols() {
        let x = expr!(x);
        assert_eq!(x, Expression::symbol(Symbol::new("x")));
    }

    #[test]
    fn test_expr_addition() {
        let sum = expr!(expr!(1), expr!(2), expr!(3));
        assert_eq!(
            sum,
            Expression::add(vec![
                Expression::integer(1),
                Expression::integer(2),
                Expression::integer(3)
            ])
        );
    }

    #[test]
    fn test_expr_multiplication() {
        let product = expr!(mul: expr!(2), expr!(x));
        assert_eq!(
            product,
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(Symbol::new("x"))
            ])
        );
    }

    #[test]
    fn test_expr_power() {
        let power = expr!(pow: expr!(x), expr!(2));
        assert_eq!(
            power,
            Expression::pow(Expression::symbol(Symbol::new("x")), Expression::integer(2))
        );
    }

    #[test]
    fn test_expr_function() {
        let func = expr!(fn: "sin", expr!(x));
        assert_eq!(
            func,
            Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))])
        );
    }

    #[test]
    fn test_expr_rational() {
        let rational = expr!(rational: expr!(1), expr!(x));
        assert_eq!(
            rational,
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(
                    Expression::symbol(Symbol::new("x")),
                    Expression::integer(-1)
                )
            ])
        );
    }

    #[test]
    fn test_symbol_from_ident() {
        let x = symbol!(x);
        assert_eq!(x, Symbol::new("x"));
    }

    #[test]
    fn test_symbol_from_string() {
        let var = symbol!("variable_name");
        assert_eq!(var, Symbol::new("variable_name"));
    }

    #[test]
    fn test_symbol_from_expression() {
        let name = "dynamic_name";
        let sym = symbol!(name);
        assert_eq!(sym, Symbol::new("dynamic_name"));
    }

    #[test]
    fn test_function_no_args() {
        let pi_func = function!(pi);
        assert_eq!(pi_func, Expression::function("pi", vec![]));
    }

    #[test]
    fn test_function_no_args_string() {
        let const_func = function!("constant");
        assert_eq!(const_func, Expression::function("constant", vec![]));
    }

    #[test]
    fn test_function_one_arg() {
        let sin_x = function!(sin, Expression::symbol(Symbol::new("x")));
        assert_eq!(
            sin_x,
            Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))])
        );
    }

    #[test]
    fn test_function_one_arg_string() {
        let func = function!("f", Expression::integer(1));
        assert_eq!(
            func,
            Expression::function("f", vec![Expression::integer(1)])
        );
    }

    #[test]
    fn test_function_two_args() {
        let log_xy = function!(
            log,
            Expression::symbol(Symbol::new("x")),
            Expression::symbol(Symbol::new("y"))
        );
        assert_eq!(
            log_xy,
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
    fn test_function_three_args() {
        let func = function!(
            f,
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3)
        );
        assert_eq!(
            func,
            Expression::function(
                "f",
                vec![
                    Expression::integer(1),
                    Expression::integer(2),
                    Expression::integer(3)
                ]
            )
        );
    }

    #[test]
    fn test_function_four_args() {
        let func = function!(
            f,
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4)
        );
        assert_eq!(
            func,
            Expression::function(
                "f",
                vec![
                    Expression::integer(1),
                    Expression::integer(2),
                    Expression::integer(3),
                    Expression::integer(4)
                ]
            )
        );
    }

    #[test]
    fn test_function_vector_args() {
        let args = vec![
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
        ];
        let func = function!(f, args.clone());
        assert_eq!(func, Expression::function("f", args));
    }

    #[test]
    fn test_function_vector_args_string() {
        let args = vec![Expression::integer(1), Expression::integer(2)];
        let func = function!("custom_func", args.clone());
        assert_eq!(func, Expression::function("custom_func", args));
    }
}

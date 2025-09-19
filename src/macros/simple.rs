//! Simple expression creation macros
//!
//! This module contains the most commonly used macros for basic mathematical expression creation.
//! These are the macros users typically reach for first when building expressions.

/// Create mathematical expressions with natural syntax
///
/// This macro supports **simple expressions** with clear limitations. For complex expressions,
/// use composition or the `parse!` macro.
///
/// # Supported Patterns
///
/// ## Basic Elements
/// ```rust
/// use mathhook::expr;
///
/// let num = expr!(42);                 // Numbers
/// let x = expr!(x);                    // Symbols
/// let pi = expr!(pi);                  // Note: use const_expr!(pi) for the constant
/// ```
///
/// ## Binary Operations (Left-to-Right Only)
/// ```rust
/// use mathhook::expr;
///
/// let add = expr!(x + 2);              // Addition
/// let sub = expr!(x - 1);              // Subtraction  
/// let mul = expr!(3 * x);              // Multiplication
/// let div = expr!(x / 2);              // Division
/// let pow = expr!(x ^ 2);              // Power
/// ```
///
/// ## Simple Functions (1-3 arguments)
/// ```rust
/// use mathhook::expr;
///
/// let sin_x = expr!(sin(x));           // One argument
/// let log_xy = expr!(log(x, y));       // Two arguments  
/// let func = expr!(f(a, b, c));        // Three arguments
/// ```
///
/// ## Simple Parentheses
/// ```rust
/// use mathhook::expr;
///
/// let grouped = expr!((x + 1));        // Single grouping
/// ```
///
/// # Limitations and Workarounds
///
/// ## Complex Expressions - Use Composition
/// ```rust
/// use mathhook::expr;
///
/// // This won't work:
/// // let quad = expr!(a * x^2 + b * x + c);
///
/// // Instead, compose step by step:
/// let x = expr!(x);
/// let a = expr!(a);
/// let b = expr!(b);
/// let c = expr!(c);
/// let x_squared = expr!(x ^ 2);
///
/// // Build the quadratic: a*x^2 + b*x + c
/// let term1 = a * x_squared;           // Use operator overloading
/// let term2 = b * x;
/// let quadratic = term1 + term2 + c;
/// ```
///
/// ## Multiple Operations - Use String Parsing
/// ```rust
/// use mathhook::{expr, parse};
///
/// // This won't work:
/// // let complex = expr!(sin(x) + cos(y) * tan(z));
///
/// // Use the parser for complex expressions:
/// let complex = parse!("sin(x) + cos(y) * tan(z)").unwrap();
/// ```
///
/// ## Nested Parentheses - Combine Approaches
/// ```rust
/// use mathhook::{expr, parse};
///
/// // This won't work:
/// // let nested = expr!((x + 1) * (x - 1));
///
/// // Option 1: Composition
/// let x = expr!(x);
/// let left = x.clone() + expr!(1);
/// let right = x - expr!(1);
/// let result = left * right;
///
/// // Option 2: Parse complex parts
/// let result = parse!("(x + 1) * (x - 1)").unwrap();
/// ```
///
/// # Best Practices
///
/// 1. **Simple expressions**: Use `expr!` for readability
/// 2. **Complex expressions**: Use `parse!` with strings  
/// 3. **Mixed complexity**: Combine `expr!` for simple parts, composition for complex parts
/// 4. **Mathematical constants**: Use `const_expr!(pi)`, `const_expr!(e)`, etc.
///
/// ```rust
/// use mathhook::{expr, const_expr, parse};
///
/// // Good: Simple and clear
/// let simple = expr!(2 * x);
///
/// // Good: Use constants macro
/// let with_pi = const_expr!(pi) * expr!(r ^ 2);
///
/// // Good: Parse complex expressions
/// let complex = parse!("sin(x^2 + 1) / (x - 2)").unwrap();
/// ```
#[macro_export]
macro_rules! expr {
    // Numbers
    ($n:literal) => {
        $crate::core::Expression::integer($n)
    };

    // Symbols
    ($id:ident) => {
        $crate::core::Expression::symbol($crate::core::Symbol::new(stringify!($id)))
    };

    // Addition
    ($left:tt + $right:tt) => {
        $crate::core::Expression::add(vec![expr!($left), expr!($right)])
    };

    // Subtraction (as addition of negative)
    ($left:tt - $right:tt) => {
        $crate::core::Expression::add(vec![
            expr!($left),
            $crate::core::Expression::mul(vec![
                $crate::core::Expression::integer(-1),
                expr!($right),
            ]),
        ])
    };

    // Multiplication
    ($left:tt * $right:tt) => {
        $crate::core::Expression::mul(vec![expr!($left), expr!($right)])
    };

    // Division (as multiplication by inverse)
    ($left:tt / $right:tt) => {
        $crate::core::Expression::mul(vec![
            expr!($left),
            $crate::core::Expression::pow(expr!($right), $crate::core::Expression::integer(-1)),
        ])
    };

    // Power
    ($base:tt ^ $exp:tt) => {
        $crate::core::Expression::pow(expr!($base), expr!($exp))
    };

    // Parentheses
    (($inner:tt)) => {
        expr!($inner)
    };

    // Functions with one argument
    ($func:ident($arg:tt)) => {
        $crate::core::Expression::function(stringify!($func), vec![expr!($arg)])
    };

    // Functions with two arguments
    ($func:ident($arg1:tt, $arg2:tt)) => {
        $crate::core::Expression::function(stringify!($func), vec![expr!($arg1), expr!($arg2)])
    };

    // Functions with three arguments
    ($func:ident($arg1:tt, $arg2:tt, $arg3:tt)) => {
        $crate::core::Expression::function(
            stringify!($func),
            vec![expr!($arg1), expr!($arg2), expr!($arg3)],
        )
    };
}

/// Create mathematical constants with natural syntax
///
/// Use this macro for well-known mathematical constants. For regular symbols,
/// use `expr!(symbol_name)` instead.
///
/// # Available Constants
///
/// ```rust
/// use mathhook::const_expr;
///
/// let pi = const_expr!(pi);        // π ≈ 3.14159...
/// let e = const_expr!(e);          // e ≈ 2.71828... (Euler's number)
/// let i = const_expr!(i);          // i (imaginary unit, √-1)
/// let inf = const_expr!(infinity); // ∞ (infinity)
/// ```
///
/// # Usage in Expressions
///
/// ```rust
/// use mathhook::{expr, const_expr};
///
/// // Circle area: π * r²
/// let r = expr!(r);
/// let area = const_expr!(pi) * (r.clone() * r);
///
/// // Euler's identity: e^(i*π) + 1 = 0
/// let pi = const_expr!(pi);
/// let e = const_expr!(e);
/// let i = const_expr!(i);
/// let identity = e.pow(i * pi) + expr!(1);
/// ```
///
/// # Note: Constants vs Symbols
///
/// ```rust
/// use mathhook::{expr, const_expr};
///
/// // Mathematical constant π
/// let pi_constant = const_expr!(pi);
///
/// // Variable named "pi"
/// let pi_variable = expr!(pi);
///
/// // These are different! One represents the mathematical constant π ≈ 3.14159,
/// // the other represents a variable that could have any value.
/// ```
#[macro_export]
macro_rules! const_expr {
    (pi) => {
        $crate::core::Expression::constant($crate::core::MathConstant::Pi)
    };
    (e) => {
        $crate::core::Expression::constant($crate::core::MathConstant::E)
    };
    (i) => {
        $crate::core::Expression::constant($crate::core::MathConstant::I)
    };
    (infinity) => {
        $crate::core::Expression::constant($crate::core::MathConstant::Infinity)
    };
}

/// Create symbols with minimal syntax
///
/// This macro provides an ergonomic way to create `Symbol` instances, eliminating
/// the repetitive `Symbol::new()` calls throughout your code.
///
/// # Single Symbol
/// ```rust
/// use mathhook::symbol;
///
/// let x = symbol!(x);                    // Symbol::new("x")
/// let alpha = symbol!("α");              // Unicode names
/// let var = symbol!("x_1");              // Complex names
/// ```
///
/// # Multiple Symbols (Tuple Destructuring)
/// ```rust
/// use mathhook::symbol;
///
/// // Create multiple symbols at once
/// let (x, y, z) = symbol!(x, y, z);
/// let (a, b, c) = symbol!(a, b, c);
///
/// // Perfect for mathematical setup
/// let (alpha, beta, gamma) = symbol!(alpha, beta, gamma);
/// ```
///
/// # Usage with Expressions
/// ```rust
/// use mathhook::{symbol, expr};
///
/// let x = symbol!(x);
/// let polynomial = x.clone() * x + expr!(2) * x + expr!(1);
///
/// // Or combine approaches
/// let (a, b, c) = symbol!(a, b, c);
/// let quadratic = a * expr!(x^2) + b * expr!(x) + c;
/// ```
///
/// # Common Use Cases
///
/// ## Test Setup
/// ```rust
/// use mathhook::symbol;
///
/// // Before: Repetitive
/// // let x = Symbol::new("x");
/// // let y = Symbol::new("y");
/// // let z = Symbol::new("z");
///
/// // After: Clean
/// let (x, y, z) = symbol!(x, y, z);
/// assert_eq!(gcd(&x, &y), expected);
/// ```
///
/// ## Mathematical Constants vs Variables
/// ```rust
/// use mathhook::{symbol, const_expr};
///
/// let pi_var = symbol!(pi);          // Variable named "pi"
/// let pi_const = const_expr!(pi);    // Mathematical constant π
/// ```
#[macro_export]
macro_rules! symbol {
    // Single symbol from identifier
    ($name:ident) => {
        $crate::core::Symbol::new(stringify!($name))
    };

    // Single symbol from string literal
    ($name:literal) => {
        $crate::core::Symbol::new($name)
    };

    // Multiple symbols - return tuple
    ($($name:ident),+ $(,)?) => {
        ($(symbol!($name),)+)
    };
}

#[cfg(test)]
mod tests {
    use crate::core::Expression;

    #[test]
    fn test_expr_macro_basic() {
        // Numbers and symbols
        let num = expr!(42);
        let x = expr!(x);

        assert!(matches!(num, Expression::Number(_)));
        assert!(matches!(x, Expression::Symbol(_)));
    }

    #[test]
    fn test_expr_macro_operations() {
        // Basic operations
        let add = expr!(x + 1);
        let mul = expr!(2 * x);
        let pow = expr!(x ^ 2);

        assert!(matches!(add, Expression::Add(_)));
        assert!(matches!(mul, Expression::Mul(_)));
        assert!(matches!(pow, Expression::Pow(_, _)));
    }

    #[test]
    fn test_expr_macro_functions() {
        let sin_x = expr!(sin(x));
        let log_xy = expr!(log(x, y));

        assert!(matches!(sin_x, Expression::Function { .. }));
        assert!(matches!(log_xy, Expression::Function { .. }));
    }

    #[test]
    fn test_const_expr_macro() {
        let pi = const_expr!(pi);
        let e = const_expr!(e);
        let i = const_expr!(i);

        assert!(matches!(pi, Expression::Constant(_)));
        assert!(matches!(e, Expression::Constant(_)));
        assert!(matches!(i, Expression::Constant(_)));
    }

    #[test]
    fn test_symbol_macro_single() {
        use crate::core::Symbol;

        // Test single symbol from identifier
        let x = symbol!(x);
        assert_eq!(x, Symbol::new("x"));

        // Test single symbol from string literal
        let alpha = symbol!("α");
        assert_eq!(alpha, Symbol::new("α"));

        let complex_name = symbol!("x_1");
        assert_eq!(complex_name, Symbol::new("x_1"));
    }

    #[test]
    fn test_symbol_macro_multiple() {
        use crate::core::Symbol;

        // Test tuple destructuring with multiple symbols
        let (x, y, z) = symbol!(x, y, z);
        assert_eq!(x, Symbol::new("x"));
        assert_eq!(y, Symbol::new("y"));
        assert_eq!(z, Symbol::new("z"));

        // Test with trailing comma
        let (a, b) = symbol!(a, b,);
        assert_eq!(a, Symbol::new("a"));
        assert_eq!(b, Symbol::new("b"));
    }

    #[test]
    fn test_symbol_macro_usage() {
        use crate::core::Symbol;

        // Test usage in mathematical context
        let (x, y) = symbol!(x, y);
        let expr_x = Expression::symbol(x.clone());
        let expr_y = Expression::symbol(y.clone());

        assert!(matches!(expr_x, Expression::Symbol(_)));
        assert!(matches!(expr_y, Expression::Symbol(_)));

        // Test that symbols are equal to manually created ones
        assert_eq!(x, Symbol::new("x"));
        assert_eq!(y, Symbol::new("y"));
    }
}

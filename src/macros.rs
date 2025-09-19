//! Mathematical expression macros
//!
//! This module provides ergonomic macros for creating, parsing, and manipulating mathematical expressions.
//! The macros are organized into logical categories:
//!
//! - [`expression`]: Core expression creation (`expr!`, `const_expr!`)
//! - [`parsing`]: Format parsing and conversion (`parse!`, `to_format!`)
//! - [`calculus`]: Calculus operations (`calculus!`)

pub mod expression {
    //! Expression creation macros
    //!
    //! Provides ergonomic macros for creating mathematical expressions and constants.

    /// Create mathematical expressions with natural syntax
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::expr;
    ///
    /// // Simple expressions
    /// let x = expr!(x);                    // Symbol
    /// let num = expr!(42);                 // Number
    /// let add = expr!(x + 2);              // Addition
    /// let mul = expr!(3 * x);              // Multiplication
    /// let pow = expr!(x ^ 2);              // Power
    ///
    /// // Complex expressions
    /// let quad = expr!(a * x^2 + b * x + c);
    /// let frac = expr!((x + 1) / (x - 1));
    ///
    /// // Functions
    /// let sin_x = expr!(sin(x));
    /// let log_xy = expr!(log(x, y));
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
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::const_expr;
    ///
    /// let pi = const_expr!(pi);        // Pi constant
    /// let e = const_expr!(e);          // Euler's number
    /// let i = const_expr!(i);          // Imaginary unit
    /// let inf = const_expr!(infinity); // Infinity
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
    }
}

pub mod parsing {
    //! Parsing and format conversion macros
    //!
    //! Provides ergonomic macros for parsing mathematical expressions and converting between formats.

    /// Parse mathematical expressions from strings with format detection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::parse;
    ///
    /// // Auto-detect format
    /// let expr1 = parse!("x^2 + 2*x + 1");           // Simple format
    /// let expr2 = parse!("\\frac{x}{y}");            // LaTeX format  
    /// let expr3 = parse!("Times[x, y]");             // Wolfram format
    ///
    /// // Explicit format
    /// let latex_expr = parse!(latex: "\\sin(x)");
    /// let wolfram_expr = parse!(wolfram: "Sin[x]");
    /// let simple_expr = parse!(simple: "sin(x)");
    /// ```
    #[macro_export]
    macro_rules! parse {
        // Auto-detect format
        ($input:expr) => {{
            let mut parser = $crate::parsing::UniversalParser::new();
            parser.parse($input)
        }};

        // Explicit LaTeX format
        (latex: $input:expr) => {{
            let mut parser = $crate::parsing::UniversalParser::new();
            parser.parse_with_language($input, $crate::parsing::MathLanguage::LaTeX)
        }};

        // Explicit Wolfram format
        (wolfram: $input:expr) => {{
            let mut parser = $crate::parsing::UniversalParser::new();
            parser.parse_with_language($input, $crate::parsing::MathLanguage::Wolfram)
        }};

        // Explicit Simple format
        (simple: $input:expr) => {{
            let mut parser = $crate::parsing::UniversalParser::new();
            parser.parse_with_language($input, $crate::parsing::MathLanguage::Simple)
        }};
    }

    /// Convert expressions to different output formats
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::{expr, to_format};
    ///
    /// let expr = expr!(x^2 + 1);
    ///
    /// let simple_output = to_format!(simple: expr);   // "x^2 + 1"
    /// let latex_output = to_format!(latex: expr);     // "x^{2} + 1"
    /// let wolfram_output = to_format!(wolfram: expr); // "Plus[Power[x, 2], 1]"
    /// ```
    #[macro_export]
    macro_rules! to_format {
        // Simple format
        (simple: $expr:expr) => {{
            let parser = $crate::parsing::UniversalParser::new();
            parser.to_simple(&$expr)
        }};

        // LaTeX format
        (latex: $expr:expr) => {{
            let parser = $crate::parsing::UniversalParser::new();
            parser.to_latex(&$expr)
        }};

        // Wolfram format
        (wolfram: $expr:expr) => {{
            let parser = $crate::parsing::UniversalParser::new();
            parser.to_wolfram(&$expr)
        }};
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_parse_macro() {
            let result = parse!("x + 1");
            assert!(result.is_ok());

            let latex_result = parse!(latex: "\\frac{1}{2}");
            assert!(latex_result.is_ok());
        }
    }
}

pub mod calculus {
    //! Calculus operation macros
    //!
    //! Provides ergonomic macros for creating calculus expressions like derivatives, integrals, limits, and series.

    /// Create calculus expressions with natural syntax
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::{expr, calculus};
    ///
    /// let x = expr!(x);
    /// let f = expr!(x^2 + 1);
    ///
    /// // Derivatives
    /// let df_dx = calculus!(derivative: f, x);           // First derivative
    /// let d2f_dx2 = calculus!(derivative: f, x, 2);      // Second derivative
    /// let df_dx_3 = calculus!(derivative: f, x, 3);      // Third derivative
    ///
    /// // Integrals
    /// let indefinite = calculus!(integral: f, x);        // Indefinite integral
    /// let definite = calculus!(integral: f, x, 0, 1);    // Definite integral from 0 to 1
    ///
    /// // Limits
    /// let limit = calculus!(limit: f, x, 0);             // Limit as x approaches 0
    ///
    /// // Sums and products
    /// let sum = calculus!(sum: f, i, 1, n);              // Sum from i=1 to n
    /// let product = calculus!(product: f, i, 1, n);      // Product from i=1 to n
    /// ```
    #[macro_export]
    macro_rules! calculus {
        // First derivative: derivative(f, x)
        (derivative: $expr:expr, $var:ident) => {
            $crate::core::Expression::derivative(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                1,
            )
        };

        // Higher order derivative: derivative(f, x, n)
        (derivative: $expr:expr, $var:ident, $order:expr) => {
            $crate::core::Expression::derivative(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                $order,
            )
        };

        // Indefinite integral: integral(f, x)
        (integral: $expr:expr, $var:ident) => {
            $crate::core::Expression::integral(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                None,
            )
        };

        // Definite integral: integral(f, x, a, b)
        (integral: $expr:expr, $var:ident, $start:expr, $end:expr) => {
            $crate::core::Expression::integral(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                Some((Box::new($start), Box::new($end))),
            )
        };

        // Limit: limit(f, x, a)
        (limit: $expr:expr, $var:ident, $approach:expr) => {
            $crate::core::Expression::limit(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                $approach,
                $crate::core::LimitDirection::Both,
            )
        };

        // Sum: sum(expr, i, start, end)
        (sum: $expr:expr, $var:ident, $start:expr, $end:expr) => {
            $crate::core::Expression::sum(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                $start,
                $end,
            )
        };

        // Product: product(expr, i, start, end)
        (product: $expr:expr, $var:ident, $start:expr, $end:expr) => {
            $crate::core::Expression::product(
                $expr,
                $crate::core::Symbol::new(stringify!($var)),
                $start,
                $end,
            )
        };
    }
}

// Re-export all macros at the crate root for convenient access
pub use calculus::*;
pub use expression::*;
pub use parsing::*;

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
        $crate::core::Expression::derivative($expr, $crate::core::Symbol::new(stringify!($var)), 1)
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
        $crate::core::Expression::integral($expr, $crate::core::Symbol::new(stringify!($var)))
    };

    // Definite integral: integral(f, x, a, b)
    (integral: $expr:expr, $var:ident, $start:expr, $end:expr) => {
        $crate::core::Expression::definite_integral(
            $expr,
            $crate::core::Symbol::new(stringify!($var)),
            $start,
            $end,
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

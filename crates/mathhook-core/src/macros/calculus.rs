//! Calculus operation macros
//!
//! High-performance calculus operations including derivatives, integrals,
//! limits, and series expansions with compile-time optimizations for
//! common mathematical patterns.

// Macros are available through crate re-exports - no explicit import needed

/// High-performance calculus operations
///
/// This macro provides ergonomic calculus operations with compile-time
/// optimizations for common derivatives and mathematical patterns.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{calculus, expr};
/// use mathhook_core::{Expression, Symbol};
///
/// let x = Symbol::new("x");
/// let x_expr = expr!(x);
///
/// // Derivative construction
/// let derivative = calculus!(derivative: expr!(fn: "sin", x_expr.clone()), x.clone(), 1);
///
/// // Integral construction  
/// let integral = calculus!(integral: x_expr.clone(), x.clone());
///
/// // Definite integral
/// let definite = calculus!(definite: x_expr.clone(), x.clone(), expr!(0), expr!(1));
///
/// // Limit construction
/// let limit = calculus!(limit: x_expr.clone(), x.clone(), expr!(0));
///
/// // Series expansion
/// let series = calculus!(series: expr!(fn: "sin", x_expr.clone()), x.clone(), expr!(0), 5);
///
/// // Common derivatives - compile-time optimized
/// let sin_derivative = calculus!(d/dx: sin(x_expr.clone()));
/// let cos_derivative = calculus!(d/dx: cos(x_expr.clone()));
/// let exp_derivative = calculus!(d/dx: exp(x_expr.clone()));
/// let ln_derivative = calculus!(d/dx: ln(x_expr.clone()));
/// ```
#[macro_export]
macro_rules! calculus {
    // Derivative construction
    (derivative: $expr:expr, $var:expr, $order:expr) => {
        $crate::Expression::derivative($expr, $var, $order)
    };

    // Integral construction
    (integral: $expr:expr, $var:expr) => {
        $crate::Expression::integral($expr, $var)
    };

    // Definite integral
    (definite: $expr:expr, $var:expr, $lower:expr, $upper:expr) => {
        $crate::Expression::definite_integral($expr, $var, $lower, $upper)
    };

    // Limit construction
    (limit: $expr:expr, $var:expr, $point:expr) => {
        $crate::Expression::limit($expr, $var, $point)
    };

    // Series expansion
    // (series: $expr:expr, $var:expr, $point:expr, $order:expr) => {
    //     $crate::Expression::series($expr, $var, $point, $order)
    // };

    // Common derivatives - compile-time optimization
    (d/dx: sin($x:expr)) => {
        $crate::Expression::function("cos", vec![$x])
    };

    (d/dx: cos($x:expr)) => {
        $crate::Expression::mul(vec![
            $crate::Expression::integer(-1),
            $crate::Expression::function("sin", vec![$x])
        ])
    };

    (d/dx: exp($x:expr)) => {
        $crate::Expression::function("exp", vec![$x])
    };

    (d/dx: ln($x:expr)) => {
        $crate::Expression::mul(vec![
            $crate::Expression::integer(1),
            $crate::Expression::pow($x, $crate::Expression::integer(-1))
        ])
    };

    (d/dx: pow($base:expr, $n:literal)) => {
        expr!(mul: expr!($n), expr!(pow: $base, expr!($n - 1)))
    };

    // Integration by parts: ∫u dv = uv - ∫v du
    (by_parts: $u:expr, $dv:expr, $var:expr) => {
        expr!(
            expr!(mul: $u, calculus!(integral: $dv, $var)),
            expr!(mul: expr!(-1), calculus!(integral:
                expr!(mul: calculus!(derivative: $u, $var, 1), $dv), $var))
        )
    };

    // Chain rule: d/dx[f(g(x))] = f'(g(x)) * g'(x)
    (chain_rule: $f:expr, $g:expr, $var:expr) => {
        expr!(mul:
            calculus!(derivative: $f, $g, 1),
            calculus!(derivative: $g, $var, 1)
        )
    };

    // Product rule: d/dx[f(x)g(x)] = f'(x)g(x) + f(x)g'(x)
    (product_rule: $f:expr, $g:expr, $var:expr) => {
        expr!(
            expr!(mul: calculus!(derivative: $f, $var, 1), $g),
            expr!(mul: $f, calculus!(derivative: $g, $var, 1))
        )
    };

    // Quotient rule: d/dx[f(x)/g(x)] = [f'(x)g(x) - f(x)g'(x)] / [g(x)]²
    (quotient_rule: $f:expr, $g:expr, $var:expr) => {
        expr!(rational:
            expr!(
                expr!(mul: calculus!(derivative: $f, $var, 1), $g),
                expr!(mul: expr!(-1), $f, calculus!(derivative: $g, $var, 1))
            ),
            expr!(pow: $g, expr!(2))
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::{Expression, Symbol};

    #[test]
    fn test_calculus_derivative() {
        let x = Symbol::new("x");
        let x_expr = Expression::symbol(x.clone());
        let derivative = calculus!(derivative: x_expr, x, 1);

        // Should create a derivative expression
        match derivative {
            Expression::Calculus(_) => (),
            _ => panic!("Expected calculus expression"),
        }
    }

    #[test]
    fn test_calculus_integral() {
        let x = Symbol::new("x");
        let x_expr = Expression::symbol(x.clone());
        let integral = calculus!(integral: x_expr, x);

        // Should create an integral expression
        match integral {
            Expression::Calculus(_) => (),
            _ => panic!("Expected calculus expression"),
        }
    }

    #[test]
    fn test_calculus_limit() {
        let x = Symbol::new("x");
        let x_expr = Expression::symbol(x.clone());
        let limit = calculus!(limit: x_expr, x, Expression::integer(0));

        // Should create a limit expression
        match limit {
            Expression::Calculus(_) => (),
            _ => panic!("Expected calculus expression"),
        }
    }

    #[test]
    fn test_calculus_series() {
        let x = Symbol::new("x");
        let x_expr = Expression::symbol(x.clone());
        let series = calculus!(series: x_expr, x, Expression::integer(0), 5);

        // Should create a series expression
        match series {
            Expression::Calculus(_) => (),
            _ => panic!("Expected calculus expression"),
        }
    }

    #[test]
    fn test_calculus_common_derivatives() {
        let x_expr = Expression::symbol(Symbol::new("x"));

        // Test sin derivative
        let sin_deriv = calculus!(d/dx: sin(x_expr.clone()));
        assert_eq!(sin_deriv, Expression::function("cos", vec![x_expr.clone()]));

        // Test cos derivative
        let cos_deriv = calculus!(d/dx: cos(x_expr.clone()));
        assert_eq!(
            cos_deriv,
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("sin", vec![x_expr.clone()])
            ])
        );

        // Test exp derivative
        let exp_deriv = calculus!(d/dx: exp(x_expr.clone()));
        assert_eq!(exp_deriv, Expression::function("exp", vec![x_expr.clone()]));
    }

    #[test]
    fn test_calculus_rules() {
        let x = Symbol::new("x");
        let f = Expression::function("f", vec![Expression::symbol(x.clone())]);
        let g = Expression::function("g", vec![Expression::symbol(x.clone())]);

        // Test product rule
        let product = calculus!(product_rule: f.clone(), g.clone(), x.clone());

        // Should be an addition of two multiplication terms
        match product {
            Expression::Add(_) => (),
            _ => panic!("Expected addition expression for product rule"),
        }
    }
}

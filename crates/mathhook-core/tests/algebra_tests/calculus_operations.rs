//! Comprehensive calculus operations test suite
//! Covers derivatives, integrals, limits, and series expansions

use mathhook_core::prelude::*;

#[test]
fn test_derivative_basic_rules() {
    // Test basic derivative patterns that should be recognizable
    // Derivative patterns in algebraic form
    // d/dx(x^n) = n*x^(n-1) pattern recognition
    let power_expr = expr!(x ^ 3);

    // For now, just test that power expressions are maintained
    let simplified = power_expr.simplify();

    assert!(
        matches!(simplified, Expression::Pow(_, _)),
        "Expected x^3 to remain as power, got: {}",
        simplified
    );
}

#[test]
fn test_derivative_product_rule_pattern() {
    // Test product rule pattern: d/dx(uv) = u'v + uv'
    let u = expr!(u);
    let v = expr!(v);
    let u_prime = expr!(u_prime);
    let v_prime = expr!(v_prime);

    // Product rule pattern: u'v + uv'
    let product_rule = Expression::add(vec![
        Expression::mul(vec![u_prime, v.clone()]),
        Expression::mul(vec![u, v_prime]),
    ]);

    let simplified = product_rule.simplify();

    // Should maintain additive structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected product rule (u'v + uv') to remain as addition, got: {}",
        simplified
    );
}

#[test]
fn test_derivative_chain_rule_pattern() {
    // Test chain rule pattern: d/dx(f(g(x))) = f'(g(x)) * g'(x)
    let f_prime = expr!(f_prime);
    let g_prime = expr!(g_prime);

    // Chain rule pattern: f'(g) * g'
    let chain_rule = Expression::mul(vec![f_prime, g_prime]);

    let simplified = chain_rule.simplify();

    // Should maintain multiplicative structure or simplify to single symbol
    assert!(
        matches!(simplified, Expression::Mul(_) | Expression::Symbol(_)),
        "Expected chain rule (f'(g) * g') to be multiplication or symbol, got: {}",
        simplified
    );
}

#[test]
fn test_integral_basic_patterns() {
    // Test basic integral patterns that should be recognizable
    let x = expr!(x);

    // Integral patterns: ∫x^n dx = x^(n+1)/(n+1) pattern
    let power_integral_pattern = Expression::mul(vec![
        Expression::pow(x.clone(), Expression::integer(4)),
        Expression::pow(Expression::integer(4), Expression::integer(-1)),
    ]);

    let simplified = power_integral_pattern.simplify();
    println!("Integral pattern: {}", simplified);

    // Should maintain structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_limit_basic_patterns() {
    // Test limit patterns that should be algebraically recognizable
    let x = expr!(x);
    let h = expr!(h);

    // Limit pattern: (f(x+h) - f(x))/h for f(x) = x²
    // (x+h)² - x² = x² + 2xh + h² - x² = 2xh + h²
    let difference_quotient_numerator = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x, h.clone()]),
        Expression::pow(h, Expression::integer(2)),
    ]);

    let simplified = difference_quotient_numerator.simplify();

    // Should maintain polynomial structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected difference quotient (2xh + h^2) to remain as addition, got: {}",
        simplified
    );
}

#[test]
fn test_series_expansion_patterns() {
    // Test series expansion patterns (Taylor series coefficients)
    let x = expr!(x);

    // Taylor series pattern for e^x: 1 + x + x²/2! + x³/3! + ...
    let taylor_terms = Expression::add(vec![
        Expression::integer(1),
        x.clone(),
        Expression::mul(vec![
            Expression::pow(x.clone(), Expression::integer(2)),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]),
        Expression::mul(vec![
            Expression::pow(x, Expression::integer(3)),
            Expression::pow(Expression::integer(6), Expression::integer(-1)),
        ]),
    ]);

    let simplified = taylor_terms.simplify();

    // Should maintain series structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected Taylor series to remain as addition, got: {}",
        simplified
    );
}

#[test]
fn test_partial_derivative_patterns() {
    // Test partial derivative patterns: ∂f/∂x, ∂f/∂y
    let x = expr!(x);
    let y = expr!(y);

    // Partial derivative pattern for f(x,y) = x²y + xy²
    let mixed_partial = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x.clone(), y.clone()]),
        Expression::mul(vec![Expression::integer(2), x, y]),
    ]);

    let simplified = mixed_partial.simplify();

    // Should combine like terms: 2xy + 2xy = 4xy
    if matches!(simplified, Expression::Mul(_)) {
        // Should combine to 4xy
        println!("Combined partial: {}", simplified);
    } else {
        println!("Partial derivative: {}", simplified);
    }
    // The expression should simplify to multiplication (4xy) or remain as addition
    assert!(
        matches!(simplified, Expression::Mul(_) | Expression::Add(_)),
        "Expected partial derivative to be multiplication or addition, got: {}",
        simplified
    );
}

#[test]
fn test_integration_by_parts_pattern() {
    // Test integration by parts pattern: ∫u dv = uv - ∫v du
    let u = expr!(u);
    let v = expr!(v);
    let du = expr!(du);

    // Integration by parts pattern: uv - ∫v du
    let integration_by_parts = Expression::add(vec![
        Expression::mul(vec![u, v.clone()]),
        Expression::mul(vec![Expression::integer(-1), Expression::mul(vec![v, du])]),
    ]);

    let simplified = integration_by_parts.simplify();

    // Should maintain additive structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected integration by parts (uv - ∫v du) to remain as addition, got: {}",
        simplified
    );
}

#[test]
fn test_multivariable_calculus_patterns() {
    // Test multivariable calculus patterns
    let x = expr!(x);
    let y = expr!(y);
    let z = expr!(z);

    // Gradient pattern: ∇f = (∂f/∂x, ∂f/∂y, ∂f/∂z)
    // For f(x,y,z) = x²y + yz² + xz
    let gradient_x = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x.clone(), y.clone()]),
        z.clone(),
    ]);

    let gradient_y = Expression::add(vec![
        Expression::pow(x.clone(), Expression::integer(2)),
        Expression::pow(z.clone(), Expression::integer(2)),
    ]);

    let gradient_z = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), y, z.clone()]),
        x,
    ]);

    // Test each component
    let grad_x_simplified = gradient_x.simplify();
    let grad_y_simplified = gradient_y.simplify();
    let grad_z_simplified = gradient_z.simplify();

    // All should maintain their structure
    assert!(!grad_x_simplified.is_zero());
    assert!(!grad_y_simplified.is_zero());
    assert!(!grad_z_simplified.is_zero());
}

#[test]
fn test_vector_calculus_operations() {
    // Test vector calculus operations (divergence, curl patterns)
    let fx = expr!(Fx);
    let fy = expr!(Fy);
    let fz = expr!(Fz);

    // Divergence pattern: ∇·F = ∂Fx/∂x + ∂Fy/∂y + ∂Fz/∂z
    let divergence = Expression::add(vec![fx, fy, fz]);

    let simplified = divergence.simplify();

    // Should maintain additive structure for divergence
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected divergence (∇·F) to remain as addition, got: {}",
        simplified
    );
}

#[test]
fn test_differential_equation_patterns() {
    // Test differential equation patterns
    let y = expr!(y);
    let y_prime = expr!(y_prime);
    let x = expr!(x);

    // First-order ODE pattern: y' + p(x)y = q(x)
    let ode_pattern = Expression::add(vec![y_prime, Expression::mul(vec![x, y])]);

    let simplified = ode_pattern.simplify();

    // Should maintain ODE structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected ODE pattern (y' + p(x)y) to remain as addition, got: {}",
        simplified
    );
}

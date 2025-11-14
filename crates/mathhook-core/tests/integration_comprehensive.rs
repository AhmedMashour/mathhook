//! Comprehensive integration tests covering all strategy layers
//!
//! Tests combinations across Waves 2-5 (rational, trig, substitution, Risch)
//! and validates fallthrough behavior when strategies fail.

use mathhook_core::calculus::integrals::strategy::integrate_with_strategy;
use mathhook_core::core::expression::data_types::CalculusData;
use mathhook_core::{symbol, Expression};

// Helper function to detect symbolic integral
fn is_symbolic_integral(expr: &Expression) -> bool {
    match expr {
        Expression::Calculus(data) => matches!(**data, CalculusData::Integral { .. }),
        _ => false,
    }
}

// Wave 2 (Rational) + Wave 3 (Table Lookup) Combinations

#[test]
fn test_rational_plus_polynomial() {
    let x = symbol!(x);

    // ∫(1/(x+1) + x^2) dx = ln|x+1| + x^3/3
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
                Expression::integer(-1),
            ),
        ]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_rational_decomposition_multiple_terms() {
    let x = symbol!(x);

    // ∫(1/(x^2-1)) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(-1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Wave 3 (Substitution) + Wave 4 (Trigonometric) Combinations

#[test]
fn test_substitution_with_trig_inside() {
    let x = symbol!(x);

    // ∫sin(x)*cos(x) dx
    let expr = Expression::mul(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_power_reduction() {
    let x = symbol!(x);

    // ∫sin^2(x) dx
    let expr = Expression::pow(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::integer(2),
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_power_reduction_cos() {
    let x = symbol!(x);

    // ∫cos^2(x) dx
    let expr = Expression::pow(
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
        Expression::integer(2),
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_product_sin_cos_different_powers() {
    let x = symbol!(x);

    // ∫sin^3(x)*cos(x) dx
    let expr = Expression::mul(vec![
        Expression::pow(
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::integer(3),
        ),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Wave 4 (Trigonometric) + Wave 5 (Risch) Combinations

#[test]
fn test_trig_exponential_product() {
    let x = symbol!(x);

    // ∫e^x * sin(x) dx
    let expr = Expression::mul(vec![
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_over_polynomial() {
    let x = symbol!(x);

    // ∫sin(x)/x dx - non-elementary (sine integral)
    let expr = Expression::mul(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);

    let _result = integrate_with_strategy(&expr, x.clone(), 0);
    // Non-elementary, may return symbolic
}

// Wave 2 (Rational) + Wave 5 (Risch) Combinations

#[test]
fn test_rational_times_exponential() {
    let x = symbol!(x);

    // ∫(1/x)*e^x dx
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
    ]);

    let _result = integrate_with_strategy(&expr, x.clone(), 0);
    // Non-elementary (exponential integral)
}

#[test]
fn test_rational_times_logarithm() {
    let x = symbol!(x);

    // ∫ln(x)/x dx
    let expr = Expression::mul(vec![
        Expression::function("ln", vec![Expression::symbol(x.clone())]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Complex Multi-Wave Combinations

#[test]
fn test_rational_plus_trig_plus_polynomial() {
    let x = symbol!(x);

    // ∫(1/(x+1) + sin(x) + x^2) dx
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
                Expression::integer(-1),
            ),
        ]),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_nested_substitution_candidate() {
    let x = symbol!(x);

    // ∫x * sin(x^2) dx
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function(
            "sin",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        ),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_chain_rule_pattern() {
    let x = symbol!(x);

    // ∫2x * e^(x^2) dx
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(x.clone()),
        Expression::function(
            "exp",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        ),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Edge Cases Spanning Multiple Strategies

#[test]
fn test_constant_times_integrable() {
    let x = symbol!(x);

    // ∫5*sin(x) dx
    let expr = Expression::mul(vec![
        Expression::integer(5),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_sum_of_different_strategy_terms() {
    let x = symbol!(x);

    // ∫(x + sin(x) + e^x) dx
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_rational_with_quadratic_denominator() {
    let x = symbol!(x);

    // ∫1/(x^2 + 1) dx = arctan(x)
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_rational_with_linear_numerator() {
    let x = symbol!(x);

    // ∫(2x + 3)/(x^2 + 3x + 2) dx
    let numerator = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(2),
    ]);
    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Fallthrough Behavior Tests

#[test]
fn test_fallthrough_table_to_rational() {
    let x = symbol!(x);

    // ∫1/(x^2 + 2x + 1) dx = ∫1/(x+1)^2 dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_fallthrough_substitution_to_trig() {
    let x = symbol!(x);

    // ∫tan(x) dx
    let expr = Expression::function("tan", vec![Expression::symbol(x.clone())]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_fallthrough_all_to_risch() {
    let x = symbol!(x);

    // ∫e^(e^x) dx - non-elementary
    let expr = Expression::function(
        "exp",
        vec![Expression::function("exp", vec![Expression::symbol(x.clone())])],
    );

    let _result = integrate_with_strategy(&expr, x.clone(), 0);
    // Non-elementary, Risch should recognize
}

// Performance Edge Cases

#[test]
fn test_large_polynomial_sum() {
    let x = symbol!(x);

    // ∫(x^10 + x^9 + ... + x + 1) dx
    let terms: Vec<Expression> = (0..=10)
        .map(|i| Expression::pow(Expression::symbol(x.clone()), Expression::integer(i)))
        .collect();
    let expr = Expression::add(terms);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_product_of_many_constants() {
    let x = symbol!(x);

    // ∫(2*3*4*5*x) dx
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::integer(3),
        Expression::integer(4),
        Expression::integer(5),
        Expression::symbol(x.clone()),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Zero and Special Value Cases

#[test]
fn test_integrate_zero() {
    let x = symbol!(x);

    // ∫0 dx
    let expr = Expression::integer(0);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_integrate_one() {
    let x = symbol!(x);

    // ∫1 dx = x
    let expr = Expression::integer(1);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Rational Function Special Cases

#[test]
fn test_proper_rational_function() {
    let x = symbol!(x);

    // ∫x/(x^2 + 1) dx
    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_improper_rational_function() {
    let x = symbol!(x);

    // ∫(x^3)/(x^2 + 1) dx
    let numerator = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Trigonometric Special Cases

#[test]
fn test_trig_secant_squared() {
    let x = symbol!(x);

    // ∫sec^2(x) dx = tan(x)
    let expr = Expression::pow(
        Expression::function("sec", vec![Expression::symbol(x.clone())]),
        Expression::integer(2),
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_sin_cubed() {
    let x = symbol!(x);

    // ∫sin^3(x) dx
    let expr = Expression::pow(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::integer(3),
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_product_even_powers() {
    let x = symbol!(x);

    // ∫sin^2(x)*cos^2(x) dx
    let expr = Expression::mul(vec![
        Expression::pow(
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::integer(2),
        ),
        Expression::pow(
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
            Expression::integer(2),
        ),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Exponential and Logarithm Cases

#[test]
fn test_exponential_polynomial_product() {
    let x = symbol!(x);

    // ∫x^2 * e^x dx
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_logarithm_alone() {
    let x = symbol!(x);

    // ∫ln(x) dx
    let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Substitution Pattern Recognition

#[test]
fn test_substitution_sqrt_linear() {
    let x = symbol!(x);

    // ∫sqrt(x + 1) dx
    let expr = Expression::function(
        "sqrt",
        vec![Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ])],
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_substitution_trig_argument() {
    let x = symbol!(x);

    // ∫cos(2x) dx
    let expr = Expression::function(
        "cos",
        vec![Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
        ])],
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Mixed Strategy Combinations

#[test]
fn test_rational_plus_exponential() {
    let x = symbol!(x);

    // ∫(1/x + e^x) dx
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_trig_plus_polynomial() {
    let x = symbol!(x);

    // ∫(sin(x) + x^3) dx
    let expr = Expression::add(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_product_requiring_parts_and_substitution() {
    let x = symbol!(x);

    // ∫x * ln(x) dx
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("ln", vec![Expression::symbol(x.clone())]),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Complex Rational Functions

#[test]
fn test_partial_fractions_three_factors() {
    let x = symbol!(x);

    // ∫1/((x-1)(x-2)(x-3)) dx
    let denominator = Expression::mul(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-3)]),
    ]);
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_rational_with_repeated_factor() {
    let x = symbol!(x);

    // ∫1/(x^2*(x+1)) dx
    let denominator = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
    ]);
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

// Boundary and Limit Cases

#[test]
fn test_very_simple_polynomial() {
    let x = symbol!(x);

    // ∫x dx
    let expr = Expression::symbol(x.clone());

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_negative_power() {
    let x = symbol!(x);

    // ∫x^(-3) dx
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-3));

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

#[test]
fn test_fractional_power() {
    let x = symbol!(x);

    // ∫x^(1/2) dx
    use mathhook_core::core::Number;
    use num_rational::BigRational;
    let expr = Expression::pow(
        Expression::symbol(x.clone()),
        Expression::Number(Number::rational(BigRational::new(1.into(), 2.into()))),
    );

    let result = integrate_with_strategy(&expr, x.clone(), 0);
    assert!(!is_symbolic_integral(&result));
}

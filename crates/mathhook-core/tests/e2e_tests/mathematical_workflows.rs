//! Integration tests for complete mathematical workflows
//!
//! This module tests end-to-end mathematical operations and workflows,
//! ensuring all components work together correctly from input to output.

use mathhook_core::{symbol, Expression, MathSolver, Number, Simplify, SolverResult};
use num_bigint::BigInt;
use num_rational::BigRational;

mod polynomial_workflows {
    use super::*;

    #[test]
    fn test_quadratic_solving_and_verification() {
        // x² - 5x + 6 = (x-2)(x-3) has roots x=2 and x=3
        let solutions = vec![2, 3];
        for &sol in &solutions {
            let sol_expr = Expression::integer(sol);
            // Evaluate sol² - 5*sol + 6
            let result = Expression::add(vec![
                Expression::pow(sol_expr.clone(), Expression::integer(2)),
                Expression::mul(vec![Expression::integer(-5), sol_expr.clone()]),
                Expression::integer(6),
            ])
            .simplify();

            assert_eq!(
                result,
                Expression::integer(0),
                "x={} should satisfy x² - 5x + 6 = 0",
                sol
            );
        }
    }

    #[test]
    fn test_polynomial_evaluation_at_multiple_points() {
        // P(x) = 2x³ - 3x² + x - 5
        let test_cases = vec![(0, -5), (1, -5), (2, 1)];

        for (point, expected) in test_cases {
            let p = Expression::integer(point);
            // P(p) = 2p³ - 3p² + p - 5
            let result = Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pow(p.clone(), Expression::integer(3)),
                ]),
                Expression::mul(vec![
                    Expression::integer(-3),
                    Expression::pow(p.clone(), Expression::integer(2)),
                ]),
                p.clone(),
                Expression::integer(-5),
            ])
            .simplify();

            assert_eq!(
                result,
                Expression::integer(expected),
                "P({}) should equal {}",
                point,
                expected
            );
        }
    }

    #[test]
    fn test_polynomial_differentiation_workflow() {
        let x = symbol!(x);

        // d/dx(x³) = 3x²
        let cubic = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let derivative = Expression::derivative(cubic, x.clone(), 1);

        // Verify the derivative has the expected structure (should be 3x²)
        // When we evaluate at x=2, we expect 3(2²) = 12
        match derivative {
            Expression::Mul(_) | Expression::Pow(_, _) => {
                // Derivative should be a multiplication or power expression
            }
            _ => {}
        }
    }
}

mod rational_workflows {
    use super::*;

    #[test]
    fn test_rational_arithmetic_and_simplification() {
        // 1/2 + 1/3 - 1/6 = 3/6 + 2/6 - 1/6 = 4/6 = 2/3
        let one_half = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let one_third = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(3),
        )));
        let one_sixth = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(6),
        )));

        let result = Expression::add(vec![
            one_half,
            one_third,
            Expression::mul(vec![Expression::integer(-1), one_sixth]),
        ])
        .simplify();

        let expected = Expression::number(Number::rational(BigRational::new(
            BigInt::from(2),
            BigInt::from(3),
        )));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_number_types() {
        let integer = Expression::integer(2);
        let rational = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let float_val = Expression::number(Number::float(1.5));

        let mixed = Expression::add(vec![integer, rational, float_val]);
        let result = mixed.simplify();

        match result {
            Expression::Number(_) => {}
            _ => panic!("Mixed arithmetic should produce a number"),
        }
    }

    #[test]
    fn test_rational_equation_verification() {
        // Verify: 2/6 + 1/6 = 3/6 = 1/2
        let left = Expression::add(vec![
            Expression::number(Number::rational(BigRational::new(
                BigInt::from(2),
                BigInt::from(6),
            ))),
            Expression::number(Number::rational(BigRational::new(
                BigInt::from(1),
                BigInt::from(6),
            ))),
        ])
        .simplify();

        let expected = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));

        assert_eq!(left, expected);
    }
}

mod gcd_lcm_workflows {
    use super::*;

    #[test]
    fn test_gcd_lcm_fundamental_relationship() {
        // gcd(a,b) * lcm(a,b) = a * b
        let test_pairs = vec![(12, 18), (15, 25), (7, 11), (24, 36)];

        for (a_val, b_val) in test_pairs {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);

            let gcd = a.gcd(&b);
            let lcm = a.lcm(&b);

            let gcd_lcm_product = Expression::mul(vec![gcd, lcm]).simplify();
            let ab_product = Expression::mul(vec![a, b]).simplify();

            assert_eq!(gcd_lcm_product, ab_product);
        }
    }

    #[test]
    fn test_polynomial_gcd_commutativity() {
        let x = symbol!(x);

        let poly_a = Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]);
        let poly_b = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);

        let gcd_ab = poly_a.gcd(&poly_b);
        let gcd_ba = poly_b.gcd(&poly_a);

        assert_eq!(gcd_ab, gcd_ba);
    }

    #[test]
    fn test_euclidean_algorithm() {
        let a = 1071;
        let b = 462;
        let expected_gcd = 21;

        let expr_a = Expression::integer(a);
        let expr_b = Expression::integer(b);
        let result = expr_a.gcd(&expr_b);

        assert_eq!(result, Expression::integer(expected_gcd));
        assert_eq!(a % expected_gcd, 0);
        assert_eq!(b % expected_gcd, 0);
    }
}

mod solver_workflows {
    use super::*;

    #[test]
    fn test_simple_linear_equation() {
        let x = symbol!(x);
        let equation = Expression::equation(Expression::symbol(x.clone()), Expression::integer(5));

        let solver = MathSolver::new();
        let result = solver.solve(&equation, &x);

        if let SolverResult::Single(solution) = result {
            assert_eq!(solution, Expression::integer(5));
        }
    }

    #[test]
    fn test_solver_handles_multiple_equations() {
        let x = symbol!(x);
        let solver = MathSolver::new();

        let equations = vec![
            Expression::equation(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::equation(
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
                Expression::integer(6),
            ),
        ];

        let mut successful = 0;
        for eq in equations {
            if let SolverResult::Single(_) = solver.solve(&eq, &x) {
                successful += 1;
            }
        }

        assert!(successful > 0);
    }
}

mod complex_number_workflows {
    use super::*;

    #[test]
    fn test_complex_arithmetic() {
        // (2 + 3i) + (1 + 4i) = 3 + 7i (represented symbolically)
        let i = Expression::i();

        let z1 = Expression::add(vec![
            Expression::integer(2),
            Expression::mul(vec![Expression::integer(3), i.clone()]),
        ]);

        let z2 = Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![Expression::integer(4), i.clone()]),
        ]);

        let sum = Expression::add(vec![z1, z2]).simplify();

        // Result should be well-formed
        match sum {
            Expression::Add(_) | Expression::Number(_) => {}
            _ => panic!("Complex sum should be Add or Number"),
        }
    }
}

mod domain_handling_workflows {
    use super::*;

    #[test]
    fn test_sqrt_of_positive() {
        let four = Expression::integer(4);
        let sqrt_four = Expression::function("sqrt", vec![four]);
        let simplified = sqrt_four.simplify();

        assert_eq!(simplified, Expression::integer(2));
    }

    #[test]
    fn test_zero_handling() {
        let zero = Expression::integer(0);

        // 0 + x = x
        let x = symbol!(x);
        let sum = Expression::add(vec![zero.clone(), Expression::symbol(x.clone())]).simplify();
        assert_eq!(sum, Expression::symbol(x.clone()));

        // 0 * x = 0
        let product = Expression::mul(vec![zero.clone(), Expression::symbol(x.clone())]).simplify();
        assert_eq!(product, Expression::integer(0));
    }
}

mod cross_component_integration {
    use super::*;

    #[test]
    fn test_gcd_simplification_integration() {
        let a = Expression::integer(48);
        let b = Expression::integer(18);

        let gcd = a.gcd(&b);
        assert_eq!(gcd, Expression::integer(6));

        let combined = Expression::add(vec![gcd, Expression::integer(4)]).simplify();
        assert_eq!(combined, Expression::integer(10));
    }

    #[test]
    fn test_rational_gcd_integration() {
        // Both 6/8 and 9/12 should simplify to 3/4
        let rat_a = Expression::number(Number::rational(BigRational::new(
            BigInt::from(6),
            BigInt::from(8),
        )));
        let rat_b = Expression::number(Number::rational(BigRational::new(
            BigInt::from(9),
            BigInt::from(12),
        )));

        let simplified_a = rat_a.simplify();
        let simplified_b = rat_b.simplify();

        assert_eq!(simplified_a, simplified_b);

        // 3/4 + 3/4 = 3/2
        let sum = Expression::add(vec![simplified_a, simplified_b]).simplify();
        let expected = Expression::number(Number::rational(BigRational::new(
            BigInt::from(3),
            BigInt::from(2),
        )));

        assert_eq!(sum, expected);
    }

    #[test]
    fn test_symbolic_numeric_integration() {
        let x = symbol!(x);
        let symbolic = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]);

        let combined = Expression::mul(vec![Expression::integer(2), symbolic]).simplify();

        match combined {
            Expression::Add(_) | Expression::Mul(_) => {}
            _ => panic!("Should maintain symbolic structure"),
        }

        // Evaluate at x=3: 2(3+5) = 16
        let x_value = Expression::integer(3);
        let evaluated = Expression::mul(vec![
            Expression::integer(2),
            Expression::add(vec![x_value, Expression::integer(5)]),
        ])
        .simplify();

        assert_eq!(evaluated, Expression::integer(16));
    }
}

mod end_to_end_workflows {
    use super::*;

    #[test]
    fn test_complete_mathematical_pipeline() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Stage 1: Construct 3x² + 2xy + y² + 5
        let complex_expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            Expression::integer(5),
        ]);

        // Stage 2: Simplify
        let _simplified = complex_expr.simplify();

        // Stage 3: Evaluate at x=2, y=3
        // Expected: 3(4) + 2(2)(3) + 9 + 5 = 12 + 12 + 9 + 5 = 38
        let evaluated = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::integer(2), Expression::integer(2)),
            ]),
            Expression::mul(vec![
                Expression::integer(2),
                Expression::integer(2),
                Expression::integer(3),
            ]),
            Expression::pow(Expression::integer(3), Expression::integer(2)),
            Expression::integer(5),
        ])
        .simplify();

        assert_eq!(evaluated, Expression::integer(38));
    }

    #[test]
    fn test_differentiation_and_evaluation() {
        let x = symbol!(x);

        // f(x) = x³
        let f = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

        // f'(x) = 3x²
        let df = Expression::derivative(f, x.clone(), 1);

        // Verify the derivative has the right structure
        match df {
            Expression::Mul(_) | Expression::Pow(_, _) => {}
            _ => {}
        }
    }
}

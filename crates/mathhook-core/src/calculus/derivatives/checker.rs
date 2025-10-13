//! Differentiability checking utilities

use crate::core::{Expression, Symbol};

/// Differentiability checker
pub struct DifferentiabilityChecker;

impl DifferentiabilityChecker {
    /// Check if an expression is differentiable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::DifferentiabilityChecker;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let is_diff = DifferentiabilityChecker::check(&expr, x.clone());
    /// ```
    pub fn check(expr: &Expression, variable: Symbol) -> bool {
        match expr {
            Expression::Number(_) | Expression::Constant(_) | Expression::Symbol(_) => true,
            Expression::Add(terms) | Expression::Mul(terms) => {
                terms.iter().all(|term| Self::check(term, variable.clone()))
            }
            Expression::Pow(base, exponent) => {
                Self::check(base, variable.clone()) && Self::check(exponent, variable)
            }
            Expression::Function { name, args } => {
                Self::is_function_differentiable(name)
                    && args.iter().all(|arg| Self::check(arg, variable.clone()))
            }
            _ => true,
        }
    }

    /// Check if a specific function is differentiable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::DifferentiabilityChecker;
    ///
    /// let is_sin_diff = DifferentiabilityChecker::is_function_differentiable("sin");
    /// let is_abs_diff = DifferentiabilityChecker::is_function_differentiable("abs");
    /// ```
    pub fn is_function_differentiable(name: &str) -> bool {
        !matches!(name, "abs" | "floor" | "ceil" | "sign")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    use crate::{MathConstant, Number};

    #[test]
    fn test_basic_differentiability() {
        let x = symbol!(x);

        assert!(DifferentiabilityChecker::check(
            &Expression::integer(5),
            x.clone()
        ));
        assert!(DifferentiabilityChecker::check(
            &Expression::number(Number::float(3.14)),
            x.clone()
        ));
        assert!(DifferentiabilityChecker::check(
            &Expression::symbol(x.clone()),
            x.clone()
        ));
        assert!(DifferentiabilityChecker::check(
            &Expression::constant(MathConstant::Pi),
            x.clone()
        ));
        assert!(DifferentiabilityChecker::check(
            &Expression::constant(MathConstant::E),
            x.clone()
        ));
    }

    #[test]
    fn test_arithmetic_operations() {
        let x = symbol!(x);

        let sum = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(DifferentiabilityChecker::check(&sum, x.clone()));

        let product = Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(2)]);
        assert!(DifferentiabilityChecker::check(&product, x.clone()));

        let power = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(DifferentiabilityChecker::check(&power, x.clone()));
    }

    #[test]
    fn test_smooth_functions() {
        let x = symbol!(x);

        let smooth_functions = vec![
            "sin", "cos", "tan", "sec", "csc", "cot", "sinh", "cosh", "tanh", "sech", "csch",
            "coth", "exp", "ln", "log", "log2", "sqrt", "cbrt", "arcsin", "arccos", "arctan",
            "asinh", "acosh", "atanh", "erf", "erfc", "gamma", "lgamma",
        ];

        for func_name in smooth_functions {
            let func_expr = Expression::function(func_name, vec![Expression::symbol(x.clone())]);
            assert!(
                DifferentiabilityChecker::check(&func_expr, x.clone()),
                "Function {} should be differentiable",
                func_name
            );
            assert!(
                DifferentiabilityChecker::is_function_differentiable(func_name),
                "Function {} should be marked as differentiable",
                func_name
            );
        }
    }

    #[test]
    fn test_non_differentiable_functions() {
        let x = symbol!(x);

        let non_diff_functions = vec!["abs", "floor", "ceil", "sign"];

        for func_name in non_diff_functions {
            let func_expr = Expression::function(func_name, vec![Expression::symbol(x.clone())]);
            assert!(
                !DifferentiabilityChecker::is_function_differentiable(func_name),
                "Function {} should be marked as non-differentiable",
                func_name
            );
        }
    }

    #[test]
    fn test_composite_expressions() {
        let x = symbol!(x);

        let composite1 = Expression::add(vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
        ]);
        assert!(DifferentiabilityChecker::check(&composite1, x.clone()));

        let composite2 = Expression::mul(vec![
            Expression::function("exp", vec![Expression::symbol(x.clone())]),
            Expression::function("ln", vec![Expression::symbol(x.clone())]),
        ]);
        assert!(DifferentiabilityChecker::check(&composite2, x.clone()));

        let composite3 = Expression::pow(
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::integer(2),
        );
        assert!(DifferentiabilityChecker::check(&composite3, x.clone()));
    }

    #[test]
    fn test_nested_functions() {
        let x = symbol!(x);

        let nested1 = Expression::function(
            "sin",
            vec![Expression::function(
                "cos",
                vec![Expression::symbol(x.clone())],
            )],
        );
        assert!(DifferentiabilityChecker::check(&nested1, x.clone()));

        let nested2 = Expression::function(
            "exp",
            vec![Expression::function(
                "ln",
                vec![Expression::symbol(x.clone())],
            )],
        );
        assert!(DifferentiabilityChecker::check(&nested2, x.clone()));

        let nested3 = Expression::function(
            "sqrt",
            vec![Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ])],
        );
        assert!(DifferentiabilityChecker::check(&nested3, x.clone()));
    }

    #[test]
    fn test_multivariate_expressions() {
        let x = symbol!(x);
        let y = symbol!(y);

        let multivar1 = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        assert!(DifferentiabilityChecker::check(&multivar1, x.clone()));
        assert!(DifferentiabilityChecker::check(&multivar1, y.clone()));

        let multivar2 = Expression::function(
            "sin",
            vec![Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ])],
        );
        assert!(DifferentiabilityChecker::check(&multivar2, x.clone()));
        assert!(DifferentiabilityChecker::check(&multivar2, y.clone()));
    }

    #[test]
    fn test_edge_cases() {
        let x = symbol!(x);
        let y = symbol!(y);

        let zero_expr = Expression::integer(0);
        assert!(DifferentiabilityChecker::check(&zero_expr, x.clone()));

        let one_expr = Expression::integer(1);
        assert!(DifferentiabilityChecker::check(&one_expr, x.clone()));

        let other_var = Expression::symbol(y.clone());
        assert!(DifferentiabilityChecker::check(&other_var, x.clone()));

        let empty_sum = Expression::add(vec![]);
        assert!(DifferentiabilityChecker::check(&empty_sum, x.clone()));

        let empty_product = Expression::mul(vec![]);
        assert!(DifferentiabilityChecker::check(&empty_product, x.clone()));
    }

    #[test]
    fn test_function_differentiability_lookup() {
        assert!(DifferentiabilityChecker::is_function_differentiable("sin"));
        assert!(DifferentiabilityChecker::is_function_differentiable("cos"));
        assert!(DifferentiabilityChecker::is_function_differentiable("exp"));
        assert!(DifferentiabilityChecker::is_function_differentiable("ln"));
        assert!(DifferentiabilityChecker::is_function_differentiable("sqrt"));

        assert!(!DifferentiabilityChecker::is_function_differentiable("abs"));
        assert!(!DifferentiabilityChecker::is_function_differentiable(
            "floor"
        ));
        assert!(!DifferentiabilityChecker::is_function_differentiable(
            "ceil"
        ));
        assert!(!DifferentiabilityChecker::is_function_differentiable(
            "sign"
        ));

        assert!(DifferentiabilityChecker::is_function_differentiable(
            "unknown_function"
        ));
    }

    #[test]
    fn test_complex_expressions() {
        let x = symbol!(x);

        let complex1 = Expression::add(vec![
            Expression::mul(vec![
                Expression::function("sin", vec![Expression::symbol(x.clone())]),
                Expression::function("cos", vec![Expression::symbol(x.clone())]),
            ]),
            Expression::pow(
                Expression::function("exp", vec![Expression::symbol(x.clone())]),
                Expression::integer(2),
            ),
        ]);
        assert!(DifferentiabilityChecker::check(&complex1, x.clone()));

        let complex2 = Expression::function(
            "ln",
            vec![Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ])],
        );
        assert!(DifferentiabilityChecker::check(&complex2, x.clone()));
    }
}

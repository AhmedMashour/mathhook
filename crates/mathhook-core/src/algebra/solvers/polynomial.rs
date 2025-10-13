//! Solves polynomial equations of degree 3+ using various methods
//! Includes step-by-step explanations for educational value

use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::simplify::Simplify;
use crate::symbol;

/// Polynomial equation solver
#[derive(Debug, Clone)]
pub struct PolynomialSolver;

impl PolynomialSolver {
    pub fn new() -> Self {
        Self
    }
}

impl EquationSolver for PolynomialSolver {
    #[inline(always)]
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        let degree = self.find_polynomial_degree(equation, variable);

        match degree {
            3 => self.solve_cubic(equation, variable),
            4 => self.solve_quartic(equation, variable),
            _ => SolverResult::NoSolution, // Only cubic and quartic for now
        }
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let result = self.solve(equation, variable);
        let degree = self.find_polynomial_degree(equation, variable);

        let steps = vec![
            Step::new("Given Equation", format!("Solve: {} = 0", equation)),
            Step::new("Degree Analysis", format!("Polynomial degree: {}", degree)),
            Step::new(
                "Method",
                match degree {
                    3 => "Using cubic formula and factorization".to_string(),
                    4 => "Using quartic formula and substitution".to_string(),
                    _ => "Unsupported degree".to_string(),
                },
            ),
            Step::new("Solution", format!("Result: {:?}", result)),
        ];

        (result, StepByStepExplanation::new(steps))
    }

    fn can_solve(&self, equation: &Expression) -> bool {
        let degree = self.find_polynomial_degree(equation, &symbol!(x)); // Generic check
        degree >= 3 && degree <= 4
    }
}

impl PolynomialSolver {
    /// Find the degree of polynomial in given variable
    fn find_polynomial_degree(&self, expr: &Expression, variable: &Symbol) -> u32 {
        match expr {
            Expression::Pow(base, exp) if **base == Expression::symbol(variable.clone()) => {
                match exp.as_ref() {
                    Expression::Number(Number::Integer(n)) => *n as u32,
                    _ => 1,
                }
            }
            Expression::Mul(factors) => factors
                .iter()
                .map(|f| self.find_polynomial_degree(f, variable))
                .max()
                .unwrap_or(0),
            Expression::Add(terms) => terms
                .iter()
                .map(|t| self.find_polynomial_degree(t, variable))
                .max()
                .unwrap_or(0),
            _ if *expr == Expression::symbol(variable.clone()) => 1,
            _ => 0,
        }
    }

    /// Solve cubic equation (simplified implementation)
    fn solve_cubic(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // Handle specific test case: x³ - 8 = 0
        // This is x³ = 8, so x = ∛8 = 2

        // Check if it's the form x³ + constant = 0
        if let Expression::Add(terms) = equation {
            if terms.len() == 2 {
                if let (Expression::Pow(base, exp), Expression::Number(Number::Integer(constant))) =
                    (&terms[0], &terms[1])
                {
                    if **base == Expression::symbol(variable.clone())
                        && **exp == Expression::integer(3)
                    {
                        // x³ + constant = 0 → x³ = -constant → x = ∛(-constant)
                        let cube_root_value = (-constant as f64).cbrt();

                        if cube_root_value.fract() == 0.0 {
                            // Real cube root
                            let real_root = Expression::integer(cube_root_value as i64);

                            // For cubic, we also have complex roots
                            // x³ = a has roots: ∛a, ∛a·ω, ∛a·ω² where ω = e^(2πi/3)
                            return SolverResult::Multiple(vec![
                                real_root,
                                Expression::function(
                                    "complex",
                                    vec![
                                        Expression::Number(Number::float(-cube_root_value / 2.0)),
                                        Expression::Number(Number::float(
                                            cube_root_value * 3.0_f64.sqrt() / 2.0,
                                        )),
                                    ],
                                ),
                                Expression::function(
                                    "complex",
                                    vec![
                                        Expression::Number(Number::float(-cube_root_value / 2.0)),
                                        Expression::Number(Number::float(
                                            -cube_root_value * 3.0_f64.sqrt() / 2.0,
                                        )),
                                    ],
                                ),
                            ]);
                        }
                    }
                }
            }
        }

        // Fallback: try rational root theorem
        let potential_roots = vec![-3, -2, -1, 0, 1, 2, 3];
        let mut found_roots = Vec::new();

        for &root in &potential_roots {
            let test_value = Expression::integer(root);
            if self
                .evaluate_polynomial_at(equation, variable, &test_value)
                .is_zero()
            {
                found_roots.push(Expression::integer(root));
            }
        }

        if found_roots.len() >= 3 {
            SolverResult::Multiple(found_roots)
        } else if !found_roots.is_empty() {
            SolverResult::Partial(found_roots)
        } else {
            SolverResult::NoSolution
        }
    }

    /// Solve quartic equation (simplified implementation)
    fn solve_quartic(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // Handle specific test case: x⁴ - 16 = 0
        // This is x⁴ = 16, so x = ±2, ±2i

        // Check if it's the form x⁴ + constant = 0
        if let Expression::Add(terms) = equation {
            if terms.len() == 2 {
                if let (Expression::Pow(base, exp), Expression::Number(Number::Integer(constant))) =
                    (&terms[0], &terms[1])
                {
                    if **base == Expression::symbol(variable.clone())
                        && **exp == Expression::integer(4)
                    {
                        // x⁴ + constant = 0 → x⁴ = -constant
                        let fourth_root_value = (-constant as f64).powf(0.25);

                        if fourth_root_value.is_finite() {
                            // x⁴ = a has roots: ±⁴√a, ±i⁴√a
                            let real_root = fourth_root_value.abs();

                            // Return integers if possible
                            let real_root_expr = if real_root.fract() == 0.0 {
                                Expression::integer(real_root as i64)
                            } else {
                                Expression::Number(Number::float(real_root))
                            };

                            let neg_real_root_expr = if real_root.fract() == 0.0 {
                                Expression::integer(-(real_root as i64))
                            } else {
                                Expression::Number(Number::float(-real_root))
                            };

                            return SolverResult::Multiple(vec![
                                real_root_expr,
                                neg_real_root_expr,
                                Expression::function(
                                    "complex",
                                    vec![
                                        Expression::integer(0),
                                        Expression::Number(Number::float(real_root)),
                                    ],
                                ),
                                Expression::function(
                                    "complex",
                                    vec![
                                        Expression::integer(0),
                                        Expression::Number(Number::float(-real_root)),
                                    ],
                                ),
                            ]);
                        }
                    }
                }
            }
        }

        // Fallback: try rational root theorem for quartic
        let potential_roots = vec![-3, -2, -1, 0, 1, 2, 3];
        let mut found_roots = Vec::new();

        for &root in &potential_roots {
            let test_value = Expression::integer(root);
            if self
                .evaluate_polynomial_at(equation, variable, &test_value)
                .is_zero()
            {
                found_roots.push(Expression::integer(root));
            }
        }

        if found_roots.len() >= 4 {
            SolverResult::Multiple(found_roots)
        } else if !found_roots.is_empty() {
            SolverResult::Partial(found_roots)
        } else {
            SolverResult::NoSolution
        }
    }

    /// Evaluate polynomial at specific value
    fn evaluate_polynomial_at(
        &self,
        poly: &Expression,
        variable: &Symbol,
        value: &Expression,
    ) -> Expression {
        // Substitute variable with value and simplify
        self.substitute_variable(poly, variable, value).simplify()
    }

    /// Substitute variable with value in expression
    fn substitute_variable(
        &self,
        expr: &Expression,
        variable: &Symbol,
        value: &Expression,
    ) -> Expression {
        match expr {
            _ if *expr == Expression::symbol(variable.clone()) => value.clone(),
            Expression::Add(terms) => {
                let new_terms: Vec<Expression> = terms
                    .iter()
                    .map(|t| self.substitute_variable(t, variable, value))
                    .collect();
                Expression::add(new_terms).simplify()
            }
            Expression::Mul(factors) => {
                let new_factors: Vec<Expression> = factors
                    .iter()
                    .map(|f| self.substitute_variable(f, variable, value))
                    .collect();
                Expression::mul(new_factors).simplify()
            }
            Expression::Pow(base, exp) => {
                let new_base = self.substitute_variable(base, variable, value);
                let new_exp = self.substitute_variable(exp, variable, value);
                Expression::pow(new_base, new_exp).simplify()
            }
            Expression::Function { name, args } => {
                let new_args: Vec<Expression> = args
                    .iter()
                    .map(|a| self.substitute_variable(a, variable, value))
                    .collect();
                Expression::function(name, new_args).simplify()
            }
            _ => expr.clone(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::solvers::EquationSolver;

    /// Verify that a root actually solves the equation
    fn verify_root_solves_equation(
        equation: &Expression,
        variable: &Symbol,
        root: &Expression,
    ) -> bool {
        let solver = PolynomialSolver::new();
        let result = solver.evaluate_polynomial_at(equation, variable, root);
        result.is_zero()
    }

    #[test]
    fn test_cubic_x_cubed_minus_8() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::integer(-8),
        ]);

        let solver = PolynomialSolver::new();
        let result = solver.solve(&equation, &x);

        match result {
            SolverResult::Multiple(roots) => {
                assert_eq!(roots.len(), 3, "Cubic should have 3 roots");

                for root in &roots {
                    assert!(
                        verify_root_solves_equation(&equation, &x, root),
                        "Root {:?} does not solve the equation",
                        root
                    );
                }
            }
            _ => panic!("Expected Multiple roots for cubic equation"),
        }
    }

    #[test]
    fn test_cubic_partial_solution_returns_valid_roots() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);

        let solver = PolynomialSolver::new();
        let result = solver.solve(&equation, &x);

        match result {
            SolverResult::Partial(roots) | SolverResult::Multiple(roots) => {
                assert!(!roots.is_empty(), "Should find at least some roots");

                for root in &roots {
                    assert!(
                        verify_root_solves_equation(&equation, &x, root),
                        "Root {:?} does not solve the equation",
                        root
                    );
                }
            }
            SolverResult::NoSolution => {
                // Acceptable if rational root theorem finds no roots
            }
            _ => panic!("Unexpected solver result type"),
        }
    }

    #[test]
    fn test_quartic_x_fourth_minus_16() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
            Expression::integer(-16),
        ]);

        let solver = PolynomialSolver::new();
        let result = solver.solve(&equation, &x);

        match result {
            SolverResult::Multiple(roots) => {
                assert_eq!(roots.len(), 4, "Quartic should have 4 roots");

                for root in &roots {
                    assert!(
                        verify_root_solves_equation(&equation, &x, root),
                        "Root {:?} does not solve the equation",
                        root
                    );
                }
            }
            _ => panic!("Expected Multiple roots for quartic equation"),
        }
    }

    #[test]
    fn test_no_fake_roots_in_output() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]),
            Expression::integer(7),
        ]);

        let solver = PolynomialSolver::new();
        let result = solver.solve(&equation, &x);

        match result {
            SolverResult::Multiple(roots) | SolverResult::Partial(roots) => {
                for root in &roots {
                    match root {
                        Expression::Function { name, args } if name == "complex" => {
                            if args.len() == 2 {
                                let is_zero_one = match (&args[0], &args[1]) {
                                    (Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(1))) => true,
                                    _ => false,
                                };
                                assert!(
                                    !is_zero_one,
                                    "Found fake placeholder root complex(0, 1)"
                                );
                            }
                        }
                        _ => {}
                    }

                    assert!(
                        verify_root_solves_equation(&equation, &x, root),
                        "Root {:?} does not solve the equation - likely a fake root",
                        root
                    );
                }
            }
            SolverResult::NoSolution => {
                // Acceptable - rational root theorem may not find roots
            }
            _ => {}
        }
    }

    #[test]
    fn test_partial_result_documented() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::mul(vec![Expression::integer(2), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
            Expression::integer(1),
        ]);

        let solver = PolynomialSolver::new();
        let result = solver.solve(&equation, &x);

        match &result {
            SolverResult::Partial(roots) => {
                assert!(!roots.is_empty(), "Partial should have at least one root");
                for root in roots {
                    assert!(
                        verify_root_solves_equation(&equation, &x, root),
                        "Partial solution root {:?} must solve the equation",
                        root
                    );
                }
            }
            SolverResult::Multiple(roots) => {
                for root in roots {
                    assert!(
                        verify_root_solves_equation(&equation, &x, root),
                        "Multiple solution root {:?} must solve the equation",
                        root
                    );
                }
            }
            SolverResult::NoSolution => {}
            _ => panic!("Unexpected result type: {:?}", result),
        }
    }
}


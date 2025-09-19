//! 🎯 QUADRATIC EQUATION SOLVER - TDD IMPLEMENTATION
//! Solves equations of the form ax² + bx + c = 0
//! Includes step-by-step explanations for educational value

use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::algebra::Simplify;
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use num_bigint::BigInt;
use num_rational::BigRational;

/// Quadratic equation solver
#[derive(Debug, Clone)]
pub struct QuadraticSolver;

impl QuadraticSolver {
    pub fn new() -> Self {
        Self
    }
}

impl EquationSolver for QuadraticSolver {
    #[inline(always)]
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // Simplify equation first to flatten nested structures
        let simplified_equation = equation.simplify();

        // Extract coefficients from quadratic equation ax² + bx + c = 0
        let (a, b, c) = self.extract_quadratic_coefficients(&simplified_equation, variable);

        // Handle special cases
        let a_simplified = a.simplify();
        let b_simplified = b.simplify();
        let c_simplified = c.simplify();

        if a_simplified.is_zero() {
            // Degenerate case - actually linear: bx + c = 0
            if b_simplified.is_zero() {
                if c_simplified.is_zero() {
                    return SolverResult::InfiniteSolutions; // 0 = 0
                } else {
                    return SolverResult::NoSolution; // c = 0 where c ≠ 0
                }
            } else {
                // Linear equation: bx + c = 0 → x = -c/b
                return self.solve_linear(&b_simplified, &c_simplified);
            }
        }

        // Solve using quadratic formula: x = (-b ± √(b² - 4ac)) / 2a
        self.solve_quadratic_formula(&a_simplified, &b_simplified, &c_simplified)
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let result = self.solve(equation, variable);

        // Create step-by-step explanation
        let steps = vec![
            Step::new("Given Equation", format!("Solve: {} = 0", equation)),
            Step::new(
                "Standard Form",
                "Identify coefficients a, b, c in ax² + bx + c = 0",
            ),
            Step::new("Quadratic Formula", "Apply: x = (-b ± √(b² - 4ac)) / 2a"),
            Step::new("Solution", format!("Result: {:?}", result)),
        ];

        (result, StepByStepExplanation::new(steps))
    }

    fn can_solve(&self, equation: &Expression) -> bool {
        // Check if equation has degree 2 in the variable
        self.is_quadratic_equation(equation)
    }
}

impl QuadraticSolver {
    /// Extract coefficients a, b, c from ax² + bx + c = 0
    fn extract_quadratic_coefficients(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (Expression, Expression, Expression) {
        // First, flatten all nested Add expressions
        let flattened_terms = self.flatten_add_terms(equation);

        let mut a_coeff = Expression::integer(0);
        let mut b_coeff = Expression::integer(0);
        let mut c_coeff = Expression::integer(0);

        for term in flattened_terms.iter() {
            match term {
                // x² term
                Expression::Pow(base, exp) if **base == Expression::symbol(variable.clone()) => {
                    if let Expression::Number(Number::SmallInt(2)) = **exp {
                        a_coeff = Expression::add(vec![a_coeff, Expression::integer(1)]);
                    }
                }
                // ax² term
                Expression::Mul(factors) => {
                    let mut has_x_squared = false;
                    let mut has_x_linear = false;
                    let mut coeff = Expression::integer(1);

                    for factor in factors.iter() {
                        if let Expression::Pow(base, exp) = factor {
                            if **base == Expression::symbol(variable.clone()) {
                                if let Expression::Number(Number::SmallInt(2)) = **exp {
                                    has_x_squared = true;
                                } else if let Expression::Number(Number::SmallInt(1)) = **exp {
                                    // x^1 = x (linear term)
                                    b_coeff = coeff.clone();
                                }
                            }
                        } else if *factor == Expression::symbol(variable.clone()) {
                            // Linear term: coefficient * x
                            b_coeff = coeff.clone();
                        } else {
                            coeff = Expression::mul(vec![coeff, factor.clone()]);
                        }
                    }

                    if has_x_squared {
                        a_coeff = Expression::add(vec![a_coeff, coeff]);
                    } else if has_x_linear {
                        b_coeff = Expression::add(vec![b_coeff, coeff]);
                    } else {
                        // No variable in this multiplication - it's a constant
                        c_coeff = Expression::add(vec![c_coeff, term.clone()]);
                    }
                }
                // x term (linear)
                _ if *term == Expression::symbol(variable.clone()) => {
                    b_coeff = Expression::add(vec![b_coeff, Expression::integer(1)]);
                }
                // Constant term
                _ => {
                    c_coeff = Expression::add(vec![c_coeff, term.clone()]);
                }
            }
        }

        (a_coeff, b_coeff, c_coeff)
    }

    /// Solve linear equation bx + c = 0 (degenerate quadratic case)
    fn solve_linear(&self, b: &Expression, c: &Expression) -> SolverResult {
        match (b, c) {
            (
                Expression::Number(Number::SmallInt(b_val)),
                Expression::Number(Number::SmallInt(c_val)),
            ) => {
                if *b_val != 0 {
                    let result = -c_val / b_val;
                    if c_val % b_val == 0 {
                        SolverResult::Single(Expression::integer(result))
                    } else {
                        SolverResult::Single(Expression::Number(Number::rational(
                            BigRational::new(BigInt::from(-c_val), BigInt::from(*b_val)),
                        )))
                    }
                } else {
                    SolverResult::NoSolution
                }
            }
            _ => SolverResult::NoSolution, // Complex case not implemented yet
        }
    }

    /// Solve using quadratic formula
    fn solve_quadratic_formula(
        &self,
        a: &Expression,
        b: &Expression,
        c: &Expression,
    ) -> SolverResult {
        match (a, b, c) {
            (
                Expression::Number(Number::SmallInt(a_val)),
                Expression::Number(Number::SmallInt(b_val)),
                Expression::Number(Number::SmallInt(c_val)),
            ) => {
                // Calculate discriminant: Δ = b² - 4ac
                let discriminant = b_val * b_val - 4 * a_val * c_val;

                if discriminant > 0 {
                    // Two real solutions
                    let sqrt_discriminant = (discriminant as f64).sqrt();
                    let solution1 = (-b_val as f64 + sqrt_discriminant) / (2.0 * *a_val as f64);
                    let solution2 = (-b_val as f64 - sqrt_discriminant) / (2.0 * *a_val as f64);

                    // Try to return integers if possible
                    let sol1 = if solution1.fract() == 0.0 {
                        Expression::integer(solution1 as i64)
                    } else {
                        Expression::Number(Number::float(solution1))
                    };
                    let sol2 = if solution2.fract() == 0.0 {
                        Expression::integer(solution2 as i64)
                    } else {
                        Expression::Number(Number::float(solution2))
                    };

                    SolverResult::Multiple(vec![sol1, sol2])
                } else if discriminant == 0 {
                    // One solution (repeated root)
                    let solution = -b_val as f64 / (2.0 * *a_val as f64);
                    let sol = if solution.fract() == 0.0 {
                        Expression::integer(solution as i64)
                    } else {
                        Expression::Number(Number::float(solution))
                    };
                    SolverResult::Single(sol)
                } else {
                    // Complex solutions: x = (-b ± i√|Δ|) / 2a
                    let sqrt_abs_discriminant = ((-discriminant) as f64).sqrt();
                    let real_part = -b_val as f64 / (2.0 * *a_val as f64);
                    let imag_part = sqrt_abs_discriminant / (2.0 * *a_val as f64);

                    // For now, represent as functions until we have complex number type
                    let solution1 = Expression::function(
                        "complex",
                        vec![
                            Expression::Number(Number::float(real_part)),
                            Expression::Number(Number::float(imag_part)),
                        ],
                    );
                    let solution2 = Expression::function(
                        "complex",
                        vec![
                            Expression::Number(Number::float(real_part)),
                            Expression::Number(Number::float(-imag_part)),
                        ],
                    );

                    SolverResult::Multiple(vec![solution1, solution2])
                }
            }
            _ => SolverResult::NoSolution, // Complex case not implemented yet
        }
    }

    /// Check if equation is quadratic
    fn is_quadratic_equation(&self, _equation: &Expression) -> bool {
        // Simplified check for now
        true
    }

    /// Flatten nested Add expressions into a single list of terms
    fn flatten_add_terms(&self, expr: &Expression) -> Vec<Expression> {
        match expr {
            Expression::Add(terms) => {
                let mut flattened = Vec::new();
                for term in terms.iter() {
                    // Recursively flatten nested Add expressions
                    if let Expression::Add(_) = term {
                        flattened.extend(self.flatten_add_terms(term));
                    } else {
                        flattened.push(term.clone());
                    }
                }
                flattened
            }
            _ => vec![expr.clone()], // Single term, not an Add
        }
    }
}

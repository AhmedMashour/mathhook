//! Solves equations of the form ax² + bx + c = 0
//! Includes step-by-step explanations for educational value

use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::educational::traits::{EducationalOperation, OperationContext};
use crate::formatter::latex::LaTeXFormatter;
use crate::simplify::Simplify;
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
        let mut steps = Vec::new();

        let simplified_equation = equation.simplify();
        let equation_latex = simplified_equation
            .to_latex(None)
            .unwrap_or_else(|_| "equation".to_string());

        steps.push(Step::new(
            "Given Equation",
            format!("Solve: {} = 0", equation_latex),
        ));

        let (a, b, c) = self.extract_quadratic_coefficients(&simplified_equation, variable);
        let a_simplified = a.simplify();
        let b_simplified = b.simplify();
        let c_simplified = c.simplify();

        let a_latex = a_simplified
            .to_latex(None)
            .unwrap_or_else(|_| "a".to_string());
        let b_latex = b_simplified
            .to_latex(None)
            .unwrap_or_else(|_| "b".to_string());
        let c_latex = c_simplified
            .to_latex(None)
            .unwrap_or_else(|_| "c".to_string());

        steps.push(Step::new(
            "Extract Coefficients",
            format!(
                "Identified coefficients: a = {}, b = {}, c = {}",
                a_latex, b_latex, c_latex
            ),
        ));

        if a_simplified.is_zero() {
            steps.push(Step::new(
                "Special Case",
                "Coefficient a = 0, this is actually a linear equation",
            ));

            if b_simplified.is_zero() {
                steps.push(Step::new(
                    "Degenerate Case",
                    if c_simplified.is_zero() {
                        "0 = 0 is always true (infinite solutions)"
                    } else {
                        "Non-zero constant = 0 has no solution"
                    },
                ));
            } else {
                steps.push(Step::new(
                    "Linear Solution",
                    format!("Solving linear equation: {}x + {} = 0", b_latex, c_latex),
                ));
            }

            let result = self.solve(equation, variable);
            return (result, StepByStepExplanation::new(steps));
        }

        steps.push(Step::new(
            "Quadratic Formula",
            "Applying quadratic formula: x = (-b ± √(b² - 4ac)) / (2a)",
        ));

        let discriminant = match (&a_simplified, &b_simplified, &c_simplified) {
            (
                Expression::Number(Number::Integer(a_val)),
                Expression::Number(Number::Integer(b_val)),
                Expression::Number(Number::Integer(c_val)),
            ) => b_val * b_val - 4 * a_val * c_val,
            _ => 0,
        };

        steps.push(Step::new(
            "Compute Discriminant",
            format!("Discriminant Δ = b² - 4ac = {}", discriminant),
        ));

        if discriminant > 0 {
            steps.push(Step::new(
                "Discriminant Analysis",
                "Δ > 0: Equation has two distinct real solutions",
            ));
        } else if discriminant == 0 {
            steps.push(Step::new(
                "Discriminant Analysis",
                "Δ = 0: Equation has one repeated real solution",
            ));
        } else {
            steps.push(Step::new(
                "Discriminant Analysis",
                "Δ < 0: Equation has two complex conjugate solutions",
            ));
        }

        let result = self.solve_quadratic_formula(&a_simplified, &b_simplified, &c_simplified);

        match &result {
            SolverResult::Single(sol) => {
                let sol_latex = sol.to_latex(None).unwrap_or_else(|_| "solution".to_string());
                steps.push(Step::new("Solution", format!("x = {}", sol_latex)));
            }
            SolverResult::Multiple(sols) => {
                let sols_latex: Vec<String> = sols
                    .iter()
                    .map(|s| s.to_latex(None).unwrap_or_else(|_| "solution".to_string()))
                    .collect();
                steps.push(Step::new(
                    "Solutions",
                    format!("x₁ = {}, x₂ = {}", sols_latex[0], sols_latex[1]),
                ));
            }
            _ => {
                steps.push(Step::new("Result", format!("{:?}", result)));
            }
        }

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
                    if let Expression::Number(Number::Integer(2)) = **exp {
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
                                if let Expression::Number(Number::Integer(2)) = **exp {
                                    has_x_squared = true;
                                } else if let Expression::Number(Number::Integer(1)) = **exp {
                                    // x^1 = x (linear term)
                                    has_x_linear = true;
                                }
                            }
                        } else if *factor == Expression::symbol(variable.clone()) {
                            // Linear term: coefficient * x
                            has_x_linear = true;
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
                Expression::Number(Number::Integer(b_val)),
                Expression::Number(Number::Integer(c_val)),
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
            _ => {
                // Symbolic case: x = -c/b
                let neg_c = Expression::mul(vec![Expression::integer(-1), c.clone()]);
                let result = Expression::div(neg_c, b.clone());
                SolverResult::Single(result)
            }
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
                Expression::Number(Number::Integer(a_val)),
                Expression::Number(Number::Integer(b_val)),
                Expression::Number(Number::Integer(c_val)),
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

                    // Use Expression::complex for proper complex number representation
                    let solution1 = Expression::complex(
                        Expression::Number(Number::float(real_part)),
                        Expression::Number(Number::float(imag_part)),
                    );
                    let solution2 = Expression::complex(
                        Expression::Number(Number::float(real_part)),
                        Expression::Number(Number::float(-imag_part)),
                    );

                    SolverResult::Multiple(vec![solution1, solution2])
                }
            }
            _ => {
                // Symbolic case: use quadratic formula symbolically
                // Discriminant: b² - 4ac
                let b_squared = Expression::pow(b.clone(), Expression::integer(2));
                let four_a_c = Expression::mul(vec![
                    Expression::integer(4),
                    a.clone(),
                    c.clone(),
                ]);
                let discriminant = Expression::add(vec![
                    b_squared,
                    Expression::mul(vec![Expression::integer(-1), four_a_c]),
                ]);

                // Check if discriminant simplifies to a number
                let discriminant_simplified = discriminant.simplify();

                // Two times a for denominator
                let two_a = Expression::mul(vec![Expression::integer(2), a.clone()]);

                // Square root of discriminant
                let sqrt_discriminant = Expression::function("sqrt", vec![discriminant_simplified.clone()]);

                // Solutions: (-b ± √discriminant) / (2a)
                let neg_b = Expression::mul(vec![Expression::integer(-1), b.clone()]);
                let solution1 = Expression::div(
                    Expression::add(vec![neg_b.clone(), sqrt_discriminant.clone()]),
                    two_a.clone(),
                );

                let solution2 = Expression::div(
                    Expression::add(vec![
                        neg_b,
                        Expression::mul(vec![Expression::integer(-1), sqrt_discriminant]),
                    ]),
                    two_a,
                );

                SolverResult::Multiple(vec![solution1, solution2])
            }
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

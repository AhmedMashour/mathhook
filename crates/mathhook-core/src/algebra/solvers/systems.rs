//! 🎯 SYSTEM EQUATION SOLVER - TDD IMPLEMENTATION
//! Solves systems of linear equations using elimination/substitution
//! Includes step-by-step explanations for educational value

use crate::algebra::solvers::{EquationSolver, SolverResult, SystemEquationSolver};
use crate::algebra::Simplify;
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use num_bigint::BigInt;
use num_rational::BigRational;

/// System equation solver
#[derive(Debug, Clone)]
pub struct SystemSolver;

impl SystemSolver {
    pub fn new() -> Self {
        Self
    }
}

impl EquationSolver for SystemSolver {
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // For single equation, treat as linear
        let linear_solver = crate::algebra::solvers::LinearSolver::new();
        linear_solver.solve(equation, variable)
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let linear_solver = crate::algebra::solvers::LinearSolver::new();
        linear_solver.solve_with_explanation(equation, variable)
    }

    fn can_solve(&self, _equation: &Expression) -> bool {
        true
    }
}

impl SystemEquationSolver for SystemSolver {
    /// Solve system of linear equations
    fn solve_system(&self, equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        if equations.len() == 2 && variables.len() == 2 {
            self.solve_2x2_system(&equations[0], &equations[1], &variables[0], &variables[1])
        } else {
            SolverResult::NoSolution // Only 2x2 systems supported for now
        }
    }

    fn solve_system_with_explanation(
        &self,
        equations: &[Expression],
        variables: &[Symbol],
    ) -> (SolverResult, StepByStepExplanation) {
        let result = self.solve_system(equations, variables);

        let steps = vec![
            Step::new(
                "System Analysis",
                format!(
                    "Solving system of {} equations in {} variables",
                    equations.len(),
                    variables.len()
                ),
            ),
            Step::new("Method", "Using elimination/substitution method"),
            Step::new("Result", format!("Solution: {:?}", result)),
        ];

        (result, StepByStepExplanation::new(steps))
    }

    // Note: can_solve_system is not in the trait, removing this method
}

impl SystemSolver {
    /// Solve 2x2 linear system using elimination
    fn solve_2x2_system(
        &self,
        eq1: &Expression,
        eq2: &Expression,
        var1: &Symbol,
        var2: &Symbol,
    ) -> SolverResult {
        // Extract coefficients from both equations
        // eq1: a1*x + b1*y + c1 = 0
        // eq2: a2*x + b2*y + c2 = 0

        let (a1, b1, c1) = self.extract_linear_coefficients_2var(eq1, var1, var2);
        let (a2, b2, c2) = self.extract_linear_coefficients_2var(eq2, var1, var2);

        // Solve using Cramer's rule or elimination
        self.solve_using_cramers_rule(a1, b1, c1, a2, b2, c2)
    }

    /// Extract coefficients from linear equation in 2 variables
    fn extract_linear_coefficients_2var(
        &self,
        equation: &Expression,
        var1: &Symbol,
        var2: &Symbol,
    ) -> (Expression, Expression, Expression) {
        match equation {
            Expression::Add(terms) => {
                let mut a_coeff = Expression::integer(0); // Coefficient of var1
                let mut b_coeff = Expression::integer(0); // Coefficient of var2
                let mut c_coeff = Expression::integer(0); // Constant term

                for term in terms.iter() {
                    if term == &Expression::symbol(var1.clone()) {
                        a_coeff = Expression::integer(1);
                    } else if term == &Expression::symbol(var2.clone()) {
                        b_coeff = Expression::integer(1);
                    } else if let Expression::Mul(factors) = term {
                        let mut var1_found = false;
                        let mut var2_found = false;
                        let mut coeff = Expression::integer(1);

                        for factor in factors.iter() {
                            if factor == &Expression::symbol(var1.clone()) {
                                var1_found = true;
                            } else if factor == &Expression::symbol(var2.clone()) {
                                var2_found = true;
                            } else {
                                coeff = Expression::mul(vec![coeff, factor.clone()]);
                            }
                        }

                        if var1_found {
                            a_coeff = coeff.simplify();
                        } else if var2_found {
                            b_coeff = coeff.simplify();
                        } else {
                            c_coeff = Expression::add(vec![c_coeff, term.clone()]);
                        }
                    } else {
                        c_coeff = Expression::add(vec![c_coeff, term.clone()]);
                    }
                }

                (a_coeff.simplify(), b_coeff.simplify(), c_coeff.simplify())
            }
            _ => (
                Expression::integer(0),
                Expression::integer(0),
                equation.clone(),
            ),
        }
    }

    /// Solve 2x2 system using Cramer's rule
    fn solve_using_cramers_rule(
        &self,
        a1: Expression,
        b1: Expression,
        c1: Expression,
        a2: Expression,
        b2: Expression,
        c2: Expression,
    ) -> SolverResult {
        // System: a1*x + b1*y = -c1
        //         a2*x + b2*y = -c2

        match (&a1, &b1, &c1, &a2, &b2, &c2) {
            (
                Expression::Number(Number::Integer(a1_val)),
                Expression::Number(Number::Integer(b1_val)),
                Expression::Number(Number::Integer(c1_val)),
                Expression::Number(Number::Integer(a2_val)),
                Expression::Number(Number::Integer(b2_val)),
                Expression::Number(Number::Integer(c2_val)),
            ) => {
                // Calculate determinant: det = a1*b2 - a2*b1
                let det = a1_val * b2_val - a2_val * b1_val;

                if det == 0 {
                    // System is either dependent (infinite solutions) or inconsistent (no solution)
                    // Check if equations are proportional
                    if self.are_equations_proportional(
                        *a1_val, *b1_val, *c1_val, *a2_val, *b2_val, *c2_val,
                    ) {
                        SolverResult::InfiniteSolutions
                    } else {
                        SolverResult::NoSolution
                    }
                } else {
                    // Unique solution using Cramer's rule
                    // x = ((-c1)*b2 - (-c2)*b1) / det
                    // y = (a1*(-c2) - a2*(-c1)) / det

                    let x_num = (-c1_val) * b2_val - (-c2_val) * b1_val;
                    let y_num = a1_val * (-c2_val) - a2_val * (-c1_val);

                    let x_sol = if x_num % det == 0 {
                        Expression::integer(x_num / det)
                    } else {
                        Expression::Number(Number::rational(BigRational::new(
                            BigInt::from(x_num),
                            BigInt::from(det),
                        )))
                    };

                    let y_sol = if y_num % det == 0 {
                        Expression::integer(y_num / det)
                    } else {
                        Expression::Number(Number::rational(BigRational::new(
                            BigInt::from(y_num),
                            BigInt::from(det),
                        )))
                    };

                    // Return as vector [x_solution, y_solution]
                    SolverResult::Multiple(vec![x_sol, y_sol])
                }
            }
            _ => SolverResult::NoSolution, // Complex coefficients not supported yet
        }
    }

    /// Check if two equations are proportional (dependent system)
    fn are_equations_proportional(
        &self,
        a1: i64,
        b1: i64,
        c1: i64,
        a2: i64,
        b2: i64,
        c2: i64,
    ) -> bool {
        // Check if (a1, b1, c1) and (a2, b2, c2) are proportional
        // This means a1/a2 = b1/b2 = c1/c2 (handling zero cases)

        if a2 == 0 && b2 == 0 && c2 == 0 {
            return a1 == 0 && b1 == 0 && c1 == 0; // Both equations are 0 = 0
        }

        // Find non-zero coefficient to use as reference
        if a2 != 0 {
            // Check if a1/a2 = b1/b2 = c1/c2
            a1 * b2 == a2 * b1 && a1 * c2 == a2 * c1 && b1 * c2 == b2 * c1
        } else if b2 != 0 {
            // a2 = 0, use b2 as reference
            a1 == 0 && b1 * c2 == b2 * c1
        } else {
            // a2 = b2 = 0, check if c2 != 0 and others are 0
            a1 == 0 && b1 == 0
        }
    }
}

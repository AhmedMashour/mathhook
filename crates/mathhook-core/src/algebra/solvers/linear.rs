//! Solves equations of the form ax + b = 0
//! Includes step-by-step explanations for educational value

use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
// Temporarily simplified for TDD success
use crate::algebra::simplify::Simplify;
use crate::algebra::solvers::{EquationSolver, SolverResult};
use num_bigint::BigInt;
use num_rational::BigRational;

/// Handles linear equations with step-by-step explanations
#[derive(Debug, Clone)]
pub struct LinearSolver {
    /// Enable step-by-step explanations
    pub show_steps: bool,
}

impl LinearSolver {
    /// Create new linear solver
    pub fn new() -> Self {
        Self { show_steps: true }
    }

    /// Create solver without step-by-step (for performance)
    pub fn new_fast() -> Self {
        Self { show_steps: false }
    }
}

impl EquationSolver for LinearSolver {
    /// Solve linear equation ax + b = 0
    #[inline(always)]
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // Handle simplified equations that lost structure
        if let Expression::Number(Number::Integer(0)) = equation {
            // If equation simplified to just 0, it means 0 = 0 (infinite solutions)
            return SolverResult::InfiniteSolutions;
        }
        if let Expression::Number(Number::Integer(n)) = equation {
            if *n != 0 {
                // If equation simplified to non-zero constant, no solution
                return SolverResult::NoSolution;
            }
        }

        // Simplify equation first to flatten nested structures
        let simplified_equation = equation.simplify();

        // Extract coefficients from simplified linear equation
        let (a, b) = self.extract_linear_coefficients(&simplified_equation, variable);

        // 🧠 SMART SOLVER: Analyze original equation structure before simplification

        // Check if original equation has patterns like 0*x + constant
        if let Some(special_result) = self.detect_special_linear_cases(equation, variable) {
            return special_result;
        }

        // Extract coefficients for normal linear analysis
        let a_simplified = a.simplify();
        let b_simplified = b.simplify();

        if a_simplified.is_zero() {
            if b_simplified.is_zero() {
                return SolverResult::InfiniteSolutions; // 0x + 0 = 0
            } else {
                return SolverResult::NoSolution; // 0x + b = 0 where b ≠ 0
            }
        }

        // Solve ax + b = 0 → x = -b/a
        // Use simplified coefficients

        // Check if we can solve numerically
        match (&a_simplified, &b_simplified) {
            (
                Expression::Number(Number::Integer(a_val)),
                Expression::Number(Number::Integer(b_val)),
            ) => {
                if *a_val != 0 {
                    // Simple case: ax + b = 0 → x = -b/a
                    let result = -b_val / a_val;
                    if b_val % a_val == 0 {
                        SolverResult::Single(Expression::integer(result))
                    } else {
                        // Create rational
                        SolverResult::Single(Expression::Number(Number::rational(
                            BigRational::new(BigInt::from(-b_val), BigInt::from(*a_val)),
                        )))
                    }
                } else {
                    SolverResult::NoSolution
                }
            }
            _ => {
                // General case - use simplified coefficients
                let neg_b = self.negate_expression(&b_simplified).simplify();
                let solution = self.divide_expressions(&neg_b, &a_simplified).simplify();

                // Try to evaluate the solution numerically if possible
                let final_solution = self.try_numeric_evaluation(&solution);
                SolverResult::Single(final_solution)
            }
        }
    }

    /// Solve with step-by-step explanation (CRITICAL USER REQUIREMENT)
    /// Using super cool, human-readable messages!
    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        // Simplify equation first to flatten nested structures
        let simplified_equation = equation.simplify();

        // Extract coefficients for analysis
        let (a, b) = self.extract_linear_coefficients(&simplified_equation, variable);

        // Handle special cases with cool explanations
        if a.is_zero() {
            return self.handle_special_case_with_style(&b);
        }

        // Calculate solution
        let a_simplified = a.simplify();
        let b_simplified = b.simplify();
        let neg_b = self.negate_expression(&b_simplified).simplify();
        let solution = self.divide_expressions(&neg_b, &a_simplified).simplify();

        // 🎯 SIMPLE STEP-BY-STEP EXPLANATION (WORKING TDD VERSION)
        let steps = vec![
            Step::new(
                "Given Equation",
                format!("We need to solve: {} = 0", format!("{}", equation)),
            ),
            Step::new(
                "Strategy",
                format!("Isolate {} using inverse operations", variable.name),
            ),
            Step::new(
                "Identify Form",
                format!("This has form: {}·{} + {} = 0", a, variable.name, b),
            ),
            Step::new(
                "Calculate",
                format!("{} = -({}) ÷ {} = {}", variable.name, b, a, solution),
            ),
            Step::new("Solution", format!("{} = {}", variable.name, solution)),
        ];
        let explanation = StepByStepExplanation::new(steps);

        (SolverResult::Single(solution), explanation)
    }

    /// Check if this solver can handle the equation
    fn can_solve(&self, equation: &Expression) -> bool {
        // Check if equation is linear in any variable
        self.is_linear_equation(equation)
    }
}

impl LinearSolver {
    /// Handle special cases with smart step explanations
    fn handle_special_case_with_style(
        &self,
        b: &Expression,
    ) -> (SolverResult, StepByStepExplanation) {
        if b.is_zero() {
            let steps = vec![
                Step::new("Special Case", "0x + 0 = 0 is always true"),
                Step::new("Result", "Infinite solutions - any value of x works"),
            ];
            (
                SolverResult::InfiniteSolutions,
                StepByStepExplanation::new(steps),
            )
        } else {
            let steps = vec![
                Step::new("Special Case", format!("0x + {} = 0 means {} = 0", b, b)),
                Step::new(
                    "Contradiction",
                    format!("But {} ≠ 0, so no solution exists", b),
                ),
            ];
            (SolverResult::NoSolution, StepByStepExplanation::new(steps))
        }
    }
    /// Extract coefficients a and b from equation ax + b = 0
    #[inline(always)]
    fn extract_linear_coefficients(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (Expression, Expression) {
        // First, flatten all nested Add expressions
        let flattened_terms = self.flatten_add_terms(equation);

        let mut coefficient = Expression::integer(0); // Coefficient of variable
        let mut constant = Expression::integer(0); // Constant term

        for term in flattened_terms.iter() {
            match term {
                Expression::Symbol(s) if s == variable => {
                    // x term (coefficient = 1)
                    coefficient = Expression::add(vec![coefficient, Expression::integer(1)]);
                }
                Expression::Mul(factors) => {
                    // Check if this is a variable term (like 2x)
                    let mut var_coeff = Expression::integer(1);
                    let mut has_variable = false;

                    for factor in factors.iter() {
                        match factor {
                            Expression::Symbol(s) if s == variable => {
                                has_variable = true;
                            }
                            _ => {
                                var_coeff = Expression::mul(vec![var_coeff, factor.clone()]);
                            }
                        }
                    }

                    if has_variable {
                        coefficient = Expression::add(vec![coefficient, var_coeff]);
                    } else {
                        constant = Expression::add(vec![constant, term.clone()]);
                    }
                }
                _ => {
                    // Constant term
                    constant = Expression::add(vec![constant, term.clone()]);
                }
            }
        }

        (coefficient, constant)
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

    /// Check if equation is linear
    fn is_linear_equation(&self, equation: &Expression) -> bool {
        // Simplified check - real implementation would be more sophisticated
        match equation {
            Expression::Add(_) => true,    // Assume addition can be linear
            Expression::Symbol(_) => true, // Single variable is linear
            Expression::Number(_) => true, // Constant is linear (degenerate)
            _ => false,                    // Powers, functions are not linear
        }
    }

    /// 🧠 SMART: Detect special linear cases before simplification
    #[inline(always)]
    fn detect_special_linear_cases(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> Option<SolverResult> {
        match equation {
            Expression::Add(terms) if terms.len() == 2 => {
                // Check for patterns: 0*x + constant
                if let [Expression::Mul(factors), constant] = &terms[..] {
                    if factors.len() == 2 {
                        if let [Expression::Number(Number::Integer(0)), var] = &factors[..] {
                            if var == &Expression::symbol(variable.clone()) {
                                // Found 0*x + constant pattern
                                match constant {
                                    Expression::Number(Number::Integer(0)) => {
                                        return Some(SolverResult::InfiniteSolutions);
                                        // 0*x + 0 = 0
                                    }
                                    _ => {
                                        return Some(SolverResult::NoSolution); // 0*x + nonzero = 0
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        None // No special case detected
    }

    /// Negate an expression
    fn negate_expression(&self, expr: &Expression) -> Expression {
        Expression::mul(vec![Expression::integer(-1), expr.clone()])
    }

    /// 🧠 SMART: Evaluate expressions with fraction handling
    #[inline(always)]
    fn try_numeric_evaluation(&self, expr: &Expression) -> Expression {
        match expr {
            // Handle -1 * (complex expression)
            Expression::Mul(factors) if factors.len() == 2 => {
                if let [Expression::Number(Number::Integer(-1)), complex_expr] = &factors[..] {
                    // Evaluate the complex expression and negate it
                    let evaluated = self.evaluate_expression(complex_expr);
                    self.negate_expression(&evaluated).simplify()
                } else {
                    expr.clone()
                }
            }
            // 🧠 SMART: Handle fractions that should be evaluated
            Expression::Function { name, args } if name == "fraction" && args.len() == 2 => {
                self.evaluate_expression(expr)
            }
            _ => expr.clone(),
        }
    }

    /// Evaluate a complex expression to a numeric result
    #[inline(always)]
    fn evaluate_expression(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Add(terms) => {
                let mut total = 0i64;
                for term in terms.iter() {
                    match self.evaluate_expression(term) {
                        Expression::Number(Number::Integer(n)) => total += n,
                        _ => return expr.clone(), // Can't evaluate
                    }
                }
                Expression::integer(total)
            }
            Expression::Mul(factors) => {
                let mut product = 1i64;
                for factor in factors.iter() {
                    match self.evaluate_expression(factor) {
                        Expression::Number(Number::Integer(n)) => product *= n,
                        _ => return expr.clone(), // Can't evaluate
                    }
                }
                Expression::integer(product)
            }
            // Handle fraction functions: fraction(numerator, denominator)
            Expression::Function { name, args } if name == "fraction" && args.len() == 2 => {
                // First evaluate the numerator and denominator
                let num_eval = self.evaluate_expression(&args[0]);
                let den_eval = self.evaluate_expression(&args[1]);

                match (&num_eval, &den_eval) {
                    (
                        Expression::Number(Number::Float(num)),
                        Expression::Number(Number::Float(den)),
                    ) => {
                        if *den != 0.0 {
                            let result = num / den;
                            if result.fract() == 0.0 {
                                Expression::integer(result as i64)
                            } else {
                                Expression::Number(Number::float(result))
                            }
                        } else {
                            expr.clone()
                        }
                    }
                    (
                        Expression::Number(Number::Integer(num)),
                        Expression::Number(Number::Integer(den)),
                    ) => {
                        if *den != 0 {
                            if num % den == 0 {
                                Expression::integer(num / den)
                            } else {
                                Expression::Number(Number::rational(BigRational::new(
                                    BigInt::from(*num),
                                    BigInt::from(*den),
                                )))
                            }
                        } else {
                            expr.clone()
                        }
                    }
                    _ => expr.clone(),
                }
            }
            Expression::Number(_) => expr.clone(),
            _ => expr.clone(),
        }
    }

    /// Divide two expressions (simplified division)
    #[inline(always)]
    fn divide_expressions(&self, numerator: &Expression, denominator: &Expression) -> Expression {
        // First simplify both expressions
        let num_simplified = numerator.simplify();
        let den_simplified = denominator.simplify();

        match (&num_simplified, &den_simplified) {
            // Simple integer division
            (Expression::Number(Number::Integer(n)), Expression::Number(Number::Integer(d))) => {
                if *d != 0 {
                    if n % d == 0 {
                        Expression::integer(n / d)
                    } else {
                        // Create rational number
                        Expression::Number(Number::rational(BigRational::new(
                            BigInt::from(*n),
                            BigInt::from(*d),
                        )))
                    }
                } else {
                    // Division by zero - should be handled as error
                    Expression::integer(0) // Placeholder
                }
            }
            // Try to simplify further - if denominator is 1, just return numerator
            (num, Expression::Number(Number::Integer(1))) => num.clone(),
            // Handle multiplication by -1 and other simple cases
            (Expression::Mul(factors), den) if factors.len() == 2 => {
                if let [Expression::Number(Number::Integer(-1)), expr] = &factors[..] {
                    // -1 * expr / den = -(expr / den)
                    let inner_div = self.divide_expressions(expr, den);
                    Expression::mul(vec![Expression::integer(-1), inner_div]).simplify()
                } else {
                    // General case
                    let fraction =
                        Expression::function("fraction", vec![num_simplified, den_simplified]);
                    fraction.simplify()
                }
            }
            // For linear solver, try to evaluate numerically if possible
            _ => {
                // For now, return as fraction function and let it simplify
                let fraction =
                    Expression::function("fraction", vec![num_simplified, den_simplified]);
                fraction.simplify()
            }
        }
    }
}

// ============================================================================
// 🧪 UNIT TESTS (INTERNAL VALIDATION)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coefficient_extraction() {
        let x = Symbol::new("x");
        let solver = LinearSolver::new();

        // Test 2x + 3
        let equation = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(3),
        ]);

        let (a, b) = solver.extract_linear_coefficients(&equation, &x);
        // The coefficient might be Mul([1, 2]) so we need to simplify it
        assert_eq!(a.simplify(), Expression::integer(2));
        assert_eq!(b.simplify(), Expression::integer(3));
    }

    #[test]
    fn test_linear_detection() {
        let x = Symbol::new("x");
        let solver = LinearSolver::new();

        // Linear equation
        let linear = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(solver.is_linear_equation(&linear));

        // Non-linear equation (power)
        let nonlinear = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(!solver.is_linear_equation(&nonlinear));
    }
}

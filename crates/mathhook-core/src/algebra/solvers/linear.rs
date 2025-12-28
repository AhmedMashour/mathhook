//! Solves equations of the form ax + b = 0
//! Includes step-by-step explanations for educational value

use crate::algebra::Expand;
use crate::core::constants::EPSILON;
use crate::core::{Commutativity, Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
// Temporarily simplified for TDD success
use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::simplify::Simplify;
use num_bigint::BigInt;
use num_rational::BigRational;

/// Handles linear equations with step-by-step explanations
#[derive(Debug, Clone)]
pub struct LinearSolver {
    /// Enable step-by-step explanations
    pub show_steps: bool,
}

impl Default for LinearSolver {
    fn default() -> Self {
        Self::new()
    }
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
    ///
    /// Fractional solutions are automatically simplified to lowest terms via
    /// `BigRational::new()`, which reduces fractions using GCD. Integer solutions
    /// (where numerator is divisible by denominator) are returned as integers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::solvers::{linear::LinearSolver, EquationSolver, SolverResult};
    /// use mathhook_core::core::{Expression, Number};
    /// use mathhook_core::symbol;
    /// use num_bigint::BigInt;
    ///
    /// let solver = LinearSolver::new_fast();
    /// let x = symbol!(x);
    ///
    /// // Example: 4x = 6 gives x = 3/2 (simplified from 6/4)
    /// let equation = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(4), Expression::symbol(x.clone())]),
    ///     Expression::integer(-6),
    /// ]);
    ///
    /// match solver.solve(&equation, &x) {
    ///     SolverResult::Single(solution) => {
    ///         if let Expression::Number(Number::Rational(r)) = solution {
    ///             assert_eq!(r.numer(), &BigInt::from(3));
    ///             assert_eq!(r.denom(), &BigInt::from(2));
    ///         }
    ///     }
    ///     _ => panic!("Expected single solution"),
    /// }
    /// ```
    #[inline(always)]
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // Handle Relation type (equations like x = 5)
        let equation_expr = if let Expression::Relation(data) = equation {
            // Convert relation to expression: left - right = 0
            Expression::add(vec![
                data.left.clone(),
                Expression::mul(vec![Expression::integer(-1), data.right.clone()]),
            ])
        } else {
            equation.clone()
        };

        // Check for noncommutative symbols - delegate to MatrixEquationSolver if found
        if equation_expr.commutativity() != Commutativity::Commutative {
            use crate::algebra::solvers::matrix_equations::MatrixEquationSolver;
            let matrix_solver = MatrixEquationSolver::new_fast();
            return matrix_solver.solve(&equation_expr, variable);
        }

        // Simplify and expand equation to flatten nested structures and distribute multiplication
        let simplified_equation = equation_expr.simplify().expand();

        // Check for identity equations (0 = 0) or contradictions AFTER simplification
        if simplified_equation.is_zero() {
            // If equation simplified to just 0, it means 0 = 0 (infinite solutions)
            return SolverResult::InfiniteSolutions;
        }
        // Check for non-zero constant (contradiction)
        if let Expression::Number(Number::Integer(n)) = simplified_equation {
            if n != 0 {
                return SolverResult::NoSolution;
            }
        }

        // Check for factored form: (x - a)(x - b)...(x - n) = 0
        if let Some(roots) = self.extract_factored_roots(&simplified_equation, variable) {
            if roots.len() == 1 {
                return SolverResult::Single(roots[0].clone());
            } else if roots.len() > 1 {
                return SolverResult::Multiple(roots);
            }
        }

        // Extract coefficients from simplified linear equation
        let (a, b) = self.extract_linear_coefficients(&simplified_equation, variable);

        // Smart solver: Analyze original equation structure before simplification

        // Check if original equation has patterns like 0*x + constant
        if let Some(special_result) = self.detect_special_linear_cases(&equation_expr, variable) {
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
        // Fractions are automatically reduced to lowest terms by BigRational::new()

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
                        // Integer solution: return as integer (e.g., 10/5 = 2)
                        SolverResult::Single(Expression::integer(result))
                    } else {
                        // Fractional solution: BigRational::new() automatically reduces to lowest terms
                        // Example: 6/4 → 3/2, 18/12 → 3/2
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
                let neg_b = b_simplified.negate().simplify();
                let solution = Self::divide_expressions(&neg_b, &a_simplified).simplify();

                // Try to evaluate the solution numerically if possible
                let final_solution = Self::try_eval_numeric_internal(&solution);
                SolverResult::Single(final_solution)
            }
        }
    }

    /// Solve with step-by-step explanation
    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let simplified_equation = equation.simplify();
        let (a, b) = self.extract_linear_coefficients(&simplified_equation, variable);

        if a.is_zero() {
            return self.handle_special_case_with_style(&b);
        }

        let a_simplified = a.simplify();
        let b_simplified = b.simplify();
        let neg_b = b_simplified.negate().simplify();
        let solution = Self::divide_expressions(&neg_b, &a_simplified).simplify();

        let steps = vec![
            Step::new(
                "Given Equation",
                format!("We need to solve: {} = 0", equation),
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
    /// Handle special cases with step explanations
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
        let flattened_terms = equation.flatten_add_terms();

        let mut coefficient = Expression::integer(0); // Coefficient of variable
        let mut constant = Expression::integer(0); // Constant term

        for term in flattened_terms.iter() {
            match term {
                Expression::Symbol(s) if s == variable => {
                    coefficient = Expression::add(vec![coefficient, Expression::integer(1)]);
                }
                Expression::Mul(factors) => {
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

    /// Check if equation is linear
    fn is_linear_equation(&self, equation: &Expression) -> bool {
        matches!(
            equation,
            Expression::Add(_) | Expression::Symbol(_) | Expression::Number(_)
        )
    }

    /// Detect special linear cases before simplification
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

    /// Extract roots from factored polynomial form: (x - a)(x - b) = 0
    fn extract_factored_roots(
        &self,
        expr: &Expression,
        variable: &Symbol,
    ) -> Option<Vec<Expression>> {
        match expr {
            Expression::Mul(factors) => {
                let mut roots = Vec::new();

                for factor in factors.iter() {
                    // Check if this factor is (x - constant) or (constant - x)
                    if let Expression::Add(terms) = factor {
                        if terms.len() == 2 {
                            // Check pattern: x + (-a) = 0 → x = a
                            if let [Expression::Symbol(s), Expression::Mul(neg_factors)] =
                                &terms[..]
                            {
                                if s == variable && neg_factors.len() == 2 {
                                    if let [Expression::Number(Number::Integer(-1)), constant] =
                                        &neg_factors[..]
                                    {
                                        roots.push(constant.clone());
                                        continue;
                                    }
                                }
                            }
                            // Check pattern: -a + x = 0 → x = a
                            if let [Expression::Mul(neg_factors), Expression::Symbol(s)] =
                                &terms[..]
                            {
                                if s == variable && neg_factors.len() == 2 {
                                    if let [Expression::Number(Number::Integer(-1)), constant] =
                                        &neg_factors[..]
                                    {
                                        roots.push(constant.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }

                if roots.is_empty() {
                    None
                } else {
                    Some(roots)
                }
            }
            _ => None,
        }
    }

    /// Internal domain-specific optimization for linear solver
    ///
    /// Evaluate expressions with fraction handling for linear equation solutions.
    /// This is a specialized version optimized for the linear solver's needs.
    ///
    /// Static helper function - doesn't depend on instance state.
    #[inline(always)]
    fn try_eval_numeric_internal(expr: &Expression) -> Expression {
        match expr {
            // Handle -1 * (complex expression)
            Expression::Mul(factors) if factors.len() == 2 => {
                if let [Expression::Number(Number::Integer(-1)), complex_expr] = &factors[..] {
                    // Evaluate the complex expression and negate it
                    let evaluated = Self::eval_exact_internal(complex_expr);
                    evaluated.negate().simplify()
                } else {
                    expr.clone()
                }
            }
            // Handle fractions that should be evaluated
            Expression::Function { name, args }
                if name.as_ref() == "fraction" && args.len() == 2 =>
            {
                Self::eval_exact_internal(expr)
            }
            _ => expr.clone(),
        }
    }

    /// Internal domain-specific optimization for linear solver
    ///
    /// Static helper function for exact arithmetic evaluation.
    /// Preserves exact arithmetic (integers/rationals) without instance state dependency.
    ///
    /// This method is kept separate from Expression::evaluate_to_f64() because it maintains
    /// mathematical exactness. For example, 1/3 stays as Rational(1,3), not 0.333...
    ///
    /// # Why Not Use evaluate_to_f64()?
    ///
    /// - evaluate_to_f64() converts to f64 (loses precision: 1/3 → 0.333...)
    /// - This method preserves rationals (keeps exactness: 1/3 → Rational(1,3))
    /// - Linear equation solutions often require exact fractions (e.g., x = 2/3)
    ///
    /// # Automatic Fraction Simplification
    ///
    /// When creating rational numbers via `BigRational::new(num, den)`, fractions are
    /// automatically reduced to lowest terms using GCD. For example:
    /// - `BigRational::new(6, 4)` → 3/2
    /// - `BigRational::new(18, 12)` → 3/2
    /// - `BigRational::new(10, 5)` → 2 (returned as integer if denominator is 1)
    ///
    /// # Returns
    ///
    /// - Expression::Number(Integer) for exact integer results
    /// - Expression::Number(Rational) for exact fractional results (automatically simplified)
    /// - Original expression if cannot be evaluated exactly
    #[inline(always)]
    fn eval_exact_internal(expr: &Expression) -> Expression {
        match expr {
            Expression::Add(terms) => {
                let mut total = 0i64;
                for term in terms.iter() {
                    match Self::eval_exact_internal(term) {
                        Expression::Number(Number::Integer(n)) => total += n,
                        _ => return expr.clone(), // Can't evaluate
                    }
                }
                Expression::integer(total)
            }
            Expression::Mul(factors) => {
                let mut product = 1i64;
                for factor in factors.iter() {
                    match Self::eval_exact_internal(factor) {
                        Expression::Number(Number::Integer(n)) => product *= n,
                        _ => return expr.clone(), // Can't evaluate
                    }
                }
                Expression::integer(product)
            }
            // Handle fraction functions: fraction(numerator, denominator)
            // BigRational::new() automatically reduces to lowest terms
            Expression::Function { name, args }
                if name.as_ref() == "fraction" && args.len() == 2 =>
            {
                // First evaluate the numerator and denominator
                let num_eval = Self::eval_exact_internal(&args[0]);
                let den_eval = Self::eval_exact_internal(&args[1]);

                match (&num_eval, &den_eval) {
                    (
                        Expression::Number(Number::Float(num)),
                        Expression::Number(Number::Float(den)),
                    ) => {
                        if den.abs() >= EPSILON {
                            let result = num / den;
                            if result.fract().abs() < EPSILON {
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
                                // BigRational::new() automatically reduces to lowest terms via GCD
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
    ///
    /// Static helper function for recursive division operations.
    /// Does not require instance state, only performs expression manipulation.
    ///
    /// Fractions created via `BigRational::new()` are automatically reduced
    /// to lowest terms using GCD.
    #[inline(always)]
    fn divide_expressions(numerator: &Expression, denominator: &Expression) -> Expression {
        // First simplify both expressions
        let num_simplified = numerator.simplify();
        let den_simplified = denominator.simplify();

        match (&num_simplified, &den_simplified) {
            // Simple integer division
            // BigRational::new() automatically reduces to lowest terms
            (Expression::Number(Number::Integer(n)), Expression::Number(Number::Integer(d))) => {
                if *d != 0 {
                    if n % d == 0 {
                        Expression::integer(n / d)
                    } else {
                        // Create rational number - automatically reduced to lowest terms
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
            // Integer divided by rational: a / (p/q) = a * (q/p)
            (Expression::Number(Number::Integer(n)), Expression::Number(Number::Rational(r))) => {
                // a / (p/q) = a * q / p
                let inverted = BigRational::new(r.denom().clone(), r.numer().clone());
                let result = BigRational::from(BigInt::from(*n)) * inverted;

                // Simplify to integer if possible
                if result.is_integer() {
                    Expression::integer(result.numer().to_string().parse().unwrap())
                } else {
                    Expression::Number(Number::rational(result))
                }
            }
            // Rational divided by integer: (p/q) / a = (p/q) / a = p/(q*a)
            (Expression::Number(Number::Rational(r)), Expression::Number(Number::Integer(d))) => {
                if *d != 0 {
                    let result = (**r).clone() / BigRational::from(BigInt::from(*d));
                    if result.is_integer() {
                        Expression::integer(result.numer().to_string().parse().unwrap())
                    } else {
                        Expression::Number(Number::rational(result))
                    }
                } else {
                    Expression::integer(0) // Placeholder
                }
            }
            // Rational divided by rational
            (
                Expression::Number(Number::Rational(num_r)),
                Expression::Number(Number::Rational(den_r)),
            ) => {
                let result = (**num_r).clone() / (**den_r).clone();
                if result.is_integer() {
                    Expression::integer(result.numer().to_string().parse().unwrap())
                } else {
                    Expression::Number(Number::rational(result))
                }
            }
            // Try to simplify further - if denominator is 1, just return numerator
            (num, Expression::Number(Number::Integer(1))) => num.clone(),
            // Handle multiplication by -1 and other simple cases
            (Expression::Mul(factors), den) if factors.len() == 2 => {
                if let [Expression::Number(Number::Integer(-1)), expr] = &factors[..] {
                    // -1 * expr / den = -(expr / den)
                    let inner_div = Self::divide_expressions(expr, den);
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
                // Return as fraction function and let it simplify
                let fraction =
                    Expression::function("fraction", vec![num_simplified, den_simplified]);
                fraction.simplify()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_coefficient_extraction() {
        let x = symbol!(x);
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
        let x = symbol!(x);
        let solver = LinearSolver::new();

        // Linear equation
        let linear = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(solver.is_linear_equation(&linear));

        // Non-linear equation (power)
        let nonlinear = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(!solver.is_linear_equation(&nonlinear));
    }
}

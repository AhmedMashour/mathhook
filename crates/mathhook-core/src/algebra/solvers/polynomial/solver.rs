//! Core polynomial solver logic for cubic and quartic equations

use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
use crate::symbol;

/// Polynomial equation solver
#[derive(Debug, Clone)]
pub struct PolynomialSolver;

impl PolynomialSolver {
    pub fn new() -> Self {
        Self
    }

    /// Find the degree of polynomial in given variable
    pub fn find_polynomial_degree(&self, expr: &Expression, variable: &Symbol) -> u32 {
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
    pub fn solve_cubic(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        if let Expression::Add(terms) = equation {
            if terms.len() == 2 {
                let (power_term, constant_term) = match (&terms[0], &terms[1]) {
                    (Expression::Number(Number::Integer(c)), p@Expression::Pow(..)) => (p, c),
                    (p@Expression::Pow(..), Expression::Number(Number::Integer(c))) => (p, c),
                    _ => {
                        return self.solve_cubic_rational_root_theorem(equation, variable);
                    }
                };

                if let Expression::Pow(base, exp) = power_term {
                    if **base == Expression::symbol(variable.clone())
                        && **exp == Expression::integer(3)
                    {
                        let cube_root_value = (-constant_term as f64).cbrt();

                        if cube_root_value.fract() == 0.0 {
                            let real_root = Expression::integer(cube_root_value as i64);
                            return SolverResult::Partial(vec![real_root]);
                        }
                    }
                }
            }
        }

        self.solve_cubic_rational_root_theorem(equation, variable)
    }

    /// Try to solve cubic using rational root theorem
    pub fn solve_cubic_rational_root_theorem(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
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
    pub fn solve_quartic(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        if let Expression::Add(terms) = equation {
            if terms.len() == 2 {
                let (power_term, constant_term) = match (&terms[0], &terms[1]) {
                    (Expression::Number(Number::Integer(c)), p@Expression::Pow(..)) => (p, c),
                    (p@Expression::Pow(..), Expression::Number(Number::Integer(c))) => (p, c),
                    _ => {
                        return self.solve_quartic_rational_root_theorem(equation, variable);
                    }
                };

                if let Expression::Pow(base, exp) = power_term {
                    if **base == Expression::symbol(variable.clone())
                        && **exp == Expression::integer(4)
                    {
                        let fourth_root_value = (-constant_term as f64).powf(0.25);

                        if fourth_root_value.is_finite() {
                            let real_root = fourth_root_value.abs();

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

                            return SolverResult::Partial(vec![real_root_expr, neg_real_root_expr]);
                        }
                    }
                }
            }
        }

        self.solve_quartic_rational_root_theorem(equation, variable)
    }

    /// Try to solve quartic using rational root theorem
    pub fn solve_quartic_rational_root_theorem(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
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
    pub fn evaluate_polynomial_at(
        &self,
        poly: &Expression,
        variable: &Symbol,
        value: &Expression,
    ) -> Expression {
        self.substitute_variable(poly, variable, value).simplify()
    }

    /// Substitute variable with value in expression
    pub fn substitute_variable(
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

    /// Extract constant term and leading coefficient from polynomial
    pub fn extract_constant_and_leading(&self, poly: &Expression, _variable: &Symbol) -> (i64, i64) {
        match poly {
            Expression::Add(terms) => {
                let mut constant = 0i64;
                let mut leading = 1i64;

                for term in terms.iter() {
                    if let Expression::Number(Number::Integer(n)) = term {
                        constant = *n;
                    } else if let Expression::Pow(_, exp) = term {
                        if let Expression::Number(Number::Integer(_)) = exp.as_ref() {
                            leading = 1;
                        }
                    }
                }

                (constant, leading)
            }
            _ => (0, 1),
        }
    }

    /// Get all divisors of a number
    pub fn get_divisors(&self, n: i64) -> Vec<i64> {
        if n == 0 {
            return vec![1];
        }

        let n = n.abs();
        let mut divisors = Vec::new();

        for i in 1..=n {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                divisors.push(i);
                if i != n / i {
                    divisors.push(n / i);
                }
            }
        }

        divisors.sort();
        divisors
    }
}

impl EquationSolver for PolynomialSolver {
    #[inline(always)]
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        let degree = self.find_polynomial_degree(equation, variable);

        match degree {
            3 => self.solve_cubic(equation, variable),
            4 => self.solve_quartic(equation, variable),
            _ => SolverResult::NoSolution,
        }
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, crate::educational::step_by_step::StepByStepExplanation) {
        super::educational::solve_with_explanation(self, equation, variable)
    }

    fn can_solve(&self, equation: &Expression) -> bool {
        let degree = self.find_polynomial_degree(equation, &symbol!(x));
        degree >= 3 && degree <= 4
    }
}

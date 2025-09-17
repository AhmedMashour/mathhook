//! Hessian matrix operations for second-order partial derivatives
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
/// Hessian matrix operations
pub struct HessianOperations;
impl HessianOperations {
    /// Compute Hessian matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::HessianOperations;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let hessian = HessianOperations::compute(&expr, &[x, y]);
    /// ```
    pub fn compute(expr: &Expression, variables: &[Symbol]) -> Vec<Vec<Expression>> {
        let n = variables.len();
        let mut hessian = Vec::with_capacity(n);
        for _ in 0..n {
            hessian.push(Vec::with_capacity(n));
        }
        for i in 0..n {
            for j in 0..n {
                if j >= i {
                    let second_partial = expr
                        .derivative(variables[i].clone())
                        .derivative(variables[j].clone())
                        .simplify();
                    hessian[i].push(second_partial);
                } else {
                    let symmetric_entry = hessian[j][i].clone();
                    hessian[i].push(symmetric_entry);
                }
            }
        }
        hessian
    }
    /// Compute Hessian determinant
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::HessianOperations;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let det = HessianOperations::determinant(&expr, vec![x, y]);
    /// ```
    pub fn determinant(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let hessian = Self::compute(expr, &variables);
        Self::matrix_determinant(&hessian)
    }
    /// Compute matrix determinant recursively
    fn matrix_determinant(matrix: &[Vec<Expression>]) -> Expression {
        let n = matrix.len();
        match n {
            0 => Expression::integer(1),
            1 => matrix[0][0].clone(),
            2 => {
                let a = &matrix[0][0];
                let b = &matrix[0][1];
                let c = &matrix[1][0];
                let d = &matrix[1][1];
                Expression::add(vec![
                    Expression::mul(vec![a.clone(), d.clone()]),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::mul(vec![b.clone(), c.clone()]),
                    ]),
                ])
                .simplify()
            }
            _ => {
                let mut det_terms = Vec::with_capacity(n);
                for j in 0..n {
                    let cofactor = Self::cofactor(matrix, 0, j);
                    let sign = if j % 2 == 0 { 1 } else { -1 };
                    det_terms.push(Expression::mul(vec![
                        Expression::integer(sign),
                        matrix[0][j].clone(),
                        cofactor,
                    ]));
                }
                Expression::add(det_terms).simplify()
            }
        }
    }
    /// Compute cofactor for matrix determinant
    fn cofactor(matrix: &[Vec<Expression>], row: usize, col: usize) -> Expression {
        let n = matrix.len();
        let minor: Vec<Vec<_>> = (0..n)
            .filter(|&i| i != row)
            .map(|i| {
                (0..n)
                    .filter(|&j| j != col)
                    .map(|j| matrix[i][j].clone())
                    .collect()
            })
            .collect();
        Self::matrix_determinant(&minor)
    }
    /// Check if Hessian is positive definite (for optimization)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::HessianOperations;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let is_pos_def = HessianOperations::is_positive_definite(&expr, vec![x, y]);
    /// ```
    pub fn is_positive_definite(expr: &Expression, variables: Vec<Symbol>) -> bool {
        let hessian = Self::compute(expr, &variables);
        Self::check_positive_definite(&hessian)
    }
    /// Check positive definiteness using leading principal minors
    fn check_positive_definite(hessian: &[Vec<Expression>]) -> bool {
        let n = hessian.len();
        for k in 1..=n {
            let submatrix: Vec<Vec<_>> = (0..k)
                .map(|i| (0..k).map(|j| hessian[i][j].clone()).collect())
                .collect();
            let det = Self::matrix_determinant(&submatrix);
            if det.is_zero() {
                return false;
            }
        }
        true
    }
    /// Compute trace of Hessian matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::HessianOperations;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(3), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
    ///     Expression::mul(vec![Expression::integer(5), Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))])
    /// ]);
    /// let trace = HessianOperations::trace(&expr, vec![x, y]);
    /// ```
    pub fn trace(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let hessian = Self::compute(expr, &variables);
        let n = hessian.len();
        let mut diagonal_terms = Vec::with_capacity(n);
        diagonal_terms.extend((0..n).map(|i| hessian[i][i].clone()));
        Expression::add(diagonal_terms).simplify()
    }
}
#[cfg(test)]
mod tests {
    use std::slice::from_ref;

    use super::*;
    use crate::expr;
    use crate::symbol;
    #[test]
    fn test_quadratic_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let quadratic = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        let hessian = HessianOperations::compute(&quadratic, &[x.clone(), y.clone()]);
        assert_eq!(hessian.len(), 2);
        assert_eq!(hessian[0].len(), 2);
        assert_eq!(hessian[1].len(), 2);
        assert_eq!(hessian[0][0].simplify(), Expression::integer(2));
        assert_eq!(hessian[1][1].simplify(), Expression::integer(2));
        assert_eq!(hessian[0][1].simplify(), Expression::integer(0));
        assert_eq!(hessian[1][0].simplify(), Expression::integer(0));
    }
    #[test]
    fn test_mixed_partial_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let mixed = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        let hessian = HessianOperations::compute(&mixed, &[x.clone(), y.clone()]);
        assert_eq!(hessian[0][0].simplify(), Expression::integer(0));
        assert_eq!(hessian[1][1].simplify(), Expression::integer(0));
        assert_eq!(hessian[0][1].simplify(), Expression::integer(1));
        assert_eq!(hessian[1][0].simplify(), Expression::integer(1));
    }
    #[test]
    fn test_cubic_polynomial_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let cubic = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::mul(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::symbol(y.clone()),
            ]),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(3)),
        ]);
        let hessian = HessianOperations::compute(&cubic, &[x.clone(), y.clone()]);
        assert_eq!(hessian.len(), 2);
        assert!(!hessian[0][0].is_zero());
        assert!(!hessian[1][1].is_zero());
        assert!(!hessian[0][1].is_zero());
        assert!(!hessian[1][0].is_zero());
    }
    #[test]
    fn test_single_variable_hessian() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(4));
        let hessian = HessianOperations::compute(&expr, from_ref(&x));
        assert_eq!(hessian.len(), 1);
        assert_eq!(hessian[0].len(), 1);
        assert!(!hessian[0][0].is_zero());
    }
    #[test]
    fn test_three_variable_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(z.clone()), Expression::integer(2)),
        ]);
        let hessian = HessianOperations::compute(&expr, &[x.clone(), y.clone(), z.clone()]);
        assert_eq!(hessian.len(), 3);
        for (i, row) in hessian.iter().enumerate().take(3) {
            assert_eq!(row.len(), 3);
            for (j, val) in row.iter().enumerate().take(3) {
                let expected = if i == j {
                    Expression::integer(2)
                } else {
                    Expression::integer(0)
                };
                assert_eq!(val.simplify(), expected);
            }
        }
    }
    #[test]
    fn test_hessian_determinant_2x2() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        let det = HessianOperations::determinant(&expr, vec![x.clone(), y.clone()]);
        assert_eq!(det.simplify(), Expression::integer(4));
    }
    #[test]
    fn test_hessian_trace() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![
                Expression::integer(5),
                Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            ]),
        ]);
        let trace = HessianOperations::trace(&expr, vec![x.clone(), y.clone()]);
        assert_eq!(trace.simplify(), Expression::integer(16));
    }
    #[test]
    fn test_constant_function_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let constant = Expression::integer(42);
        let hessian = HessianOperations::compute(&constant, &[x.clone(), y.clone()]);
        for value in hessian.iter().flatten() {
            assert_eq!(value.simplify(), expr!(0));
        }
    }
    #[test]
    fn test_linear_function_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let linear = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
            Expression::integer(1),
        ]);
        let hessian = HessianOperations::compute(&linear, &[x.clone(), y.clone()]);
        for value in hessian.iter().flatten() {
            assert_eq!(value.simplify(), expr!(0));
        }
    }
    #[test]
    fn test_hessian_symmetry() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::symbol(y.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            ]),
        ]);
        let hessian = HessianOperations::compute(&expr, &[x.clone(), y.clone()]);
        assert_eq!(hessian[0][1], hessian[1][0]);
    }
    #[test]
    fn test_trigonometric_hessian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let trig_expr = Expression::add(vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(y.clone())]),
        ]);
        let hessian = HessianOperations::compute(&trig_expr, &[x.clone(), y.clone()]);
        assert_eq!(hessian.len(), 2);
        assert!(!hessian[0][0].is_zero());
        assert!(!hessian[1][1].is_zero());
        assert_eq!(hessian[0][1].simplify(), Expression::integer(0));
        assert_eq!(hessian[1][0].simplify(), Expression::integer(0));
    }
}

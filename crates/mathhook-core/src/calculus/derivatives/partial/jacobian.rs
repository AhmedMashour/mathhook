//! Jacobian matrix operations for vector-valued functions

use super::{gradient::GradientOperations, utils::MatrixUtils};
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
use std::collections::HashMap;

/// Jacobian matrix operations
pub struct JacobianOperations;

impl JacobianOperations {
    /// Compute Jacobian matrix for vector-valued function F: ℝⁿ → ℝᵐ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::calculus::derivatives::PartialUtils;
    /// use mathhook_core::calculus::derivatives::MatrixUtils;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianOperations;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let functions = vec![
    ///     Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]),
    ///     Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())])
    /// ];
    /// let variables = vec![x, y];
    /// let jacobian = JacobianOperations::compute(&functions, &variables);
    /// ```
    pub fn compute(functions: &[Expression], variables: &[Symbol]) -> Vec<Vec<Expression>> {
        let m = functions.len();
        let mut jacobian = Vec::with_capacity(m);

        for function in functions {
            let gradient = GradientOperations::compute(function, variables.to_vec());
            jacobian.push(gradient);
        }

        jacobian
    }

    /// Compute Jacobian matrix with caching for repeated variable sets
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianOperations;
    /// use std::collections::HashMap;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let functions = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone())
    /// ];
    /// let variables = vec![x, y];
    /// let mut cache = HashMap::new();
    /// let jacobian = JacobianOperations::compute_cached(&functions, &variables, &mut cache);
    /// ```
    pub fn compute_cached(
        functions: &[Expression],
        variables: &[Symbol],
        cache: &mut HashMap<String, Expression>,
    ) -> Vec<Vec<Expression>> {
        let m = functions.len();
        let n = variables.len();
        let mut jacobian = Vec::with_capacity(m);

        for (func_idx, function) in functions.iter().enumerate() {
            let mut gradient_row = Vec::with_capacity(n);

            for (var_idx, var) in variables.iter().enumerate() {
                let cache_key = format!("f{}_{}", func_idx, var_idx);
                let partial = cache
                    .entry(cache_key)
                    .or_insert_with(|| function.derivative(var.clone()).simplify())
                    .clone();
                gradient_row.push(partial);
            }

            jacobian.push(gradient_row);
        }

        jacobian
    }

    /// Compute transpose of Jacobian matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianOperations;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let functions = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone())
    /// ];
    /// let variables = vec![x, y];
    /// let jacobian_t = JacobianOperations::transpose(&functions, &variables);
    /// ```
    pub fn transpose(functions: &[Expression], variables: &[Symbol]) -> Vec<Vec<Expression>> {
        let jacobian = Self::compute(functions, variables);
        Self::matrix_transpose(&jacobian)
    }

    /// Helper function to transpose a matrix
    fn matrix_transpose(matrix: &[Vec<Expression>]) -> Vec<Vec<Expression>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        (0..cols)
            .map(|j| (0..rows).map(|i| matrix[i][j].clone()).collect())
            .collect()
    }
}

/// Jacobian determinant operations
pub struct JacobianDeterminant;

impl JacobianDeterminant {
    /// Compute Jacobian determinant
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianDeterminant;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let functions = vec![
    ///     Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]),
    ///     Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())])
    /// ];
    /// let variables = vec![x, y];
    /// let jac_det = JacobianDeterminant::compute(&functions, &variables);
    /// ```
    pub fn compute(functions: &[Expression], variables: &[Symbol]) -> Expression {
        if functions.len() != variables.len() {
            panic!(
                "Jacobian determinant requires square matrix: {} functions vs {} variables",
                functions.len(),
                variables.len()
            );
        }

        let jacobian_matrix = JacobianOperations::compute(functions, variables);
        MatrixUtils::determinant(&jacobian_matrix)
    }

    /// Compute absolute value of Jacobian determinant (for coordinate transformations)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianDeterminant;
    ///
    /// let r = symbol!(r);
    /// let theta = symbol!(theta);
    /// let functions = vec![
    ///     Expression::mul(vec![
    ///         Expression::symbol(r.clone()),
    ///         Expression::function("cos", vec![Expression::symbol(theta.clone())])
    ///     ]),
    ///     Expression::mul(vec![
    ///         Expression::symbol(r.clone()),
    ///         Expression::function("sin", vec![Expression::symbol(theta.clone())])
    ///     ])
    /// ];
    /// let variables = vec![r, theta];
    /// let abs_jac_det = JacobianDeterminant::absolute(&functions, &variables);
    /// ```
    pub fn absolute(functions: &[Expression], variables: &[Symbol]) -> Expression {
        let det = Self::compute(functions, variables);
        Expression::function("abs", vec![det])
    }

    /// Check if Jacobian is singular (determinant = 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianDeterminant;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let functions = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(x.clone())  // Linearly dependent
    /// ];
    /// let variables = vec![x, y];
    /// let is_singular = JacobianDeterminant::is_singular(&functions, &variables);
    /// ```
    pub fn is_singular(functions: &[Expression], variables: &[Symbol]) -> bool {
        let det = Self::compute(functions, variables);
        super::utils::PartialUtils::is_zero(&det)
    }

    /// Compute condition number estimate (for numerical stability analysis)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::JacobianDeterminant;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let functions = vec![
    ///     Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]),
    ///     Expression::add(vec![
    ///         Expression::symbol(x.clone()),
    ///         Expression::mul(vec![
    ///             Expression::mul(vec![
    ///                 Expression::integer(1),
    ///                 Expression::pow(Expression::integer(1000), Expression::integer(-1))
    ///             ]),
    ///             Expression::symbol(y.clone())
    ///         ])
    ///     ])
    /// ];
    /// let variables = vec![x, y];
    /// let condition = JacobianDeterminant::condition_estimate(&functions, &variables);
    /// ```
    pub fn condition_estimate(functions: &[Expression], variables: &[Symbol]) -> Expression {
        let det = Self::compute(functions, variables);
        let abs_det = Expression::function("abs", vec![det]);

        Expression::pow(abs_det, Expression::integer(-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_linear_transformation_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::add(vec![
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
                Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
            ]),
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::mul(vec![Expression::integer(4), Expression::symbol(y.clone())]),
            ]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 2);
        assert_eq!(jacobian[0].len(), 2);
        assert_eq!(jacobian[1].len(), 2);

        assert_eq!(jacobian[0][0].simplify(), Expression::integer(2));
        assert_eq!(jacobian[0][1].simplify(), Expression::integer(3));
        assert_eq!(jacobian[1][0].simplify(), Expression::integer(1));
        assert_eq!(jacobian[1][1].simplify(), Expression::integer(4));
    }

    #[test]
    fn test_nonlinear_transformation_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 2);
        assert_eq!(jacobian[0].len(), 2);
        assert_eq!(jacobian[1].len(), 2);

        assert!(!jacobian[0][0].is_zero());
        assert_eq!(jacobian[0][1].simplify(), Expression::integer(0));
        assert!(!jacobian[1][0].is_zero());
        assert!(!jacobian[1][1].is_zero());
    }

    #[test]
    fn test_polar_to_cartesian_jacobian() {
        let r = symbol!(r);
        let theta = symbol!(theta);

        let functions = vec![
            Expression::mul(vec![
                Expression::symbol(r.clone()),
                Expression::function("cos", vec![Expression::symbol(theta.clone())]),
            ]),
            Expression::mul(vec![
                Expression::symbol(r.clone()),
                Expression::function("sin", vec![Expression::symbol(theta.clone())]),
            ]),
        ];

        let variables = vec![r.clone(), theta.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 2);
        assert_eq!(jacobian[0].len(), 2);
        assert_eq!(jacobian[1].len(), 2);

        assert!(!jacobian[0][0].is_zero());
        assert!(!jacobian[0][1].is_zero());
        assert!(!jacobian[1][0].is_zero());
        assert!(!jacobian[1][1].is_zero());
    }

    #[test]
    fn test_single_variable_jacobian() {
        let x = symbol!(x);

        let functions = vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )];

        let variables = vec![x.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 1);
        assert_eq!(jacobian[0].len(), 1);
        assert!(!jacobian[0][0].is_zero());
    }

    #[test]
    fn test_three_variable_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);

        let functions = vec![
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
                Expression::symbol(z.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::pow(Expression::symbol(z.clone()), Expression::integer(2)),
        ];

        let variables = vec![x.clone(), y.clone(), z.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 3);
        for row in &jacobian {
            assert_eq!(row.len(), 3);
        }

        assert_eq!(jacobian[0][0].simplify(), Expression::integer(1));
        assert_eq!(jacobian[0][1].simplify(), Expression::integer(1));
        assert_eq!(jacobian[0][2].simplify(), Expression::integer(1));

        assert!(!jacobian[1][0].is_zero());
        assert!(!jacobian[1][1].is_zero());
        assert_eq!(jacobian[1][2].simplify(), Expression::integer(0));

        assert_eq!(jacobian[2][0].simplify(), Expression::integer(0));
        assert_eq!(jacobian[2][1].simplify(), Expression::integer(0));
        assert!(!jacobian[2][2].is_zero());
    }

    #[test]
    fn test_jacobian_caching() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(y.clone())]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let mut cache = HashMap::new();

        let jacobian1 = JacobianOperations::compute_cached(&functions, &variables, &mut cache);
        let jacobian2 = JacobianOperations::compute_cached(&functions, &variables, &mut cache);

        assert_eq!(jacobian1.len(), 2);
        assert_eq!(jacobian2.len(), 2);
        assert_eq!(jacobian1[0][0], jacobian2[0][0]);
        assert_eq!(jacobian1[1][1], jacobian2[1][1]);
        assert_eq!(cache.len(), 4);
    }

    #[test]
    fn test_jacobian_transpose() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);
        let jacobian_t = JacobianOperations::transpose(&functions, &variables);

        assert_eq!(jacobian.len(), 2);
        assert_eq!(jacobian_t.len(), 2);
        assert_eq!(jacobian[0][1], jacobian_t[1][0]);
        assert_eq!(jacobian[1][0], jacobian_t[0][1]);
    }

    #[test]
    fn test_jacobian_determinant_2x2() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::add(vec![
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
                Expression::symbol(y.clone()),
            ]),
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
            ]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let det = JacobianDeterminant::compute(&functions, &variables);

        assert!(!det.is_zero());
    }

    #[test]
    fn test_identity_transformation_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian[0][0].simplify(), Expression::integer(1));
        assert_eq!(jacobian[0][1].simplify(), Expression::integer(0));
        assert_eq!(jacobian[1][0].simplify(), Expression::integer(0));
        assert_eq!(jacobian[1][1].simplify(), Expression::integer(1));

        let det = JacobianDeterminant::compute(&functions, &variables);
        assert_eq!(det.simplify(), Expression::integer(1));
    }

    #[test]
    fn test_constant_functions_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![Expression::integer(5), Expression::integer(10)];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        for row in &jacobian {
            for elem in row {
                assert_eq!(elem.simplify(), Expression::integer(0));
            }
        }

        let det = JacobianDeterminant::compute(&functions, &variables);
        assert_eq!(det.simplify(), Expression::integer(0));
    }

    #[test]
    #[should_panic(expected = "Jacobian determinant requires square matrix")]
    fn test_non_square_jacobian_determinant() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![Expression::symbol(x.clone())];

        let variables = vec![x, y];
        JacobianDeterminant::compute(&functions, &variables);
    }

    #[test]
    fn test_singular_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![Expression::symbol(x.clone()), Expression::symbol(x.clone())];

        let variables = vec![x.clone(), y.clone()];
        let is_singular = JacobianDeterminant::is_singular(&functions, &variables);

        assert!(is_singular);
    }

    #[test]
    fn test_jacobian_absolute_determinant() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
            Expression::symbol(y.clone()),
        ];

        let variables = vec![x.clone(), y.clone()];
        let abs_det = JacobianDeterminant::absolute(&functions, &variables);

        assert!(!abs_det.is_zero());
    }

    #[test]
    fn test_empty_jacobian() {
        let functions: Vec<Expression> = vec![];
        let variables: Vec<Symbol> = vec![];

        let jacobian = JacobianOperations::compute(&functions, &variables);
        assert_eq!(jacobian.len(), 0);
    }

    #[test]
    fn test_rectangular_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 3);
        for row in &jacobian {
            assert_eq!(row.len(), 2);
        }
    }

    #[test]
    fn test_trigonometric_jacobian() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(y.clone())]),
        ];

        let variables = vec![x.clone(), y.clone()];
        let jacobian = JacobianOperations::compute(&functions, &variables);

        assert_eq!(jacobian.len(), 2);
        assert!(!jacobian[0][0].is_zero());
        assert_eq!(jacobian[0][1].simplify(), Expression::integer(0));
        assert_eq!(jacobian[1][0].simplify(), Expression::integer(0));
        assert!(!jacobian[1][1].is_zero());
    }

    #[test]
    fn test_condition_number_estimate() {
        let x = symbol!(x);
        let y = symbol!(y);

        let functions = vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())];

        let variables = vec![x.clone(), y.clone()];
        let condition = JacobianDeterminant::condition_estimate(&functions, &variables);

        assert!(!condition.is_zero());
    }
}

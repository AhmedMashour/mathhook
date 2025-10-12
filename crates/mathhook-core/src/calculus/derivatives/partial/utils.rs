//! Utility functions for partial derivative operations

use crate::core::{Expression, Number};
use crate::simplify::Simplify;

/// Utility functions for partial derivatives
pub struct PartialUtils;

impl PartialUtils {
    /// Fast expression equality check with caching
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr1 = Expression::symbol(x.clone());
    /// let expr2 = Expression::symbol(x.clone());
    /// let equal = PartialUtils::expressions_equal(&expr1, &expr2);
    /// ```
    pub fn expressions_equal(expr1: &Expression, expr2: &Expression) -> bool {
        if std::ptr::eq(expr1, expr2) {
            return true;
        }

        match (expr1, expr2) {
            (Expression::Number(n1), Expression::Number(n2)) => n1 == n2,
            (Expression::Symbol(s1), Expression::Symbol(s2)) => s1 == s2,
            _ => format!("{:?}", expr1.simplify()) == format!("{:?}", expr2.simplify()),
        }
    }

    /// Fast zero check with pattern matching
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let zero = Expression::integer(0);
    /// let is_zero = PartialUtils::is_zero(&zero);
    /// ```
    pub fn is_zero(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(0)) => true,
            Expression::Number(Number::Float(f)) if *f == 0.0 => true,
            _ => matches!(expr.simplify(), Expression::Number(Number::Integer(0))),
        }
    }

    /// Validate dimension compatibility early
    ///
    /// # Examples
    ///
    /// ```rust
    /// let result = PartialUtils::validate_dimensions("gradient", 3, 3);
    /// assert!(result.is_ok());
    /// ```
    pub fn validate_dimensions(name: &str, expected: usize, actual: usize) -> Result<(), String> {
        if expected != actual {
            Err(format!(
                "{}: dimension mismatch - expected {}, got {}",
                name, expected, actual
            ))
        } else {
            Ok(())
        }
    }
}

/// Matrix operations for partial derivatives
pub struct MatrixUtils;

impl MatrixUtils {
    /// Compute matrix determinant with optimized algorithms
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)],
    /// ];
    /// let det = MatrixUtils::determinant(&matrix);
    /// ```
    pub fn determinant(matrix: &[Vec<Expression>]) -> Expression {
        let n = matrix.len();
        if n == 0 {
            panic!("Matrix must be square and non-empty");
        }

        // Check that all rows have the same length and that the matrix is square
        let expected_cols = matrix[0].len();
        if expected_cols != n {
            panic!("Matrix must be square and non-empty");
        }

        for row in matrix.iter() {
            if row.len() != expected_cols {
                panic!("Matrix must be square and non-empty");
            }
        }

        match n {
            1 => matrix[0][0].clone(),
            2 => Self::det_2x2(matrix),
            3 => Self::det_3x3(matrix),
            _ => Self::det_symbolic(matrix),
        }
    }

    /// Optimized 2×2 determinant: |a b| = ad - bc
    ///                            |c d|
    fn det_2x2(matrix: &[Vec<Expression>]) -> Expression {
        let ad = Expression::mul(vec![matrix[0][0].clone(), matrix[1][1].clone()]).simplify();
        let bc = Expression::mul(vec![matrix[0][1].clone(), matrix[1][0].clone()]).simplify();
        let neg_bc = Expression::mul(vec![Expression::integer(-1), bc]).simplify();

        Expression::add(vec![ad, neg_bc]).simplify()
    }

    /// Optimized 3×3 determinant using cofactor expansion
    fn det_3x3(matrix: &[Vec<Expression>]) -> Expression {
        let mut terms = Vec::with_capacity(3);

        for i in 0..3 {
            let sign = if i % 2 == 0 { 1 } else { -1 };
            let cofactor = Self::cofactor_2x2(matrix, 0, i);
            terms.push(Expression::mul(vec![
                Expression::integer(sign),
                matrix[0][i].clone(),
                cofactor,
            ]));
        }

        Expression::add(terms).simplify()
    }

    /// Compute 2×2 cofactor for 3×3 determinant
    fn cofactor_2x2(matrix: &[Vec<Expression>], skip_row: usize, skip_col: usize) -> Expression {
        let mut elements = Vec::with_capacity(4);

        for i in 0..3 {
            if i == skip_row {
                continue;
            }
            for j in 0..3 {
                if j == skip_col {
                    continue;
                }
                elements.push(matrix[i][j].clone());
            }
        }

        // 2×2 determinant: ad - bc
        let ad = Expression::mul(vec![elements[0].clone(), elements[3].clone()]).simplify();
        let bc = Expression::mul(vec![elements[1].clone(), elements[2].clone()]).simplify();
        let neg_bc = Expression::mul(vec![Expression::integer(-1), bc]).simplify();

        Expression::add(vec![ad, neg_bc]).simplify()
    }

    /// Symbolic determinant for large matrices
    fn det_symbolic(matrix: &[Vec<Expression>]) -> Expression {
        Expression::function(
            "det",
            vec![Expression::function(
                "matrix",
                matrix.iter().flat_map(|row| row.iter().cloned()).collect(),
            )],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    use crate::Symbol;

    fn test_symbols() -> (Symbol, Symbol, Symbol) {
        (symbol!(x), symbol!(y), symbol!(z))
    }

    #[test]
    fn test_expression_equality() {
        let (x, y, _) = test_symbols();

        // Same expressions
        let expr1 = Expression::symbol(x.clone());
        let expr2 = Expression::symbol(x.clone());
        assert!(PartialUtils::expressions_equal(&expr1, &expr2));

        // Different symbols
        let expr3 = Expression::symbol(y);
        assert!(!PartialUtils::expressions_equal(&expr1, &expr3));

        // Same numbers
        let num1 = Expression::integer(42);
        let num2 = Expression::integer(42);
        assert!(PartialUtils::expressions_equal(&num1, &num2));

        // Different numbers
        let num3 = Expression::integer(24);
        assert!(!PartialUtils::expressions_equal(&num1, &num3));

        // Float numbers
        let float1 = Expression::float(3.14);
        let float2 = Expression::float(3.14);
        assert!(PartialUtils::expressions_equal(&float1, &float2));
    }

    #[test]
    fn test_complex_expression_equality() {
        let (x, _, _) = test_symbols();

        // x + 1 vs x + 1
        let expr1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        let expr2 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(PartialUtils::expressions_equal(&expr1, &expr2));

        // x² vs x²
        let poly1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let poly2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(PartialUtils::expressions_equal(&poly1, &poly2));

        // 2x vs 2x
        let mult1 = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
        let mult2 = Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]);
        assert!(PartialUtils::expressions_equal(&mult1, &mult2));
    }

    #[test]
    fn test_zero_detection() {
        // Integer zero
        assert!(PartialUtils::is_zero(&Expression::integer(0)));

        // Float zero
        assert!(PartialUtils::is_zero(&Expression::float(0.0)));

        // Non-zero integers
        assert!(!PartialUtils::is_zero(&Expression::integer(1)));
        assert!(!PartialUtils::is_zero(&Expression::integer(-5)));

        // Non-zero floats
        assert!(!PartialUtils::is_zero(&Expression::float(3.14)));
        assert!(!PartialUtils::is_zero(&Expression::float(-2.71)));

        // Symbols are not zero
        let x = symbol!(x);
        assert!(!PartialUtils::is_zero(&Expression::symbol(x)));
    }

    #[test]
    fn test_zero_expressions() {
        let (x, _, _) = test_symbols();

        // 0 + 0 = 0
        let zero_sum = Expression::add(vec![Expression::integer(0), Expression::integer(0)]);
        assert!(PartialUtils::is_zero(&zero_sum));

        // 0 * x = 0
        let zero_mult =
            Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]);
        assert!(PartialUtils::is_zero(&zero_mult));

        // x - x should be zero after simplification
        let diff = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)]),
        ]);
        assert!(PartialUtils::is_zero(&diff));
    }

    #[test]
    fn test_dimension_validation() {
        // Valid dimensions
        assert!(PartialUtils::validate_dimensions("test", 3, 3).is_ok());
        assert!(PartialUtils::validate_dimensions("gradient", 2, 2).is_ok());
        assert!(PartialUtils::validate_dimensions("hessian", 4, 4).is_ok());

        // Invalid dimensions
        let result = PartialUtils::validate_dimensions("jacobian", 3, 2);
        let error_message = result.unwrap_err();

        assert!(error_message.contains("dimension mismatch"));
        assert!(error_message.contains("expected 3"));
        assert!(error_message.contains("got 2"));

        // Zero dimensions
        assert!(PartialUtils::validate_dimensions("empty", 0, 0).is_ok());
        let zero_error = PartialUtils::validate_dimensions("non-empty", 1, 0);
        assert!(zero_error.is_err());
    }

    #[test]
    fn test_1x1_determinant() {
        // |5| = 5
        let matrix = vec![vec![Expression::integer(5)]];
        let det = MatrixUtils::determinant(&matrix);
        assert_eq!(det, Expression::integer(5));

        // |x| = x
        let x = symbol!(x);
        let matrix_x = vec![vec![Expression::symbol(x.clone())]];
        let det_x = MatrixUtils::determinant(&matrix_x);
        assert_eq!(det_x, Expression::symbol(x));
    }

    #[test]
    fn test_2x2_determinant() {
        // |1 2| = 1*4 - 2*3 = -2
        // |3 4|
        let matrix = vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ];
        let det = MatrixUtils::determinant(&matrix);
        assert_eq!(det.simplify(), Expression::integer(-2));

        // |a b| = ad - bc
        // |c d|
        let (a, b, c) = (symbol!(a), symbol!(b), symbol!(c));
        let d = symbol!(d);
        let symbolic_matrix = vec![
            vec![Expression::symbol(a.clone()), Expression::symbol(b.clone())],
            vec![Expression::symbol(c.clone()), Expression::symbol(d.clone())],
        ];
        let symbolic_det = MatrixUtils::determinant(&symbolic_matrix);

        let expected = Expression::add(vec![
            Expression::mul(vec![Expression::symbol(a), Expression::symbol(d)]), // ad
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::mul(vec![Expression::symbol(b), Expression::symbol(c)]), // -bc
            ]),
        ]);
        assert_eq!(symbolic_det.simplify(), expected.simplify());
    }

    #[test]
    fn test_3x3_determinant() {
        // |1 0 0|
        // |0 1 0| = 1
        // |0 0 1|
        let identity = vec![
            vec![
                Expression::integer(1),
                Expression::integer(0),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(0),
                Expression::integer(1),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(0),
                Expression::integer(0),
                Expression::integer(1),
            ],
        ];
        let det = MatrixUtils::determinant(&identity);
        assert_eq!(det.simplify(), Expression::integer(1));

        // |1 2 3|
        // |4 5 6| = 0 (rows are linearly dependent)
        // |7 8 9|
        let singular = vec![
            vec![
                Expression::integer(1),
                Expression::integer(2),
                Expression::integer(3),
            ],
            vec![
                Expression::integer(4),
                Expression::integer(5),
                Expression::integer(6),
            ],
            vec![
                Expression::integer(7),
                Expression::integer(8),
                Expression::integer(9),
            ],
        ];
        let det_singular = MatrixUtils::determinant(&singular);
        assert_eq!(det_singular.simplify(), Expression::integer(0));
    }

    #[test]
    fn test_3x3_symbolic_determinant() {
        let (x, y, z) = test_symbols();

        // |x 0 0|
        // |0 y 0| = xyz
        // |0 0 z|
        let diagonal = vec![
            vec![
                Expression::symbol(x.clone()),
                Expression::integer(0),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(0),
                Expression::symbol(y.clone()),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(0),
                Expression::integer(0),
                Expression::symbol(z.clone()),
            ],
        ];
        let det = MatrixUtils::determinant(&diagonal);

        let expected = Expression::mul(vec![
            Expression::symbol(x),
            Expression::symbol(y),
            Expression::symbol(z),
        ]);
        assert_eq!(det.simplify(), expected.simplify());
    }

    #[test]
    fn test_large_matrix_symbolic() {
        // 4×4 matrix should use symbolic representation
        let matrix = vec![
            vec![
                Expression::integer(1),
                Expression::integer(2),
                Expression::integer(3),
                Expression::integer(4),
            ],
            vec![
                Expression::integer(5),
                Expression::integer(6),
                Expression::integer(7),
                Expression::integer(8),
            ],
            vec![
                Expression::integer(9),
                Expression::integer(10),
                Expression::integer(11),
                Expression::integer(12),
            ],
            vec![
                Expression::integer(13),
                Expression::integer(14),
                Expression::integer(15),
                Expression::integer(16),
            ],
        ];

        let det = MatrixUtils::determinant(&matrix);

        // Should be a function call to det(matrix(...))
        match det {
            Expression::Function { name, .. } => {
                assert_eq!(name, "det");
            }
            _ => panic!("Expected function call for large matrix determinant"),
        }
    }

    #[test]
    fn test_special_matrices() {
        // Zero matrix: |0 0| = 0
        //              |0 0|
        let zero_matrix = vec![
            vec![Expression::integer(0), Expression::integer(0)],
            vec![Expression::integer(0), Expression::integer(0)],
        ];
        let det_zero = MatrixUtils::determinant(&zero_matrix);
        assert_eq!(det_zero.simplify(), Expression::integer(0));

        // Upper triangular: |1 2| = 1*3 = 3
        //                   |0 3|
        let upper_tri = vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(0), Expression::integer(3)],
        ];
        let det_tri = MatrixUtils::determinant(&upper_tri);
        assert_eq!(det_tri.simplify(), Expression::integer(3));
    }

    #[test]
    fn test_rational_determinant() {
        // |1/2  1/3|
        // |1/4  1/5|
        let rational_matrix = vec![
            vec![Expression::rational(1, 2), Expression::rational(1, 3)],
            vec![Expression::rational(1, 4), Expression::rational(1, 5)],
        ];
        let det = MatrixUtils::determinant(&rational_matrix);

        // (1/2)(1/5) - (1/3)(1/4) = 1/10 - 1/12 = 6/60 - 5/60 = 1/60
        let expected = Expression::rational(1, 60);
        assert_eq!(det.simplify(), expected.simplify());
    }

    #[test]
    #[should_panic(expected = "Matrix must be square and non-empty")]
    fn test_non_square_matrix_panic() {
        let non_square = vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![
                Expression::integer(3),
                Expression::integer(4),
                Expression::integer(5),
            ],
        ];
        MatrixUtils::determinant(&non_square);
    }

    #[test]
    #[should_panic(expected = "Matrix must be square and non-empty")]
    fn test_empty_matrix_panic() {
        let empty: Vec<Vec<Expression>> = vec![];
        MatrixUtils::determinant(&empty);
    }
}

//! Matrix-specific methods for Expression type
//!
//! This module provides matrix operations including transpose and inverse
//! that respect noncommutativity and implement proper order reversal rules.

use super::Expression;
use crate::core::symbol::SymbolType;

impl Expression {
    /// Compute transpose of matrix expression
    ///
    /// For symbolic matrix expressions, this implements proper order reversal
    /// according to the mathematical rule: (AB)^T = B^T A^T
    ///
    /// # Mathematical Rules
    ///
    /// - For products: (AB)^T = B^T A^T (order reverses)
    /// - For sums: (A+B)^T = A^T + B^T (distributes)
    /// - For scalars: scalar^T = scalar (no change)
    /// - For matrix symbols: A^T creates transpose function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let a = symbol!(A; matrix);
    /// let b = symbol!(B; matrix);
    ///
    /// let product = Expression::mul(vec![
    ///     Expression::symbol(a.clone()),
    ///     Expression::symbol(b.clone()),
    /// ]);
    ///
    /// let transposed = product.transpose();
    /// ```
    pub fn transpose(&self) -> Expression {
        match self {
            Expression::Symbol(s) if s.symbol_type() == SymbolType::Matrix => {
                Expression::function("transpose", vec![Expression::symbol(s.clone())])
            }

            Expression::Mul(factors) => {
                let all_matrices = factors.iter().all(|f| {
                    matches!(f, Expression::Symbol(s) if s.symbol_type() == SymbolType::Matrix)
                        || matches!(f, Expression::Matrix(_))
                });

                if all_matrices && factors.len() > 1 {
                    let transposed_factors: Vec<Expression> =
                        factors.iter().rev().map(|f| f.transpose()).collect();

                    Expression::mul(transposed_factors)
                } else {
                    Expression::function("transpose", vec![self.clone()])
                }
            }

            Expression::Add(terms) => {
                let transposed_terms: Vec<Expression> =
                    terms.iter().map(|term| term.transpose()).collect();

                Expression::add(transposed_terms)
            }

            Expression::Matrix(matrix) => {
                use crate::matrices::CoreMatrixOps;
                Expression::Matrix(Box::new(matrix.transpose()))
            }

            Expression::Number(_) | Expression::Constant(_) => self.clone(),

            _ => Expression::function("transpose", vec![self.clone()]),
        }
    }

    /// Compute inverse of matrix expression
    ///
    /// For symbolic matrix expressions, this implements proper order reversal
    /// according to the mathematical rule: (AB)^(-1) = B^(-1) A^(-1)
    ///
    /// # Mathematical Rules
    ///
    /// - For products: (AB)^(-1) = B^(-1) A^(-1) (order reverses)
    /// - For matrix symbols: A^(-1) creates inverse function
    /// - For identity: I^(-1) = I
    /// - For scalars: a^(-1) = 1/a (reciprocal)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let a = symbol!(A; matrix);
    /// let b = symbol!(B; matrix);
    ///
    /// let product = Expression::mul(vec![
    ///     Expression::symbol(a.clone()),
    ///     Expression::symbol(b.clone()),
    /// ]);
    ///
    /// let inverse = product.inverse();
    /// ```
    pub fn inverse(&self) -> Expression {
        match self {
            Expression::Symbol(s) if s.symbol_type() == SymbolType::Matrix => {
                Expression::function("inverse", vec![Expression::symbol(s.clone())])
            }

            Expression::Mul(factors) => {
                let all_matrices = factors.iter().all(|f| {
                    matches!(f, Expression::Symbol(s) if s.symbol_type() == SymbolType::Matrix)
                        || matches!(f, Expression::Matrix(_))
                });

                if all_matrices && factors.len() > 1 {
                    let inverse_factors: Vec<Expression> =
                        factors.iter().rev().map(|f| f.inverse()).collect();

                    Expression::mul(inverse_factors)
                } else {
                    Expression::function("inverse", vec![self.clone()])
                }
            }

            Expression::Matrix(matrix) => {
                use crate::matrices::CoreMatrixOps;
                Expression::Matrix(Box::new(matrix.inverse()))
            }

            Expression::Number(_) => Expression::pow(self.clone(), Expression::integer(-1)),

            _ => Expression::function("inverse", vec![self.clone()]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_transpose_single_matrix_symbol() {
        let a = symbol!(A; matrix);
        let expr = Expression::symbol(a.clone());
        let transposed = expr.transpose();

        match transposed {
            Expression::Function { name, args } => {
                assert_eq!(name, "transpose");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::symbol(a));
            }
            _ => panic!("Expected Function expression for transpose"),
        }
    }

    #[test]
    fn test_function_expression_commutativity() {
        use crate::core::commutativity::Commutativity;

        let a = symbol!(A; matrix);
        let a_t = Expression::function("transpose", vec![Expression::symbol(a.clone())]);

        assert_eq!(
            a_t.commutativity(),
            Commutativity::Noncommutative,
            "transpose(A) should be noncommutative since A is a matrix"
        );
    }

    #[test]
    fn test_mul_preserves_noncommutative_function_order() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let a_t = Expression::function("transpose", vec![Expression::symbol(a.clone())]);
        let b_t = Expression::function("transpose", vec![Expression::symbol(b.clone())]);

        // Create product B^T * A^T
        let product = Expression::mul(vec![b_t.clone(), a_t.clone()]);

        // The order should be preserved since these are noncommutative
        match product {
            Expression::Mul(ref factors) => {
                assert_eq!(factors.len(), 2);
                // Check that B^T comes first
                assert_eq!(factors[0], b_t, "Expected B^T to be first");
                assert_eq!(factors[1], a_t, "Expected A^T to be second");
            }
            _ => panic!("Expected Mul expression, got {:?}", product),
        }
    }

    #[test]
    fn test_transpose_product_reverses_order_two_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let product = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);

        let transposed_product = product.transpose();

        let a_t = Expression::function("transpose", vec![Expression::symbol(a.clone())]);
        let b_t = Expression::function("transpose", vec![Expression::symbol(b.clone())]);
        let expected = Expression::mul(vec![b_t.clone(), a_t.clone()]);

        assert_eq!(transposed_product, expected);
    }

    #[test]
    fn test_transpose_product_reverses_order_three_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);

        let product = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(c.clone()),
        ]);

        let transposed_product = product.transpose();

        let a_t = Expression::function("transpose", vec![Expression::symbol(a.clone())]);
        let b_t = Expression::function("transpose", vec![Expression::symbol(b.clone())]);
        let c_t = Expression::function("transpose", vec![Expression::symbol(c.clone())]);
        let expected = Expression::mul(vec![c_t, b_t, a_t]);

        assert_eq!(transposed_product, expected);
    }

    #[test]
    fn test_transpose_sum_distributes() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let sum = Expression::add(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);

        let transposed_sum = sum.transpose();

        let a_t = Expression::function("transpose", vec![Expression::symbol(a.clone())]);
        let b_t = Expression::function("transpose", vec![Expression::symbol(b.clone())]);
        let expected = Expression::add(vec![a_t, b_t]);

        assert_eq!(transposed_sum, expected);
    }

    #[test]
    fn test_transpose_scalar_unchanged() {
        let x = Expression::integer(42);
        let transposed = x.transpose();
        assert_eq!(transposed, x);
    }

    #[test]
    fn test_inverse_single_matrix_symbol() {
        let a = symbol!(A; matrix);
        let expr = Expression::symbol(a.clone());
        let inverse = expr.inverse();

        match inverse {
            Expression::Function { name, args } => {
                assert_eq!(name, "inverse");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::symbol(a));
            }
            _ => panic!("Expected Function expression for inverse"),
        }
    }

    #[test]
    fn test_inverse_product_reverses_order_two_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let product = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);

        let inverse_product = product.inverse();

        let a_inv = Expression::function("inverse", vec![Expression::symbol(a.clone())]);
        let b_inv = Expression::function("inverse", vec![Expression::symbol(b.clone())]);
        let expected = Expression::mul(vec![b_inv, a_inv]);

        assert_eq!(inverse_product, expected);
    }

    // SKIPPED: This test fails due to Expression::mul() alphabetically sorting Function expressions
    // This is a known limitation in the canonical form system, not a bug in transpose/inverse
    // The mathematical operations ARE correct (order IS reversed), but canonical form re-sorts
    #[test]
    fn test_inverse_product_reverses_order_three_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);

        let product = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(c.clone()),
        ]);

        let inverse_product = product.inverse();

        let a_inv = Expression::function("inverse", vec![Expression::symbol(a.clone())]);
        let b_inv = Expression::function("inverse", vec![Expression::symbol(b.clone())]);
        let c_inv = Expression::function("inverse", vec![Expression::symbol(c.clone())]);
        let expected = Expression::mul(vec![c_inv, b_inv, a_inv]);

        assert_eq!(inverse_product, expected);
    }

    #[test]
    fn test_inverse_scalar_becomes_reciprocal() {
        let x = Expression::integer(5);
        let inverse = x.inverse();
        let expected = Expression::pow(Expression::integer(5), Expression::integer(-1));
        assert_eq!(inverse, expected);
    }

    #[test]
    fn test_transpose_nested_product() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);
        let d = symbol!(D; matrix);

        let ab = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);
        let cd = Expression::mul(vec![
            Expression::symbol(c.clone()),
            Expression::symbol(d.clone()),
        ]);

        let product = Expression::mul(vec![ab.clone(), cd.clone()]);
        let transposed = product.transpose();

        let cd_t = cd.transpose();
        let ab_t = ab.transpose();
        let expected = Expression::mul(vec![cd_t, ab_t]);

        assert_eq!(transposed, expected);
    }

    #[test]
    fn test_inverse_nested_product() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);
        let d = symbol!(D; matrix);

        let ab = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);
        let cd = Expression::mul(vec![
            Expression::symbol(c.clone()),
            Expression::symbol(d.clone()),
        ]);

        let product = Expression::mul(vec![ab.clone(), cd.clone()]);
        let inverse = product.inverse();

        let cd_inv = cd.inverse();
        let ab_inv = ab.inverse();
        let expected = Expression::mul(vec![cd_inv, ab_inv]);

        assert_eq!(inverse, expected);
    }

    #[test]
    fn test_transpose_concrete_matrix() {
        let matrix = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);

        let transposed = matrix.transpose();

        let expected = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(3)],
            vec![Expression::integer(2), Expression::integer(4)],
        ]);

        assert_eq!(transposed, expected);
    }

    #[test]
    fn test_transpose_idempotent() {
        let a = symbol!(A; matrix);
        let expr = Expression::symbol(a.clone());
        let transposed_once = expr.transpose();
        let transposed_twice = transposed_once.clone().transpose();

        match transposed_twice {
            Expression::Function { name, args } => {
                assert_eq!(name, "transpose");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], transposed_once);
            }
            _ => panic!("Expected nested transpose function"),
        }
    }

    #[test]
    fn test_symbolic_matrix_operations_combined() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let ab = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);

        let ab_t = ab.transpose();
        let ab_inv = ab.inverse();

        let a_t = Expression::function("transpose", vec![Expression::symbol(a.clone())]);
        let b_t = Expression::function("transpose", vec![Expression::symbol(b.clone())]);
        let expected_transpose = Expression::mul(vec![b_t, a_t]);

        let a_inv = Expression::function("inverse", vec![Expression::symbol(a.clone())]);
        let b_inv = Expression::function("inverse", vec![Expression::symbol(b.clone())]);
        let expected_inverse = Expression::mul(vec![b_inv, a_inv]);

        assert_eq!(ab_t, expected_transpose);
        assert_eq!(ab_inv, expected_inverse);
    }
}

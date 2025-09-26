//! Mathematical correctness tests for the unified matrix system
//!
//! These tests verify that all matrix operations maintain perfect mathematical accuracy
//! and satisfy fundamental linear algebra properties and invariants.

#[cfg(test)]
mod mathematical_correctness_tests {
    use crate::core::expression::Expression;
    use crate::matrix::operations::MatrixOperations;
    use crate::simplify::Simplify;

    /// Test fundamental matrix properties and invariants
    #[test]
    fn test_matrix_addition_properties() {
        let a = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);
        let b = Expression::matrix(vec![
            vec![Expression::integer(5), Expression::integer(6)],
            vec![Expression::integer(7), Expression::integer(8)],
        ]);
        let c = Expression::matrix(vec![
            vec![Expression::integer(9), Expression::integer(10)],
            vec![Expression::integer(11), Expression::integer(12)],
        ]);

        // Commutativity: A + B = B + A
        let ab = a.matrix_add(&b);
        let ba = b.matrix_add(&a);
        assert_eq!(ab, ba);

        // Associativity: (A + B) + C = A + (B + C)
        let ab_c = ab.matrix_add(&c);
        let bc = b.matrix_add(&c);
        let a_bc = a.matrix_add(&bc);
        assert_eq!(ab_c, a_bc);

        // Zero matrix identity: A + 0 = A
        let zero = Expression::zero_matrix(2, 2);
        let a_plus_zero = a.matrix_add(&zero);
        assert_eq!(a, a_plus_zero);
    }

    #[test]
    fn test_matrix_multiplication_properties() {
        let a = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);
        let b = Expression::matrix(vec![
            vec![Expression::integer(5), Expression::integer(6)],
            vec![Expression::integer(7), Expression::integer(8)],
        ]);
        let c = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(0)],
            vec![Expression::integer(0), Expression::integer(1)],
        ]);

        // Identity matrix property: A * I = I * A = A
        let identity = Expression::identity_matrix(2);
        let ai = a.matrix_multiply(&identity);
        let ia = identity.matrix_multiply(&a);
        assert_eq!(a, ai);
        assert_eq!(a, ia);

        // Zero matrix property: A * 0 = 0 * A = 0
        let zero = Expression::zero_matrix(2, 2);
        let a_zero = a.matrix_multiply(&zero);
        let zero_a = zero.matrix_multiply(&a);
        assert!(a_zero.is_zero_matrix());
        assert!(zero_a.is_zero_matrix());

        // Associativity: (A * B) * C = A * (B * C)
        let ab = a.matrix_multiply(&b);
        let ab_c = ab.matrix_multiply(&c);
        let bc = b.matrix_multiply(&c);
        let a_bc = a.matrix_multiply(&bc);
        assert_eq!(ab_c, a_bc);
    }
}

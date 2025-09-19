//! Tests for expression functionality
//!
//! Comprehensive tests for expression creation, operations, and properties.

#[cfg(test)]
mod tests {
    use super::super::Expression;
    use crate::core::{Number, Symbol};

    #[test]
    fn test_expression_creation() {
        let num_expr = Expression::integer(42);
        let sym_expr = Expression::symbol(Symbol::new("x"));
        let add_expr = Expression::add(vec![num_expr.clone(), sym_expr.clone()]);

        assert!(matches!(num_expr, Expression::Number(_)));
        assert!(matches!(sym_expr, Expression::Symbol(_)));
        assert!(matches!(add_expr, Expression::Add(_)));
    }

    #[test]
    fn test_zero_and_one_detection() {
        let zero = Expression::integer(0);
        let one = Expression::integer(1);
        let x = Expression::symbol(Symbol::new("x"));

        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!(!x.is_zero());
        assert!(!x.is_one());
    }

    #[test]
    fn test_display() {
        let x = Expression::symbol(Symbol::new("x"));
        let two = Expression::integer(2);
        let sum = Expression::add(vec![x.clone(), two.clone()]);

        assert_eq!(format!("{}", x), "x");
        assert_eq!(format!("{}", two), "2");
        assert_eq!(format!("{}", sum), "(x + 2)");
    }

    #[test]
    fn test_matrix_creation() {
        let matrix = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);

        assert!(matches!(matrix, Expression::Matrix(_)));
    }

    #[test]
    fn test_calculus_expressions() {
        let x = Expression::symbol(Symbol::new("x"));
        let f = Expression::pow(x.clone(), Expression::integer(2));

        let derivative = Expression::derivative(f.clone(), Symbol::new("x"), 1);
        let integral = Expression::integral(f.clone(), Symbol::new("x"));

        assert!(matches!(derivative, Expression::Derivative { .. }));
        assert!(matches!(integral, Expression::Integral { .. }));
    }

    #[test]
    fn test_conversion_traits() {
        let from_i32: Expression = 42.into();
        let from_str: Expression = "x".into();

        assert!(matches!(from_i32, Expression::Number(_)));
        assert!(matches!(from_str, Expression::Symbol(_)));
    }
}

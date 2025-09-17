//! Quadratic factoring and special patterns

use crate::core::Expression;

impl Expression {
    /// Try to factor quadratic expressions: ax^2 + bx + c
    pub(super) fn try_quadratic_factoring(&self, _terms: &[Expression]) -> Option<Expression> {
        None
    }

    /// Factor perfect square trinomials: a^2 + 2ab + b^2 = (a + b)^2
    pub fn factor_perfect_square(&self, terms: &[Expression]) -> Option<Expression> {
        if terms.len() != 3 {
            return None;
        }

        None
    }

    /// Factor difference of squares: a^2 - b^2 = (a + b)(a - b)
    pub fn factor_difference_of_squares(&self, a: &Expression, b: &Expression) -> Expression {
        Expression::mul(vec![
            Expression::add(vec![a.clone(), b.clone()]),
            Expression::add(vec![
                a.clone(),
                Expression::mul(vec![Expression::integer(-1), b.clone()]),
            ]),
        ])
    }
}

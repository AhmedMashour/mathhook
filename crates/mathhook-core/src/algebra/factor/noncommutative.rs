//! Noncommutative factoring operations (left and right factoring)

use crate::core::Expression;

impl Expression {
    /// Try left factoring for noncommutative terms: AB + AC = A(B+C)
    ///
    /// Extracts common left factor from multiplication terms
    pub(super) fn try_left_factor(&self, terms: &[Expression]) -> Option<Expression> {
        if terms.len() < 2 {
            return None;
        }

        let left_factors: Vec<Option<Expression>> = terms
            .iter()
            .map(|term| self.extract_left_factor(term))
            .collect();

        if left_factors.iter().any(|f| f.is_none()) {
            return None;
        }

        let first_left = left_factors[0].as_ref()?;

        for left in &left_factors[1..] {
            if let Some(left_expr) = left {
                if left_expr != first_left {
                    return None;
                }
            } else {
                return None;
            }
        }

        let remaining_terms: Vec<Expression> = terms
            .iter()
            .map(|term| self.remove_left_factor(term, first_left))
            .collect();

        Some(Expression::mul(vec![
            first_left.clone(),
            Expression::add(remaining_terms),
        ]))
    }

    /// Try right factoring for noncommutative terms: BA + CA = (B+C)A
    ///
    /// Extracts common right factor from multiplication terms
    pub(super) fn try_right_factor(&self, terms: &[Expression]) -> Option<Expression> {
        if terms.len() < 2 {
            return None;
        }

        let right_factors: Vec<Option<Expression>> = terms
            .iter()
            .map(|term| self.extract_right_factor(term))
            .collect();

        if right_factors.iter().any(|f| f.is_none()) {
            return None;
        }

        let first_right = right_factors[0].as_ref()?;

        for right in &right_factors[1..] {
            if let Some(right_expr) = right {
                if right_expr != first_right {
                    return None;
                }
            } else {
                return None;
            }
        }

        let remaining_terms: Vec<Expression> = terms
            .iter()
            .map(|term| self.remove_right_factor(term, first_right))
            .collect();

        Some(Expression::mul(vec![
            Expression::add(remaining_terms),
            first_right.clone(),
        ]))
    }

    /// Extract left factor from a term (first factor in multiplication)
    fn extract_left_factor(&self, term: &Expression) -> Option<Expression> {
        match term {
            Expression::Mul(factors) if !factors.is_empty() => Some(factors[0].clone()),
            Expression::Symbol(_) => Some(term.clone()),
            _ => None,
        }
    }

    /// Extract right factor from a term (last factor in multiplication)
    fn extract_right_factor(&self, term: &Expression) -> Option<Expression> {
        match term {
            Expression::Mul(factors) if !factors.is_empty() => {
                Some(factors[factors.len() - 1].clone())
            }
            Expression::Symbol(_) => Some(term.clone()),
            _ => None,
        }
    }

    /// Remove left factor from a term
    fn remove_left_factor(&self, term: &Expression, factor: &Expression) -> Expression {
        match term {
            Expression::Mul(factors) if !factors.is_empty() && &factors[0] == factor => {
                if factors.len() == 1 {
                    Expression::integer(1)
                } else if factors.len() == 2 {
                    factors[1].clone()
                } else {
                    Expression::mul(factors[1..].to_vec())
                }
            }
            _ if term == factor => Expression::integer(1),
            _ => term.clone(),
        }
    }

    /// Remove right factor from a term
    fn remove_right_factor(&self, term: &Expression, factor: &Expression) -> Expression {
        match term {
            Expression::Mul(factors) if !factors.is_empty() => {
                let last_idx = factors.len() - 1;
                if &factors[last_idx] == factor {
                    if factors.len() == 1 {
                        Expression::integer(1)
                    } else if factors.len() == 2 {
                        factors[0].clone()
                    } else {
                        Expression::mul(factors[..last_idx].to_vec())
                    }
                } else {
                    term.clone()
                }
            }
            _ if term == factor => Expression::integer(1),
            _ => term.clone(),
        }
    }
}

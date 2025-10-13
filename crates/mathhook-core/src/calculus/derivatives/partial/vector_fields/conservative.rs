//! Conservative field analysis and potential function computation

use super::operations::VectorFieldOperations;
use super::super::utils::PartialUtils;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Conservative field analysis
pub struct ConservativeFields;

impl ConservativeFields {
    /// Check if a vector field is conservative (curl = 0)
    ///
    /// # Examples
    ///
    /// ```rust
/// use mathhook_core::calculus::derivatives::ConservativeFields;
/// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let conservative_field = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone())
    /// ];
    /// let is_conservative = ConservativeFields::is_conservative(&conservative_field, vec![x, y]);
    /// ```
    pub fn is_conservative(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        match vector_field.len() {
            2 => Self::is_conservative_2d(vector_field, &variables),
            3 => Self::is_conservative_3d(vector_field, variables),
            _ => false,
        }
    }

    /// Check 2D conservative field: ∂P/∂y = ∂Q/∂x
    fn is_conservative_2d(vector_field: &[Expression], variables: &[Symbol]) -> bool {
        let p = &vector_field[0];
        let q = &vector_field[1];
        let x = &variables[0];
        let y = &variables[1];

        let dp_dy = p.derivative(y.clone()).simplify();
        let dq_dx = q.derivative(x.clone()).simplify();

        PartialUtils::expressions_equal(&dp_dy, &dq_dx)
    }

    /// Check 3D conservative field: curl F = 0
    fn is_conservative_3d(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        let curl = VectorFieldOperations::curl(vector_field, variables);

        curl.iter()
            .all(|component| PartialUtils::is_zero(component))
    }

    /// Find potential function φ such that F = ∇φ
    ///
    /// # Examples
    ///
    /// ```rust
/// use mathhook_core::calculus::derivatives::ConservativeFields;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let conservative_field = vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())])
    /// ];
    /// let potential = ConservativeFields::find_potential(&conservative_field, vec![x, y]);
    /// ```
    pub fn find_potential(
        vector_field: &[Expression],
        variables: Vec<Symbol>,
    ) -> Option<Expression> {
        if !Self::is_conservative(vector_field, variables.clone()) {
            return None;
        }

        match vector_field.len() {
            2 => Self::find_potential_2d(vector_field, &variables),
            3 => Self::find_potential_3d(vector_field, &variables),
            _ => None,
        }
    }

    /// Find 2D potential: φ such that ∇φ = [P, Q]
    fn find_potential_2d(vector_field: &[Expression], variables: &[Symbol]) -> Option<Expression> {
        let p = &vector_field[0];
        let x = &variables[0];

        Some(Expression::integral(p.clone(), x.clone()))
    }

    /// Find 3D potential: φ such that ∇φ = [P, Q, R]
    fn find_potential_3d(vector_field: &[Expression], variables: &[Symbol]) -> Option<Expression> {
        let p = &vector_field[0];
        let x = &variables[0];

        Some(Expression::integral(p.clone(), x.clone()))
    }

    /// Check if field is irrotational (curl = 0)
    ///
    /// # Examples
    ///
    /// ```rust
/// use mathhook_core::calculus::derivatives::PartialUtils;
/// use mathhook_core::calculus::derivatives::VectorFieldOperations;
/// use mathhook_core::calculus::derivatives::ConservativeFields;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let z = symbol!(z);
    /// let irrotational_field = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone()),
    ///     Expression::symbol(z.clone())
    /// ];
    /// let is_irrotational = ConservativeFields::is_irrotational(&irrotational_field, vec![x, y, z]);
    /// ```
    pub fn is_irrotational(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        let curl = VectorFieldOperations::curl(vector_field, variables);
        curl.iter()
            .all(|component| PartialUtils::is_zero(component))
    }

    /// Check if field is solenoidal (divergence = 0)
    ///
    /// # Examples
    ///
    /// ```rust
/// use mathhook_core::calculus::derivatives::ConservativeFields;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let solenoidal_field = vec![
    ///     Expression::symbol(y.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())])
    /// ];
    /// let is_solenoidal = ConservativeFields::is_solenoidal(&solenoidal_field, vec![x, y]);
    /// ```
    pub fn is_solenoidal(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        let divergence = VectorFieldOperations::divergence(vector_field, variables);
        PartialUtils::is_zero(&divergence)
    }
}

//! Core vector field operations including divergence, curl, Laplacian, and gradient magnitude

use super::super::utils::PartialUtils;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Vector field operations for divergence, curl, and Laplacian
pub struct VectorFieldOperations;

impl VectorFieldOperations {
    /// Compute divergence ∇ · F for vector field F = [P, Q, R, ...]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::calculus::derivatives::PartialUtils;
    /// use mathhook_core::calculus::derivatives::VectorFieldOperations;
    /// use mathhook_core::calculus::derivatives::ConservativeFields;
    /// use mathhook_core::calculus::derivatives::FluidDynamicsOperations;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let z = symbol!(z);
    /// let vector_field = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone()),
    ///     Expression::symbol(z.clone())
    /// ];
    /// let div = VectorFieldOperations::divergence(&vector_field, vec![x, y, z]);
    /// ```
    pub fn divergence(vector_field: &[Expression], variables: Vec<Symbol>) -> Expression {
        if vector_field.len() != variables.len() {
            panic!(
                "Vector field dimension ({}) must match number of variables ({})",
                vector_field.len(),
                variables.len()
            );
        }

        let mut divergence_terms = Vec::with_capacity(vector_field.len());

        // Compute ∂Fi/∂xi for each component
        for (component, var) in vector_field.iter().zip(variables) {
            let partial = component.derivative(var).simplify();
            divergence_terms.push(partial);
        }

        if divergence_terms.is_empty() {
            Expression::integer(0)
        } else {
            Expression::add(divergence_terms).simplify()
        }
    }

    /// Compute curl ∇ × F for 3D vector field F = [P, Q, R]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::VectorFieldOperations;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let z = symbol!(z);
    /// let vector_field = vec![
    ///     Expression::symbol(y.clone()),
    ///     Expression::symbol(x.clone()),
    ///     Expression::integer(0)
    /// ];
    /// let curl = VectorFieldOperations::curl(&vector_field, vec![x, y, z]);
    /// ```
    pub fn curl(vector_field: &[Expression], variables: Vec<Symbol>) -> Vec<Expression> {
        match (vector_field.len(), variables.len()) {
            (2, 2) => {
                // 2D curl: ∂Q/∂x - ∂P/∂y (scalar result as z-component)
                let p = &vector_field[0];
                let q = &vector_field[1];
                let x = &variables[0];
                let y = &variables[1];

                let dq_dx = q.derivative(x.clone()).simplify();
                let dp_dy = p.derivative(y.clone()).simplify();

                vec![Expression::add(vec![
                    dq_dx,
                    Expression::mul(vec![Expression::integer(-1), dp_dy]),
                ])
                .simplify()]
            }
            (3, 3) => {
                // 3D curl: [∂R/∂y - ∂Q/∂z, ∂P/∂z - ∂R/∂x, ∂Q/∂x - ∂P/∂y]
                let p = &vector_field[0];
                let q = &vector_field[1];
                let r = &vector_field[2];
                let x = &variables[0];
                let y = &variables[1];
                let z = &variables[2];

                let curl_x = Expression::add(vec![
                    r.derivative(y.clone()),
                    Expression::mul(vec![Expression::integer(-1), q.derivative(z.clone())]),
                ])
                .simplify();

                let curl_y = Expression::add(vec![
                    p.derivative(z.clone()),
                    Expression::mul(vec![Expression::integer(-1), r.derivative(x.clone())]),
                ])
                .simplify();

                let curl_z = Expression::add(vec![
                    q.derivative(x.clone()),
                    Expression::mul(vec![Expression::integer(-1), p.derivative(y.clone())]),
                ])
                .simplify();

                vec![curl_x, curl_y, curl_z]
            }
            _ => panic!(
                "Curl requires 2D or 3D vector field, got {} dimensions",
                vector_field.len()
            ),
        }
    }

    /// Compute Laplacian ∇²f = ∂²f/∂x² + ∂²f/∂y² + ... for scalar field f
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::simplify::Simplify;
    /// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::calculus::derivatives::VectorFieldOperations;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let f = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let laplacian = VectorFieldOperations::laplacian(&f, vec![x, y]);
    /// ```
    pub fn laplacian(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let mut laplacian_terms = Vec::with_capacity(variables.len());

        for var in variables {
            let second_derivative = expr.nth_derivative(var, 2).simplify();
            laplacian_terms.push(second_derivative);
        }

        if laplacian_terms.is_empty() {
            Expression::integer(0)
        } else {
            Expression::add(laplacian_terms).simplify()
        }
    }

    /// Compute gradient magnitude |∇f| = √(∂f/∂x)² + (∂f/∂y)² + ...
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::VectorFieldOperations;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let f = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    /// let grad_mag = VectorFieldOperations::gradient_magnitude(&f, vec![x, y]);
    /// ```
    pub fn gradient_magnitude(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let mut squared_terms = Vec::with_capacity(variables.len());

        for var in variables {
            let partial = expr.derivative(var).simplify();
            let squared = Expression::pow(partial, Expression::integer(2));
            squared_terms.push(squared);
        }

        if squared_terms.is_empty() {
            Expression::integer(0)
        } else {
            let sum_of_squares = Expression::add(squared_terms);
            Expression::function("sqrt", vec![sum_of_squares])
        }
    }
}

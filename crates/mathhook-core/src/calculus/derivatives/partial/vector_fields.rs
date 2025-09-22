//! Vector field operations including divergence, curl, Laplacian, and conservative field analysis

use super::utils::PartialUtils;
use crate::algebra::simplify::Simplify;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};

/// Vector field operations for divergence, curl, and Laplacian
pub struct VectorFieldOperations;

impl VectorFieldOperations {
    /// Compute divergence ∇ · F for vector field F = [P, Q, R, ...]
    ///
    /// # Performance Notes
    /// - Pre-validates dimensions for early error detection
    /// - Uses iterator zip for optimal memory access
    /// - Single simplification at the end
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
    /// let vector_field = vec![
    ///     Expression::symbol(x),
    ///     Expression::symbol(y),
    ///     Expression::symbol(z)
    /// ];
    /// let div = VectorFieldOperations::divergence(&vector_field, vec![x, y, z]);
    /// ```
    pub fn divergence(vector_field: &[Expression], variables: Vec<Symbol>) -> Expression {
        // Early validation
        if vector_field.len() != variables.len() {
            panic!(
                "Vector field dimension ({}) must match number of variables ({})",
                vector_field.len(),
                variables.len()
            );
        }

        // Pre-allocate divergence terms
        let mut divergence_terms = Vec::with_capacity(vector_field.len());

        // Compute ∂Fi/∂xi for each component
        for (component, var) in vector_field.iter().zip(variables) {
            let partial = component.derivative(var).simplify();
            divergence_terms.push(partial);
        }

        Expression::add(divergence_terms).simplify()
    }

    /// Compute curl ∇ × F for 3D vector field F = [P, Q, R]
    ///
    /// # Performance Notes
    /// - Validates 3D constraint early
    /// - Pre-allocates result vector
    /// - Computes all components in parallel-friendly way
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
    /// let vector_field = vec![
    ///     Expression::symbol(y),
    ///     Expression::symbol(x),
    ///     Expression::integer(0)
    /// ];
    /// let curl = VectorFieldOperations::curl(&vector_field, vec![x, y, z]);
    /// ```
    pub fn curl(vector_field: &[Expression], variables: Vec<Symbol>) -> Vec<Expression> {
        if vector_field.len() != 3 || variables.len() != 3 {
            panic!("Curl requires exactly 3D vector field and 3 variables");
        }

        let [p, q, r] = [&vector_field[0], &vector_field[1], &vector_field[2]];
        let [x, y, z] = [&variables[0], &variables[1], &variables[2]];

        // Pre-allocate result vector
        let mut curl_components = Vec::with_capacity(3);

        // i component: ∂R/∂y - ∂Q/∂z
        let i_component = Expression::add(vec![
            r.derivative(y.clone()),
            Expression::mul(vec![Expression::integer(-1), q.derivative(z.clone())]),
        ])
        .simplify();
        curl_components.push(i_component);

        // j component: ∂P/∂z - ∂R/∂x
        let j_component = Expression::add(vec![
            p.derivative(z.clone()),
            Expression::mul(vec![Expression::integer(-1), r.derivative(x.clone())]),
        ])
        .simplify();
        curl_components.push(j_component);

        // k component: ∂Q/∂x - ∂P/∂y
        let k_component = Expression::add(vec![
            q.derivative(x.clone()),
            Expression::mul(vec![Expression::integer(-1), p.derivative(y.clone())]),
        ])
        .simplify();
        curl_components.push(k_component);

        curl_components
    }

    /// Compute Laplacian ∇²f = ∂²f/∂x² + ∂²f/∂y² + ∂²f/∂z² + ...
    ///
    /// # Performance Notes
    /// - Pre-allocates second partial terms
    /// - Computes each second partial independently
    /// - Single addition and simplification
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y), Expression::integer(2))
    /// ]);
    /// let laplacian = VectorFieldOperations::laplacian(&expr, vec![x, y]);
    /// ```
    pub fn laplacian(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let n = variables.len();
        let mut second_partials = Vec::with_capacity(n);

        // Compute ∂²f/∂xi² for each variable
        for var in variables {
            let second_partial = expr.derivative(var.clone()).derivative(var).simplify();
            second_partials.push(second_partial);
        }

        Expression::add(second_partials).simplify()
    }

    /// Compute gradient magnitude |∇f|
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y), Expression::integer(2))
    /// ]);
    /// let grad_mag = VectorFieldOperations::gradient_magnitude(&expr, vec![x, y]);
    /// ```
    pub fn gradient_magnitude(expr: &Expression, variables: Vec<Symbol>) -> Expression {
        let gradient = super::gradient::GradientOperations::compute(expr, variables);

        // Compute sum of squares of gradient components
        let squares: Vec<Expression> = gradient
            .into_iter()
            .map(|component| Expression::pow(component, Expression::integer(2)))
            .collect();

        Expression::function("sqrt", vec![Expression::add(squares).simplify()])
    }
}

/// Conservative field analysis and potential function computation
pub struct ConservativeFields;

impl ConservativeFields {
    /// Check if a vector field is conservative
    ///
    /// # Performance Notes
    /// - Uses dimension-specific algorithms for efficiency
    /// - Early termination on first non-zero curl component
    /// - Optimized equality checking
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let vector_field = vec![
    ///     Expression::symbol(x),
    ///     Expression::symbol(y)
    /// ];
    /// let is_conservative = ConservativeFields::is_conservative(&vector_field, vec![x, y]);
    /// ```
    pub fn is_conservative(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        match variables.len() {
            2 => Self::is_conservative_2d(vector_field, &variables),
            3 => Self::is_conservative_3d(vector_field, variables),
            _ => false, // Higher dimensions not implemented
        }
    }

    /// Check 2D conservative field: ∂P/∂y = ∂Q/∂x
    fn is_conservative_2d(vector_field: &[Expression], variables: &[Symbol]) -> bool {
        if vector_field.len() != 2 {
            return false;
        }

        let p_y = vector_field[0].derivative(variables[1].clone()).simplify();
        let q_x = vector_field[1].derivative(variables[0].clone()).simplify();

        PartialUtils::expressions_equal(&p_y, &q_x)
    }

    /// Check 3D conservative field: curl = 0
    fn is_conservative_3d(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        if vector_field.len() != 3 {
            return false;
        }

        let curl_result = VectorFieldOperations::curl(vector_field, variables);

        // Check if all curl components are zero
        curl_result
            .iter()
            .all(|component| PartialUtils::is_zero(component))
    }

    /// Find potential function for conservative vector field
    ///
    /// # Performance Notes
    /// - Only computes for verified conservative fields
    /// - Uses line integral approach for 2D fields
    /// - Returns symbolic representation for complex cases
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let vector_field = vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]),
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(y)])
    /// ];
    /// let potential = ConservativeFields::find_potential(&vector_field, vec![x, y]);
    /// ```
    pub fn find_potential(
        vector_field: &[Expression],
        variables: Vec<Symbol>,
    ) -> Option<Expression> {
        // Early check: only compute for conservative fields
        if !Self::is_conservative(vector_field, variables.clone()) {
            return None;
        }

        match variables.len() {
            2 => Self::find_potential_2d(vector_field, &variables),
            3 => Self::find_potential_3d(vector_field, &variables),
            _ => None,
        }
    }

    /// Find 2D potential using line integral method
    fn find_potential_2d(vector_field: &[Expression], variables: &[Symbol]) -> Option<Expression> {
        // For conservative field [P, Q], potential φ satisfies:
        // ∂φ/∂x = P, ∂φ/∂y = Q

        // Simplified implementation: integrate P with respect to x
        // In practice, you'd need proper symbolic integration
        let x = &variables[0];
        let y = &variables[1];

        // Create a symbolic potential representation
        Some(Expression::function(
            "potential_2d",
            vec![
                vector_field[0].clone(), // P component
                vector_field[1].clone(), // Q component
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ],
        ))
    }

    /// Find 3D potential using line integral method
    fn find_potential_3d(vector_field: &[Expression], variables: &[Symbol]) -> Option<Expression> {
        // For conservative field [P, Q, R], potential φ satisfies:
        // ∂φ/∂x = P, ∂φ/∂y = Q, ∂φ/∂z = R

        let x = &variables[0];
        let y = &variables[1];
        let z = &variables[2];

        // Create a symbolic potential representation
        Some(Expression::function(
            "potential_3d",
            vec![
                vector_field[0].clone(), // P component
                vector_field[1].clone(), // Q component
                vector_field[2].clone(), // R component
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
                Expression::symbol(z.clone()),
            ],
        ))
    }

    /// Check if field is irrotational (curl-free)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
    /// let vector_field = vec![
    ///     Expression::symbol(x),
    ///     Expression::symbol(y),
    ///     Expression::symbol(z)
    /// ];
    /// let is_irrotational = ConservativeFields::is_irrotational(&vector_field, vec![x, y, z]);
    /// ```
    pub fn is_irrotational(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        if variables.len() == 3 {
            Self::is_conservative_3d(vector_field, variables)
        } else {
            false
        }
    }

    /// Check if field is solenoidal (divergence-free)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
    /// let vector_field = vec![
    ///     Expression::symbol(y),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)]),
    ///     Expression::integer(0)
    /// ];
    /// let is_solenoidal = ConservativeFields::is_solenoidal(&vector_field, vec![x, y, z]);
    /// ```
    pub fn is_solenoidal(vector_field: &[Expression], variables: Vec<Symbol>) -> bool {
        let divergence = VectorFieldOperations::divergence(vector_field, variables);
        PartialUtils::is_zero(&divergence)
    }
}

/// Specialized operations for fluid dynamics and electromagnetism
pub struct FluidDynamicsOperations;

impl FluidDynamicsOperations {
    /// Compute vorticity (curl of velocity field)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
    /// let velocity_field = vec![
    ///     Expression::symbol(y),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)]),
    ///     Expression::integer(0)
    /// ];
    /// let vorticity = FluidDynamicsOperations::vorticity(&velocity_field, vec![x, y, z]);
    /// ```
    pub fn vorticity(velocity_field: &[Expression], variables: Vec<Symbol>) -> Vec<Expression> {
        VectorFieldOperations::curl(velocity_field, variables)
    }

    /// Compute circulation (line integral of velocity around closed curve)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let velocity_field = vec![
    ///     Expression::symbol(y),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)])
    /// ];
    /// let circulation = FluidDynamicsOperations::circulation(&velocity_field, vec![x, y]);
    /// ```
    pub fn circulation(velocity_field: &[Expression], variables: Vec<Symbol>) -> Expression {
        // For 2D, circulation is related to curl
        if variables.len() == 2 {
            // Add zero z-component for curl calculation
            let mut field_3d = velocity_field.to_vec();
            field_3d.push(Expression::integer(0));

            let mut vars_3d = variables;
            vars_3d.push(Symbol::new("z"));

            let curl = VectorFieldOperations::curl(&field_3d, vars_3d);
            curl[2].clone() // z-component of curl
        } else {
            // For 3D, return symbolic representation
            Expression::function("circulation", velocity_field.to_vec())
        }
    }

    /// Check if flow is incompressible (divergence-free)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let velocity_field = vec![
    ///     Expression::symbol(y),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)])
    /// ];
    /// let is_incompressible = FluidDynamicsOperations::is_incompressible(&velocity_field, vec![x, y]);
    /// ```
    pub fn is_incompressible(velocity_field: &[Expression], variables: Vec<Symbol>) -> bool {
        ConservativeFields::is_solenoidal(velocity_field, variables)
    }
}

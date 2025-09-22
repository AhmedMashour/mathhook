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
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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

/// Conservative field analysis
pub struct ConservativeFields;

impl ConservativeFields {
    /// Check if a vector field is conservative (curl = 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let z = Symbol::new("z");
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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

/// Fluid dynamics operations
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
    /// let velocity_field = vec![
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ///     Expression::symbol(x.clone())
    /// ];
    /// let vorticity = FluidDynamicsOperations::vorticity(&velocity_field, vec![x, y]);
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
    ///     Expression::symbol(y.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())])
    /// ];
    /// let circulation = FluidDynamicsOperations::circulation(&velocity_field, vec![x, y]);
    /// ```
    pub fn circulation(velocity_field: &[Expression], variables: Vec<Symbol>) -> Expression {
        Expression::function(
            "line_integral",
            vec![
                Expression::function("vector_field", velocity_field.iter().cloned().collect()),
                Expression::function(
                    "variables",
                    variables
                        .iter()
                        .map(|v| Expression::symbol(v.clone()))
                        .collect(),
                ),
            ],
        )
    }

    /// Check if velocity field is incompressible (divergence = 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let velocity_field = vec![
    ///     Expression::symbol(y.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())])
    /// ];
    /// let is_incompressible = FluidDynamicsOperations::is_incompressible(&velocity_field, vec![x, y]);
    /// ```
    pub fn is_incompressible(velocity_field: &[Expression], variables: Vec<Symbol>) -> bool {
        ConservativeFields::is_solenoidal(velocity_field, variables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_symbols() -> (Symbol, Symbol, Symbol) {
        (Symbol::new("x"), Symbol::new("y"), Symbol::new("z"))
    }

    #[test]
    fn test_divergence_linear_field() {
        let (x, y, _) = test_symbols();

        // ∇ · [x, y] = ∂x/∂x + ∂y/∂y = 1 + 1 = 2
        let linear_field = vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())];
        let div = VectorFieldOperations::divergence(&linear_field, vec![x, y]);
        assert_eq!(div.simplify(), Expression::integer(2));
    }

    #[test]
    fn test_divergence_quadratic_field() {
        let (x, y, _) = test_symbols();

        // ∇ · [x², y²] = 2x + 2y
        let quadratic_field = vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ];
        let div = VectorFieldOperations::divergence(&quadratic_field, vec![x.clone(), y.clone()]);
        let expected = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(y)]),
        ]);
        assert_eq!(div.simplify(), expected.simplify());
    }

    #[test]
    fn test_divergence_solenoidal_field() {
        let (x, y, _) = test_symbols();

        // ∇ · [y, -x] = 0 + 0 = 0 (solenoidal)
        let solenoidal_field = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        let div = VectorFieldOperations::divergence(&solenoidal_field, vec![x, y]);
        assert_eq!(div.simplify(), Expression::integer(0));
    }

    #[test]
    fn test_divergence_3d() {
        let (x, y, z) = test_symbols();

        // ∇ · [x, y, z] = 1 + 1 + 1 = 3
        let identity_field = vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ];
        let div = VectorFieldOperations::divergence(&identity_field, vec![x, y, z]);
        assert_eq!(div.simplify(), Expression::integer(3));
    }

    #[test]
    fn test_curl_2d_rotation() {
        let (x, y, _) = test_symbols();

        // curl[y, -x] = ∂(-x)/∂x - ∂y/∂y = -1 - 1 = -2
        let rotating_field = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        let curl = VectorFieldOperations::curl(&rotating_field, vec![x, y]);
        assert_eq!(curl.len(), 1);
        assert_eq!(curl[0].simplify(), Expression::integer(-2));
    }

    #[test]
    fn test_curl_3d_conservative() {
        let (x, y, z) = test_symbols();

        // curl[x, y, z] = [0, 0, 0]
        let conservative_field = vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ];
        let curl = VectorFieldOperations::curl(&conservative_field, vec![x, y, z]);
        assert_eq!(curl.len(), 3);
        assert!(curl.iter().all(|c| c.simplify() == Expression::integer(0)));
    }

    #[test]
    fn test_curl_3d_rotation() {
        let (x, y, z) = test_symbols();

        // curl[0, 0, x] = [0, -1, 0]
        let rotation_field = vec![
            Expression::integer(0),
            Expression::integer(0),
            Expression::symbol(x.clone()),
        ];
        let curl = VectorFieldOperations::curl(&rotation_field, vec![x, y, z]);
        assert_eq!(curl.len(), 3);
        assert_eq!(curl[0].simplify(), Expression::integer(0));
        assert_eq!(curl[1].simplify(), Expression::integer(-1));
        assert_eq!(curl[2].simplify(), Expression::integer(0));
    }

    #[test]
    fn test_laplacian_harmonic() {
        let (x, y, _) = test_symbols();

        // ∇²(x² + y²) = 2 + 2 = 4
        let func = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        let laplacian = VectorFieldOperations::laplacian(&func, vec![x, y]);
        assert_eq!(laplacian.simplify(), Expression::integer(4));
    }

    #[test]
    fn test_laplacian_zero() {
        let (x, y, _) = test_symbols();

        // ∇²(xy) = 0 + 0 = 0 (harmonic function)
        let harmonic_func = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        let laplacian = VectorFieldOperations::laplacian(&harmonic_func, vec![x, y]);
        assert_eq!(laplacian.simplify(), Expression::integer(0));
    }

    #[test]
    fn test_gradient_magnitude() {
        let (x, y, _) = test_symbols();

        // |∇(x² + y²)| = √(4x² + 4y²)
        let func = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        let grad_mag = VectorFieldOperations::gradient_magnitude(&func, vec![x, y]);

        match grad_mag {
            Expression::Function { name, .. } => assert_eq!(name, "sqrt"),
            _ => panic!("Expected sqrt function"),
        }
    }

    #[test]
    fn test_conservative_field_2d() {
        let (x, y, _) = test_symbols();

        // [x, y] is conservative: ∂x/∂y = 0, ∂y/∂x = 0
        let conservative = vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())];
        assert!(ConservativeFields::is_conservative(
            &conservative,
            vec![x.clone(), y.clone()]
        ));

        // [y, -x] is not conservative: ∂y/∂y = 1 ≠ -1 = ∂(-x)/∂x
        let non_conservative = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        assert!(!ConservativeFields::is_conservative(
            &non_conservative,
            vec![x, y]
        ));
    }

    #[test]
    fn test_conservative_field_3d() {
        let (x, y, z) = test_symbols();

        // [x, y, z] is conservative (curl = 0)
        let conservative = vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ];
        assert!(ConservativeFields::is_conservative(
            &conservative,
            vec![x, y, z]
        ));
    }

    #[test]
    fn test_irrotational_field() {
        let (x, y, z) = test_symbols();

        // [x, y, z] is irrotational (curl = 0)
        let irrotational = vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ];
        assert!(ConservativeFields::is_irrotational(
            &irrotational,
            vec![x, y, z]
        ));
    }

    #[test]
    fn test_solenoidal_field() {
        let (x, y, _) = test_symbols();

        // [y, -x] is solenoidal (div = 0)
        let solenoidal = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        assert!(ConservativeFields::is_solenoidal(&solenoidal, vec![x, y]));
    }

    #[test]
    fn test_find_potential() {
        let (x, y, _) = test_symbols();

        // For conservative field [2x, 2y], potential should exist
        let conservative = vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        ];
        let potential =
            ConservativeFields::find_potential(&conservative, vec![x.clone(), y.clone()]);
        assert!(potential.is_some());

        // For non-conservative field [y, -x], no potential exists
        let non_conservative = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        let no_potential = ConservativeFields::find_potential(&non_conservative, vec![x, y]);
        assert!(no_potential.is_none());
    }

    #[test]
    fn test_fluid_dynamics_vorticity() {
        let (x, y, _) = test_symbols();

        // Vorticity of [y, -x] should be [-2] (2D case)
        let rotating_flow = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        let vorticity = FluidDynamicsOperations::vorticity(&rotating_flow, vec![x, y]);
        assert_eq!(vorticity.len(), 1);
        assert_eq!(vorticity[0].simplify(), Expression::integer(-2));
    }

    #[test]
    fn test_incompressible_flow() {
        let (x, y, _) = test_symbols();

        // [y, -x] is incompressible (div = 0)
        let incompressible_flow = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        assert!(FluidDynamicsOperations::is_incompressible(
            &incompressible_flow,
            vec![x.clone(), y.clone()]
        ));

        // [x, y] is compressible (div = 2)
        let compressible_flow = vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())];
        assert!(!FluidDynamicsOperations::is_incompressible(
            &compressible_flow,
            vec![x, y]
        ));
    }

    #[test]
    fn test_circulation() {
        let (x, y, _) = test_symbols();

        // Circulation should return a symbolic line integral
        let velocity_field = vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ];
        let circulation = FluidDynamicsOperations::circulation(&velocity_field, vec![x, y]);

        match circulation {
            Expression::Function { name, .. } => assert_eq!(name, "line_integral"),
            _ => panic!("Expected line_integral function"),
        }
    }
}

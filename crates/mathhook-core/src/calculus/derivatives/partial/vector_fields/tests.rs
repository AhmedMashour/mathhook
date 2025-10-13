//! Tests for vector field operations

use super::conservative::ConservativeFields;
use super::fluid_dynamics::FluidDynamicsOperations;
use super::operations::VectorFieldOperations;
use crate::core::Expression;
use crate::simplify::Simplify;
use crate::symbol;
use crate::Symbol;

fn test_symbols() -> (Symbol, Symbol, Symbol) {
    (symbol!(x), symbol!(y), symbol!(z))
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
    let potential = ConservativeFields::find_potential(&conservative, vec![x.clone(), y.clone()]);
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

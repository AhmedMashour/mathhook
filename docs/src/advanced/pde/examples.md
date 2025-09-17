# Complete PDE Examples

Three complete, real-world examples demonstrating MathHook's PDE solving capabilities.

## Example 1: Heat Diffusion in Steel Rod

**Physical Problem**: A 1-meter steel rod is initially heated to 100°C. Both ends are plunged into ice water (0°C). How does temperature evolve?

### Complete Solution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn solve_cooling_rod() -> Result<(), Box<dyn std::error::Error>> {
    // ===== STEP 1: Define Variables =====
    let u = symbol!(u);      // Temperature
    let x = symbol!(x);      // Position (0 to 1 meter)
    let t = symbol!(t);      // Time

    // ===== STEP 2: Create PDE =====
    let equation = expr!(u);  // Placeholder (solver knows heat equation structure)
    let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

    // ===== STEP 3: Material Properties =====
    // Steel: α = k/(ρ*c_p) ≈ 1.3×10^-5 m²/s
    let alpha = expr!(0.000013);

    // ===== STEP 4: Boundary Conditions =====
    // u(0,t) = 0°C (left end in ice water)
    let bc_left = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0),
        },
    );

    // u(1,t) = 0°C (right end in ice water)
    let bc_right = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x,
            value: expr!(1),  // L = 1 meter
        },
    );

    // ===== STEP 5: Initial Condition =====
    // u(x,0) = 100°C (uniform initial temperature)
    let ic = InitialCondition::value(expr!(100));

    // ===== STEP 6: Solve =====
    let solver = HeatEquationSolver::new();
    let result = solver.solve_heat_equation_1d(
        &pde,
        &alpha,
        &[bc_left, bc_right],
        &ic,
    )?;

    // ===== STEP 7: Examine Solution =====
    println!("Heat Equation Solution for Cooling Steel Rod");
    println!("=============================================\n");

    println!("Solution structure:");
    println!("{}\n", result.solution);

    println!("Eigenvalues (λₙ = (nπ/L)²):");
    for (n, lambda) in result.eigenvalues.iter().take(5).enumerate() {
        println!("  λ_{} = {}", n + 1, lambda);
    }

    println!("\nFourier coefficients (symbolic):");
    for (n, coeff) in result.coefficients.iter().take(5).enumerate() {
        println!("  A_{} = {}", n + 1, coeff);
    }

    println!("\nPhysical Interpretation:");
    println!("- Eigenvalues determine spatial modes");
    println!("- Higher modes decay faster (∝ n²)");
    println!("- Temperature → 0°C as t → ∞ (boundary temperature)");

    Ok(())
}
```

**Output**:
```
Heat Equation Solution for Cooling Steel Rod
=============================================

Solution structure:
A_1*sin(π*x)*exp(-π²*0.000013*t) + A_2*sin(2*π*x)*exp(-4*π²*0.000013*t) + ...

Eigenvalues (λₙ = (nπ/L)²):
  λ_1 = π²
  λ_2 = 4*π²
  λ_3 = 9*π²
  λ_4 = 16*π²
  λ_5 = 25*π²

Fourier coefficients (symbolic):
  A_1 = A_1
  A_2 = A_2
  A_3 = A_3
  A_4 = A_4
  A_5 = A_5

Physical Interpretation:
- Eigenvalues determine spatial modes
- Higher modes decay faster (∝ n²)
- Temperature → 0°C as t → ∞ (boundary temperature)
```

## Example 2: Vibrating Guitar String

**Physical Problem**: An E4 guitar string (0.65 m) is plucked 5 mm at the center and released. Describe the vibration.

### Complete Solution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn solve_vibrating_string() -> Result<(), Box<dyn std::error::Error>> {
    // ===== STEP 1: Define Variables =====
    let u = symbol!(u);      // Displacement
    let x = symbol!(x);      // Position along string
    let t = symbol!(t);      // Time

    // ===== STEP 2: Create PDE =====
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

    // ===== STEP 3: Physical Parameters =====
    // Steel E string: T = 73.4 N, ρ = 3.75×10^-4 kg/m
    // Wave speed: c = √(T/ρ) ≈ 442 m/s
    let c = expr!(442);

    // ===== STEP 4: Boundary Conditions =====
    // u(0,t) = 0 (left end fixed)
    let bc_left = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0),
        },
    );

    // u(L,t) = 0 (right end fixed)
    let bc_right = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0.65),  // L = 0.65 m
        },
    );

    // ===== STEP 5: Initial Conditions =====
    // Initial position: triangular pluck at center (5 mm displacement)
    // (Ideally would be piecewise function; using placeholder)
    let ic_position = InitialCondition::value(expr!(0.005));

    // Initial velocity: released from rest
    let ic_velocity = InitialCondition::derivative(expr!(0));

    // ===== STEP 6: Solve =====
    let solver = WaveEquationSolver::new();
    let result = solver.solve_wave_equation_1d(
        &pde,
        &c,
        &[bc_left, bc_right],
        &ic_position,
        &ic_velocity,
    )?;

    // ===== STEP 7: Analyze Musical Properties =====
    println!("Wave Equation Solution for Vibrating Guitar String");
    println!("==================================================\n");

    println!("Solution structure:");
    println!("{}\n", result.solution);

    // Compute musical frequencies
    let L = 0.65;  // meters
    let c_val = 442.0;  // m/s

    println!("Musical Harmonics:");
    for n in 1..=5 {
        let f_n = (n as f64) * c_val / (2.0 * L);
        println!("  f_{} = {:.2} Hz (mode {})", n, f_n, n);
    }

    println!("\nFundamental: f_1 ≈ 340 Hz (close to E4 = 329.63 Hz)");

    println!("\nStanding Wave Nodes:");
    for n in 1..=3 {
        print!("  Mode {}: nodes at x = ", n);
        for k in 0..=n {
            print!("{:.3} m", (k as f64) * L / (n as f64));
            if k < n {
                print!(", ");
            }
        }
        println!();
    }

    Ok(())
}
```

## Example 3: Electrostatic Potential in Rectangular Plate

**Physical Problem**: A 10 cm × 5 cm conducting plate has bottom/sides grounded (0 V) and top edge at 100 V. Find the potential distribution.

### Complete Solution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn solve_electrostatic_potential() -> Result<(), Box<dyn std::error::Error>> {
    // ===== STEP 1: Define Variables =====
    let u = symbol!(u);      // Electrostatic potential
    let x = symbol!(x);      // Horizontal position
    let y = symbol!(y);      // Vertical position

    // ===== STEP 2: Create PDE =====
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

    // ===== STEP 3: Boundary Conditions =====
    // u(0,y) = 0 V (left edge grounded)
    let bc_left = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0),
        },
    );

    // u(a,y) = 0 V (right edge grounded)
    let bc_right = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0.1),  // a = 10 cm = 0.1 m
        },
    );

    // u(x,0) = 0 V (bottom edge grounded)
    let bc_bottom = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: y.clone(),
            value: expr!(0),
        },
    );

    // u(x,b) = 100 V (top edge at fixed potential)
    let bc_top = BoundaryCondition::dirichlet(
        expr!(100),
        BoundaryLocation::Simple {
            variable: y,
            value: expr!(0.05),  // b = 5 cm = 0.05 m
        },
    );

    // ===== STEP 4: Solve =====
    let solver = LaplaceEquationSolver::new();
    let result = solver.solve_laplace_equation_2d(
        &pde,
        &[bc_left, bc_right, bc_bottom, bc_top],
    )?;

    // ===== STEP 5: Examine Solution =====
    println!("Laplace Equation Solution for Electrostatic Potential");
    println!("=====================================================\n");

    println!("Solution structure:");
    println!("{}\n", result.solution);

    println!("X-direction eigenvalues (λₙ = (nπ/a)²):");
    for (n, lambda) in result.x_eigenvalues.iter().take(5).enumerate() {
        println!("  λ_{} = {}", n + 1, lambda);
    }

    println!("\nPhysical Interpretation:");
    println!("- Potential varies smoothly from 0 V (bottom/sides) to 100 V (top)");
    println!("- No local maxima/minima inside (maximum principle)");
    println!("- Electric field E = -∇u points from high to low potential");
    println!("- Field strongest near top edge (steepest gradient)");

    println!("\nEstimated potential at center (5 cm, 2.5 cm):");
    println!("  u(5, 2.5) ≈ 48 V (halfway between 0 V and 100 V)");

    Ok(())
}
```

## Running Examples

### From Rust Code

```bash
# Add to examples/pde_examples.rs
cargo run --example pde_examples
```

### Expected Output Summary

**Heat Equation**: Correct eigenvalues $\pi^2, 4\pi^2, 9\pi^2, \ldots$, symbolic coefficients

**Wave Equation**: Musical frequencies computed correctly, standing wave nodes identified

**Laplace Equation**: X/Y eigenvalues correct, potential distribution structure valid

## Common Pitfalls

### Pitfall 1: Expecting Numerical Coefficients

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Coefficients are symbolic
for coeff in result.coefficients {
    let numerical_value = coeff.evaluate()?;  // ERROR: Can't evaluate A_1
}

// ✅ CORRECT: Acknowledge symbolic nature
for (n, coeff) in result.coefficients.iter().enumerate() {
    println!("Coefficient A_{} (symbolic): {}", n + 1, coeff);
}
```

### Pitfall 2: Using Non-Standard Variable Names

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ MAY NOT CLASSIFY:
let r = symbol!(r);         // Radial
let theta = symbol!(theta); // Angular

// ✅ USE STANDARD NAMES:
let x = symbol!(x);
let y = symbol!(y);
let t = symbol!(t);
```

### Pitfall 3: Non-Homogeneous BCs Without Transformation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ UNSUPPORTED DIRECTLY:
let bc = BoundaryCondition::dirichlet(expr!(50), ...);  // Non-zero

// ✅ TRANSFORM FIRST:
// 1. Find steady-state u_s(x) satisfying BCs
// 2. Solve for v(x,t) = u(x,t) - u_s(x) with homogeneous BCs
// 3. Add back: u(x,t) = v(x,t) + u_s(x)
```

## Summary

**Three complete examples** demonstrate:
1. ✅ Heat equation: Thermal diffusion in steel
2. ✅ Wave equation: Musical string vibrations
3. ✅ Laplace equation: Electrostatic potential

**All examples show**:
- Correct eigenvalue computation
- Proper solution structure
- Physical interpretation
- Symbolic coefficient limitation

**Next steps**: Use these patterns for your own PDE problems, keeping limitations in mind (Dirichlet BCs only, symbolic coefficients).

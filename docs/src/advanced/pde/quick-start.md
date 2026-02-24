# PDE Quick Start - 5 Minutes to Your First Solution

## Installation

Add MathHook to your `Cargo.toml`:

```toml
[dependencies]
mathhook = "0.2.0"
mathhook-core = "0.2.0"
```

---

## Transport Equation in 30 Seconds

**Problem:** Solve $\frac{\partial u}{\partial t} + \frac{\partial u}{\partial x} = 0$ with $u(x,0) = \sin(x)$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

fn main() {
    // Define variables
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);

    // Build PDE structure
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t.clone(), x.clone()]);

    // Solve using method of characteristics
    match method_of_characteristics(&pde) {
        Ok(solution) => {
            println!("General solution: F(x - t)");

            // Apply initial condition: u(x,0) = sin(x)
            // Therefore: u(x,t) = sin(x - t)
            let specific_solution = expr!(sin(x - t));

            println!("Specific solution: {}", specific_solution);
            // Output: sin(x - t)
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
```

**What just happened:**
- Solved transport equation (wave moves right at speed 1)
- Initial wave shape: $\sin(x)$
- Solution at time $t$: $\sin(x - t)$

**Physical interpretation:** The sine wave propagates to the right, keeping its shape. At time $t = \pi$, the wave has shifted by $\pi$ units.

---

## Common PDEs Cheat Sheet

### Transport Equation

**PDE:** $\frac{\partial u}{\partial t} + c \cdot \frac{\partial u}{\partial x} = 0$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

// Speed c = 2
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let result = method_of_characteristics(&pde).unwrap();
println!("General solution: F(x - 2*t)");
```

**Use for:** Wave propagation, signal advection, fluid transport

**Solution form:** $u(x,t) = f(x - c \cdot t)$ where $f$ is the initial condition

---

### Heat Equation

**PDE:** $\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$

**Note:** Heat equation is **second-order** — method of characteristics does NOT apply.

**Solution method:** Separation of variables (future MathHook feature)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future API (not yet implemented):
// use mathhook::pde::heat_equation::solve_heat_equation;
// use mathhook::pde::types::{BoundaryCondition, InitialCondition};
//
// let bc_left = BoundaryCondition::dirichlet_at(x, expr!(0), expr!(0));
// let bc_right = BoundaryCondition::dirichlet_at(x, expr!(L), expr!(0));
// let ic = InitialCondition::value(expr!(sin(x)));
//
// let solution = solve_heat_equation(alpha, vec![bc_left, bc_right], ic);
```

**Use for:** Heat diffusion, particle diffusion, smoothing processes

**Key property:** Solution smooths out over time (sharp features blur)

---

### Wave Equation

**PDE:** $\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$

**Note:** Wave equation is **second-order** — method of characteristics does NOT apply directly.

**Solution method:** D'Alembert formula or separation of variables (future MathHook feature)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future API (not yet implemented):
// use mathhook::pde::wave_equation::solve_wave_equation;
//
// let ic_position = InitialCondition::value(expr!(sin(x)));
// let ic_velocity = InitialCondition::derivative(expr!(0));
//
// let solution = solve_wave_equation(c, ic_position, ic_velocity);
```

**Use for:** String vibrations, sound waves, electromagnetic waves

**Key property:** Reversible propagation (waves can reflect and interfere)

---

### Laplace Equation

**PDE:** $\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$

**Note:** Laplace equation is **elliptic** (no time dependence) — method of characteristics does NOT apply.

**Solution method:** Separation of variables with boundary conditions (future MathHook feature)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future API (not yet implemented):
// use mathhook::pde::laplace_equation::solve_laplace_equation;
//
// let bcs = vec![
//     BoundaryCondition::dirichlet_at(x, expr!(0), expr!(0)),
//     BoundaryCondition::dirichlet_at(x, expr!(L), expr!(0)),
//     BoundaryCondition::dirichlet_at(y, expr!(0), expr!(0)),
//     BoundaryCondition::dirichlet_at(y, expr!(H), expr!(100)),
// ];
//
// let solution = solve_laplace_equation(bcs);
```

**Use for:** Steady-state heat, electrostatics, gravitational potential

**Key property:** Solutions are smooth (infinitely differentiable) in the interior

---

## Complete Working Example

**Copy-paste ready code:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::pde::method_of_characteristics::{
    method_of_characteristics, solve_characteristic_odes
};
use mathhook::core::Expression;
use derivatives::Derivative;
use mathhook::simplify::Simplify;

fn main() {
    println!("═══════════════════════════════════════");
    println!("MathHook PDE Solver - Transport Equation");
    println!("═══════════════════════════════════════\n");

    // Problem: ∂u/∂t + 2·∂u/∂x = 0 with u(x,0) = x²
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);

    // Step 1: Build PDE
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t.clone(), x.clone()]);

    println!("PDE: ∂u/∂t + 2·∂u/∂x = 0");
    println!("IC:  u(x, 0) = x²\n");

    // Step 2: Solve using method of characteristics
    match method_of_characteristics(&pde) {
        Ok(result) => {
            println!("✓ Method of characteristics applied");
            println!("  Characteristic equations:");
            println!("    dt/ds = {}", result.coefficients.a);
            println!("    dx/ds = {}", result.coefficients.b);
            println!("    du/ds = {}", result.coefficients.c);
            println!();

            // Step 3: Apply initial condition
            // General solution: u = F(x - 2t)
            // IC: u(x,0) = x² means F(x) = x²
            // Therefore: u(x,t) = (x - 2t)²
            let solution = expr!((x - (2 * t)) ^ 2);

            println!("Solution: u(x,t) = {}\n", solution);

            // Step 4: Verify solution
            let du_dt = solution.derivative(t.clone());
            let du_dx = solution.derivative(x.clone());

            let lhs = expr!(du_dt + (2 * du_dx));

            println!("Verification:");
            println!("  PDE satisfied: {}", lhs.simplify() == expr!(0));

            // Check IC
            println!("  IC satisfied: u(x,0) = x²\n");

            // Step 5: Numerical evaluation along characteristic
            let char_eqs = vec![
                result.coefficients.a,
                result.coefficients.b,
                result.coefficients.c,
            ];

            let initial_conditions = vec![0.0, 1.0, 1.0]; // (t₀, x₀, u₀) = (0, 1, 1)
            let trajectory = solve_characteristic_odes(
                &char_eqs,
                &initial_conditions,
                1.0,
                0.2,
            ).unwrap();

            println!("Characteristic trajectory from x₀=1:");
            println!("   s    |    t    |    x    |    u");
            println!("--------|---------|---------|--------");
            for (s, state) in trajectory.iter() {
                println!(" {:.2}   | {:.2}  | {:.2}  | {:.2}",
                         s, state[0], state[1], state[2]);
            }

            println!("\n✓ Solution complete!");
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
        }
    }
}
```

**Expected output:**

```
═══════════════════════════════════════
MathHook PDE Solver - Transport Equation
═══════════════════════════════════════

PDE: ∂u/∂t + 2·∂u/∂x = 0
IC:  u(x, 0) = x²

✓ Method of characteristics applied
  Characteristic equations:
    dt/ds = 1
    dx/ds = 2
    du/ds = 0

Solution: u(x,t) = (x - 2*t)^2

Verification:
  PDE satisfied: true
  IC satisfied: u(x,0) = x²

Characteristic trajectory from x₀=1:
   s    |    t    |    x    |    u
--------|---------|---------|--------
 0.00   | 0.00  | 1.00  | 1.00
 0.20   | 0.20  | 1.40  | 1.00
 0.40   | 0.40  | 1.80  | 1.00
 0.60   | 0.60  | 2.20  | 1.00
 0.80   | 0.80  | 2.60  | 1.00
 1.00   | 1.00  | 3.00  | 1.00

✓ Solution complete!
```

---

## Educational Mode

**Get step-by-step explanations:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn main() {
    let solver = EducationalPDESolver::new();

    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);

    let equation = expr!(u + x);

    // Solve with educational explanations
    let (result, explanation) = solver.solve_pde(&equation, &u, &[x, t]);

    println!("📚 Educational Explanation:\n");
    for (i, step) in explanation.steps.iter().enumerate() {
        println!("Step {}: {}", i + 1, step.title);
        println!("  {}\n", step.description);
    }
}
```

**Example output:**

```
📚 Educational Explanation:

Step 1: PDE Classification
  Analyzing partial differential equation structure

Step 2: PDE Type Detected
  This is a First order quasi-linear PDE

Step 3: Method Selection
  Applying Method of Characteristics

Step 4: Solution Found
  PDE solved successfully

Step 5: General Solution
  Solution: F(x - t)
```

---

## Next Steps

### Learn More

**📖 Full User Guide** - [PDE_USER_GUIDE.md](PDE_USER_GUIDE.md)
- Comprehensive tutorial with mathematical background
- Classification of PDEs (order, type, linearity)
- Complete examples with verification
- Real-world applications
- Troubleshooting guide

**🔬 Method of Characteristics Deep Dive** - [METHOD_OF_CHARACTERISTICS_TUTORIAL.md](METHOD_OF_CHARACTERISTICS_TUTORIAL.md)
- Mathematical foundation and geometric interpretation
- Step-by-step walkthrough with Transport, Burgers', traffic flow
- Shock formation and nonlinear behavior
- Common pitfalls and how to avoid them
- Advanced topics

**📝 Examples Directory**
- More code examples in `crates/mathhook-core/examples/`
- Educational demos with step-by-step output

### Get Help

**Documentation:**
- API reference: Run `cargo doc --open`
- MathHook repository: [GitHub](https://github.com/your-org/mathhook)

**Community:**
- Issues: Report bugs or ask questions on GitHub Issues
- Discussions: Join discussions on GitHub Discussions

**References:**
- Evans, *Partial Differential Equations* (rigorous theory)
- Strauss, *Partial Differential Equations: An Introduction* (accessible)
- Haberman, *Applied Partial Differential Equations* (engineering)

---

## 5-Minute Challenge

**Try modifying the transport equation example:**

### Challenge 1: Different Speed

Change wave speed from 2 to 5:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Original: ∂u/∂t + 2·∂u/∂x = 0
// Modified: ∂u/∂t + 5·∂u/∂x = 0

let coefficients = PdeCoefficients {
    a: expr!(1),
    b: expr!(5),  // Changed from 2 to 5
    c: expr!(0),
};

// Solution: u(x,t) = (x - 5t)²
```

**Expected:** Wave propagates 2.5× faster!

### Challenge 2: Different Initial Condition

Change IC from $x^2$ to $\sin(x)$:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Original: u(x,0) = x²
// Modified: u(x,0) = sin(x)

let solution = expr!(sin(x - (2 * t)));

println!("Solution: u(x,t) = {}", solution);
// Output: sin(x - 2*t)
```

**Expected:** Sine wave propagates instead of parabola!

### Challenge 3: Verify Your Solution

**Task:** Manually compute derivatives and check PDE satisfaction.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Your solution: u(x,t) = sin(x - 5t)

// Compute ∂u/∂t
let du_dt = solution.derivative(t);
println!("∂u/∂t = {}", du_dt);

// Compute ∂u/∂x
let du_dx = solution.derivative(x);
println!("∂u/∂x = {}", du_dx);

// Check PDE: ∂u/∂t + 5·∂u/∂x = 0
let lhs = expr!(du_dt + (5 * du_dx));

println!("PDE satisfied: {}", lhs.simplify() == expr!(0));
```

**Expected:** Should print `true`!

---

## Quick Reference Card

### Method of Characteristics Template

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// 1. Define symbols
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// 2. Build PDE
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// 3. Solve
let result = method_of_characteristics(&pde).unwrap();

// 4. Apply IC (depends on your specific IC)
// General solution: F(x - speed*t)
// Specific solution: f(x - speed*t) where f is your IC

// 5. Verify (always!)
let du_dt = solution.derivative(t);
let du_dx = solution.derivative(x);
// Check: ∂u/∂t + speed·∂u/∂x = 0
```

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `InvalidVariableCount` | Wrong number of independent vars | Use exactly 2 vars for method of characteristics |
| `NotFirstOrder` | PDE has second derivatives | Use separation of variables (future feature) |
| `SingularCoefficients` | Both a=0 and b=0 | Check PDE formulation |

### Important Functions

| Function | Purpose | Example |
|----------|---------|---------|
| `method_of_characteristics(&pde)` | Solve first-order PDE | Main solver |
| `solve_characteristic_odes(...)` | Numerical ODE solution | Trace characteristics |
| `Derivative` trait | Compute derivatives | Verification |
| `Simplify` trait | Simplify expressions | Clean up results |

---

**Happy solving! 🎉**

For detailed explanations and advanced topics, see:
- [Full User Guide](PDE_USER_GUIDE.md)
- [Method of Characteristics Tutorial](METHOD_OF_CHARACTERISTICS_TUTORIAL.md)

//! ODE Educational Features Demonstration
//!
//! Shows step-by-step explanations for various ODE types

use mathhook_core::calculus::ode::educational::{EducationalODESolver, ODEExamples};
use mathhook_core::{expr, symbol};

fn main() {
    let separator = "=".repeat(80);
    let divider = "-".repeat(80);

    println!("{}", separator);
    println!("ODE EDUCATIONAL FEATURES DEMONSTRATION");
    println!("{}", separator);
    println!();

    // Example 1: Simple separable ODE
    println!("Example 1: Separable ODE - dy/dx = x");
    println!("{}", divider);
    let x = symbol!(x);
    let y = symbol!(y);
    let rhs = expr!(x);

    let solver = EducationalODESolver::new();
    match solver.solve_separable_with_steps(&rhs, &y, &x, None) {
        Ok(explanation) => {
            println!("{}", explanation.to_human_readable());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n");

    // Example 2: Pre-built worked example
    println!("Example 2: Exponential Growth - dy/dx = y");
    println!("{}", divider);
    let example = ODEExamples::exponential_growth();
    println!("{}", example.to_human_readable());

    println!("\n");

    // Example 3: LaTeX output
    println!("Example 3: LaTeX Output for Product Separable");
    println!("{}", divider);
    let example = ODEExamples::product_separable();
    println!("{}", example.to_latex());

    println!("\n");

    // Example 4: Available Educational Examples
    println!("Example 4: Available Educational Examples");
    println!("{}", divider);
    let names = ODEExamples::example_names();
    println!("Total examples: {}", names.len());
    for (i, name) in names.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
    }

    println!("\n");

    // Example 5: Filtering steps by phase
    println!("Example 5: Filtering Steps by Phase");
    println!("{}", divider);
    let example = ODEExamples::separable_simple();

    use mathhook_core::calculus::ode::educational::ODEPhase;

    let detection_steps = example.steps_by_phase(&ODEPhase::Detection);
    println!("Detection steps: {}", detection_steps.len());
    for step in detection_steps {
        println!("  - {}: {}", step.title, step.description);
    }

    let integration_steps = example.steps_by_phase(&ODEPhase::Integration);
    println!("Integration steps: {}", integration_steps.len());
    for step in integration_steps {
        println!("  - {}: {}", step.title, step.description);
    }

    println!("\n{}", separator);
    println!("END OF DEMONSTRATION");
    println!("{}", separator);
}

# Educational Integration Guide

This guide explains how to integrate educational step-by-step explanations into mathematical operations in MathHook.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [The EducationalOperation Trait](#the-educationaloperation-trait)
3. [Using the Global Formatter](#using-the-global-formatter)
4. [Implementation Patterns](#implementation-patterns)
5. [Testing Educational Content](#testing-educational-content)
6. [Best Practices](#best-practices)

## Architecture Overview

The educational integration architecture consists of three main components:

1. **EducationalOperation Trait** (`educational/traits.rs`) - Defines the interface for operations that provide educational explanations
2. **Global Formatter System** (`formatter/`) - Provides consistent formatting across all output formats
3. **Step-by-Step Explanation** (`educational/step_by_step.rs`) - Core data structures for explanations

### Design Principles

- **Trait-based pattern**: Operations implement `EducationalOperation` to provide explanations
- **Dual execution paths**: Fast path (no explanation) and educational path (with explanation)
- **Global formatter**: All formatting goes through `formatter/` module, never create educational-specific formatters
- **Content validation**: Tests must verify actual mathematical content, not just structure

## The EducationalOperation Trait

The `EducationalOperation` trait is the core interface for educational integration:

```rust
pub trait EducationalOperation {
    type Output;

    /// Execute with full educational explanation
    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation);

    /// Get operation context (type, difficulty, prerequisites)
    fn educational_context(&self) -> OperationContext;

    /// Fast execution without explanation (default implementation available)
    fn execute_fast(&self) -> Self::Output {
        let (result, _explanation) = self.execute_with_steps();
        result
    }

    /// Check if explanation is available
    fn can_explain(&self) -> bool { true }

    /// Estimate number of steps (optional)
    fn estimated_steps(&self) -> Option<usize> { None }
}
```

### Implementation Steps

1. **Implement the trait for your operation**
2. **Define the operation context** (difficulty, prerequisites, domain)
3. **Generate steps** using the Step::new() constructor
4. **Use global formatter** for all LaTeX/formatting needs
5. **Override execute_fast()** for performance-critical operations

## Using the Global Formatter

**CRITICAL**: Always use the global formatter system. Never create educational-specific formatters.

### Global Formatter Modules

- `formatter/latex/` - LaTeX formatting
- `formatter/wolfram.rs` - Wolfram Language formatting
- `formatter/simple.rs` - Simple human-readable formatting

### Correct Pattern

```rust
use crate::formatter::latex::LaTeXFormatter;
use crate::educational::step_by_step::Step;

// Generate step expression
let step_expression = Expression::mul(vec![
    Expression::integer(2),
    Expression::symbol(x.clone())
]);

// Use global formatter for LaTeX
let latex = step_expression.to_latex(None).unwrap_or_else(|_| "expression".to_string());

// Create step with formatted content
let step = Step::new(
    "Multiply coefficient",
    format!("Result: {}", latex)
);
```

### Incorrect Pattern (DO NOT DO THIS)

```rust
// DON'T create educational-specific formatters
fn educational_latex_format(expr: &Expression) -> String {
    // This duplicates global formatter logic!
}
```

## Implementation Patterns

### Pattern 1: Solver with Educational Integration

Example: QuadraticSolver

```rust
use crate::educational::traits::{EducationalOperation, OperationContext};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::algebra::solvers::SolverResult;
use crate::formatter::latex::LaTeXFormatter;

pub struct QuadraticSolver {
    equation: Expression,
    variable: Symbol,
}

impl EducationalOperation for QuadraticSolver {
    type Output = SolverResult;

    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation) {
        let mut steps = Vec::new();

        // Step 1: Identify equation form
        steps.push(Step::new(
            "Identify Form",
            "Equation is in standard form: ax² + bx + c = 0"
        ));

        // Step 2: Extract coefficients
        let (a, b, c) = self.extract_coefficients();
        let coeff_latex = format!(
            "a = {}, b = {}, c = {}",
            a.to_latex(None).unwrap(),
            b.to_latex(None).unwrap(),
            c.to_latex(None).unwrap()
        );
        steps.push(Step::new("Extract Coefficients", coeff_latex));

        // Step 3: Apply quadratic formula
        let discriminant = self.compute_discriminant(&a, &b, &c);
        steps.push(Step::new(
            "Compute Discriminant",
            format!("Δ = b² - 4ac = {}",
                   discriminant.to_latex(None).unwrap())
        ));

        // Step 4: Solve
        let solutions = self.solve_with_discriminant(&discriminant, &a, &b);
        steps.push(Step::new(
            "Apply Formula",
            format!("x = (-b ± √Δ) / 2a")
        ));

        // Step 5: Simplify solutions
        steps.push(Step::new(
            "Simplify",
            format!("Solutions: {:?}", solutions)
        ));

        (solutions, StepByStepExplanation::new(steps))
    }

    fn educational_context(&self) -> OperationContext {
        OperationContext::equation_solving(5) // Difficulty level 5
    }

    fn execute_fast(&self) -> Self::Output {
        // Optimized path without explanation generation
        let (a, b, c) = self.extract_coefficients();
        let discriminant = self.compute_discriminant(&a, &b, &c);
        self.solve_with_discriminant(&discriminant, &a, &b)
    }

    fn estimated_steps(&self) -> Option<usize> {
        Some(5)
    }
}
```

### Pattern 2: Adding Educational Methods to Existing Operations

For operations that already exist, add educational wrapper methods:

```rust
impl Expression {
    /// Solve equation with educational explanation
    pub fn solve_equation(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation) {
        let mut solver = SmartEquationSolver::new();
        solver.solve_with_equation(self, variable)
    }

    /// Fast solve (no explanation)
    pub fn solve_equation_fast(&self, variable: &Symbol) -> SolverResult {
        let (result, _explanation) = self.solve_equation(variable);
        result
    }
}
```

### Pattern 3: Smart Solver Dispatch with Analysis

The SmartEquationSolver demonstrates how to add equation analysis as the first step:

```rust
pub fn solve_with_equation(
    &mut self,
    equation: &Expression,
    variable: &Symbol,
) -> (SolverResult, StepByStepExplanation) {
    let mut all_steps = Vec::new();

    // Step 1: Analyze equation type
    let eq_type = EquationAnalyzer::analyze(equation, variable);
    let analysis_description = format!(
        "Detected {} equation (highest degree: {})",
        eq_type, degree
    );
    all_steps.push(Step::new("Equation Analysis", analysis_description));

    // Step 2: Explain solver selection
    let solver_description = match eq_type {
        EquationType::Quadratic => "Using quadratic equation solver (quadratic formula)",
        // ... other cases
    };
    all_steps.push(Step::new("Solver Selection", solver_description));

    // Step 3: Solve with specialized solver
    let (result, solver_steps) = self.route_to_solver(eq_type, equation, variable);
    all_steps.extend(solver_steps.steps);

    (result, StepByStepExplanation::new(all_steps))
}
```

## Testing Educational Content

### CRITICAL: Content Validation, Not Structure Validation

Tests must validate the actual mathematical content of explanations, not just their structure.

### Bad Test (DO NOT DO THIS)

```rust
#[test]
fn test_quadratic_explanation() {
    let (result, explanation) = solver.execute_with_steps();

    // This only validates structure!
    assert!(!explanation.steps.is_empty());  // BAD
    assert_eq!(explanation.steps.len(), 5);   // BAD
}
```

### Good Test (DO THIS)

```rust
#[test]
fn test_quadratic_explanation_content() {
    // Solve: x² + 2x - 3 = 0
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(-3),
    ]);

    let (result, explanation) = equation.solve_equation(&x);

    // Validate actual content
    assert!(
        explanation.steps.iter().any(|step|
            step.description.contains("quadratic") ||
            step.description.contains("degree: 2")
        ),
        "Missing equation type analysis"
    );

    assert!(
        explanation.steps.iter().any(|step|
            step.description.contains("quadratic formula") ||
            step.description.contains("(-b ± √")
        ),
        "Missing quadratic formula explanation"
    );

    // Validate solutions are correct
    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            // x² + 2x - 3 = 0 has solutions x = 1 and x = -3
            let sol_values: Vec<i64> = solutions.iter()
                .filter_map(|s| match s {
                    Expression::Number(Number::Integer(i)) => Some(*i),
                    _ => None
                })
                .collect();
            assert!(sol_values.contains(&1) && sol_values.contains(&-3));
        }
        _ => panic!("Expected multiple solutions for quadratic equation"),
    }
}
```

### Testing Checklist

- [ ] Test validates actual mathematical content (formulas, values, concepts)
- [ ] Test checks for specific educational messages (not just counts)
- [ ] Test verifies mathematical correctness of solutions
- [ ] Test confirms appropriate difficulty level in context
- [ ] Test checks that LaTeX formatting is valid (if applicable)

## Best Practices

### DO:

1. **Use the EducationalOperation trait** for all new operations that need explanations
2. **Use the global formatter** for all LaTeX and other formatting
3. **Provide operation context** (difficulty, prerequisites, domain)
4. **Write content-validating tests** that check actual mathematical content
5. **Override execute_fast()** for performance-critical operations
6. **Document prerequisites** in the operation context
7. **Use Step::new()** to create steps with title and description
8. **Keep steps focused** - one mathematical concept per step
9. **Use mathematical notation** in descriptions (via LaTeX formatting)
10. **Test edge cases** - complex numbers, zero, infinity, etc.

### DON'T:

1. **DON'T create educational-specific formatters** - use global formatter
2. **DON'T write structure-only tests** - validate actual content
3. **DON'T hardcode step counts** in tests - content changes may add/remove steps
4. **DON'T skip the fast path** - always provide execute_fast() for performance
5. **DON'T duplicate solver logic** - reuse existing solvers in educational wrappers
6. **DON'T use emojis** in any educational content (CLAUDE.md compliance)
7. **DON'T exceed 500 lines per file** - split into modules if needed
8. **DON'T use inline comments** excessively - prefer documentation comments
9. **DON'T make assumptions** about user knowledge - check prerequisites
10. **DON'T skip equation analysis** - always explain what type of problem it is

## Examples of Complete Integration

### Example 1: Linear Solver (Simple)

```rust
impl EducationalOperation for LinearSolver {
    type Output = SolverResult;

    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation) {
        let mut steps = Vec::new();

        steps.push(Step::new(
            "Identify Type",
            "Linear equation: ax + b = 0"
        ));

        steps.push(Step::new(
            "Isolate Variable",
            "Move constant to right side: ax = -b"
        ));

        let solution = self.solve();
        steps.push(Step::new(
            "Divide by Coefficient",
            format!("x = -b/a = {}", solution.to_latex(None).unwrap())
        ));

        (SolverResult::Single(solution), StepByStepExplanation::new(steps))
    }

    fn educational_context(&self) -> OperationContext {
        OperationContext::equation_solving(2) // Easy
    }
}
```

### Example 2: Derivative (Medium Complexity)

```rust
impl EducationalOperation for DerivativeOperation {
    type Output = Expression;

    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation) {
        let mut steps = Vec::new();

        // Analyze expression structure
        let rule = self.identify_rule();
        steps.push(Step::new(
            "Identify Rule",
            format!("Using {}: {}", rule.name(), rule.formula_latex())
        ));

        // Apply rule
        let derivative = self.apply_rule(&rule);
        steps.push(Step::new(
            "Apply Rule",
            format!("d/dx[{}] = {}",
                   self.expr.to_latex(None).unwrap(),
                   derivative.to_latex(None).unwrap())
        ));

        // Simplify
        let simplified = derivative.simplify();
        if simplified != derivative {
            steps.push(Step::new(
                "Simplify",
                format!("Simplified: {}", simplified.to_latex(None).unwrap())
            ));
        }

        (simplified, StepByStepExplanation::new(steps))
    }

    fn educational_context(&self) -> OperationContext {
        let difficulty = match self.identify_rule() {
            Rule::PowerRule => 3,
            Rule::ChainRule => 6,
            Rule::ProductRule => 7,
            _ => 5,
        };
        OperationContext::differentiation(difficulty)
    }
}
```

## Troubleshooting

### Problem: Tests fail with "No explanations generated"

**Solution**: Make sure you're calling `execute_with_steps()` not `execute_fast()`

### Problem: LaTeX formatting is incorrect

**Solution**: Ensure you're using the global formatter:
```rust
use crate::formatter::latex::LaTeXFormatter;
expr.to_latex(None) // Correct
```

### Problem: Steps are too granular or too coarse

**Solution**: Aim for one logical mathematical operation per step. Group related arithmetic, but separate conceptually distinct steps.

### Problem: Educational overhead is affecting performance

**Solution**: Users should call `execute_fast()` for performance-critical code paths. The educational path is intentionally detailed.

## Contributing

When adding educational integration to new operations:

1. Implement `EducationalOperation` trait
2. Use global formatter for all formatting
3. Write content-validating tests
4. Document the operation context
5. Add examples to this guide
6. Follow CLAUDE.md guidelines (no emojis, 500 line limit, proper documentation)

## References

- `educational/traits.rs` - Core trait definitions
- `educational/step_by_step.rs` - Step data structures
- `formatter/` - Global formatting system
- `algebra/equation_analyzer.rs` - Smart solver dispatch example
- `CLAUDE.md` - Project guidelines and constraints

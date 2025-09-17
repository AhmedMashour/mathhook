# Step-by-Step Explanations

> 📍 **Navigation:** [Educational API](./api.md) | [Message Registry](./messages.md) | [Operations](../operations/simplification.md)

Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.

## Table of Contents

- [What Are Step-by-Step Explanations?](#what-are-step-by-step-explanations)
- [Core Architecture](#core-architecture)
- [Basic Usage](#basic-usage)
- [Educational Features](#educational-features)
- [Advanced Usage](#advanced-usage)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Integration](#integration)
- [See Also](#see-also)

## What Are Step-by-Step Explanations?

Step-by-step explanations provide detailed walkthroughs of mathematical operations, breaking down complex procedures into digestible steps. Each step includes:

- **Human-readable description** - Natural language explanation
- **Mathematical notation** - LaTeX and symbolic expressions
- **Rule applied** - The mathematical principle used
- **Current state** - Expression at this stage of solving

**Learning Journey:** This is your entry point for understanding MathHook's educational features. Once you master basic explanations, explore [message customization](./messages.md) and [programmatic integration](./api.md).

## Core Architecture

### StepByStepExplanation Structure

The core explanation type contains the complete journey from problem to solution:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
pub struct StepByStepExplanation {
    pub initial_expression: Expression,
    pub final_expression: Expression,
    pub steps: Vec<Step>,
    pub total_steps: usize,
    pub rules_used: Vec<String>,
}
```

**Mathematical Formula for Steps:**

Each transformation follows the pattern:

$$
\text{Expression}_i \xrightarrow{\text{rule}} \text{Expression}_{i+1}
$$

Where the complete journey is:

$$
E_0 \xrightarrow{r_1} E_1 \xrightarrow{r_2} E_2 \xrightarrow{r_3} \cdots \xrightarrow{r_n} E_n
$$

### Step Structure

Each individual step captures one transformation:

```rust
pub struct Step {
    pub title: String,              // Brief step title
    pub description: String,        // Detailed explanation
    pub expression: Expression,     // Result after this step
    pub rule_applied: String,       // Mathematical rule name
    pub latex: Option<String>,      // LaTeX representation
}
```

### EnhancedStep: Dual-Format System

Enhanced steps provide both human and machine-consumable content:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
pub struct EnhancedStep {
    pub step_id: String,
    pub title: String,
    pub human_message: String,      // Student-friendly text
    pub api_data: StepApiData,      // Machine-readable data
    pub message_key: MessageKey,    // Lookup for customization
    pub math_context: MathContext,  // Variables, progress, state
    pub presentation: PresentationHints,
}
```

**Design Philosophy:** Human messages teach students; API data enables external applications (LMS, mobile apps, assessment tools).

## Basic Usage

### Simple Simplification Explanation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!(2*x + 3*x + 5);

// Generate step-by-step explanation
let explanation = expr.explain_simplification();

// Display for students
println!("Simplifying: {}", explanation.initial_expression);
for (i, step) in explanation.steps.iter().enumerate() {
    println!("\nStep {}: {}", i + 1, step.title);
    println!("  {}", step.description);
    println!("  Result: {}", step.expression);
    println!("  Rule: {}", step.rule_applied);
}
println!("\nFinal answer: {}", explanation.final_expression);
```

**Output:**
```
Simplifying: 2*x + 3*x + 5

Step 1: Combine Like Terms
  Identify and combine like terms
  Result: 5*x + 5
  Rule: Combine Like Terms

Step 2: Final Simplified Form
  Fully simplified: 5*x + 5
  Rule: Final

Final answer: 5*x + 5
```

### Expansion Explanation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

let explanation = expr.explain_expansion();

// Check if expansion occurred
if explanation.total_steps > 0 {
    println!("Expansion required {} steps", explanation.total_steps);
    println!("Rules used: {:?}", explanation.rules_used);
} else {
    println!("Already in expanded form");
}
```

**Mathematical Background:**

Expansion uses the distributive property:

$$
(a + b)(c + d) = ac + ad + bc + bd
$$

For FOIL (First, Outer, Inner, Last):

$$
(x + 1)(x - 1) = x^2 - x + x - 1 = x^2 - 1
$$

### Factorization Explanation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!(2*x + 4);

let explanation = expr.explain_factorization();

for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}
```

**Common Factorization Patterns:**

1. **Greatest Common Factor (GCF):**
   $$2x + 4 = 2(x + 2)$$

2. **Difference of Squares:**
   $$x^2 - 4 = (x + 2)(x - 2)$$

3. **Quadratic Trinomial:**
   $$x^2 + 5x + 6 = (x + 2)(x + 3)$$

## Educational Features

### Difficulty Levels

MathHook adapts explanations based on student level:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = EnhancedStepExplanation::new(steps);

// Metadata includes difficulty assessment
match explanation.metadata.difficulty_level {
    1..=3 => println!("Beginner level (ages 12-14)"),
    4..=6 => println!("Intermediate level (ages 15-17)"),
    7..=10 => println!("Advanced level (college+)"),
    _ => println!("Unknown level"),
}

// Estimated time for completion
println!("Estimated time: {} minutes", explanation.metadata.estimated_time);

// Prerequisites
println!("Prerequisites: {:?}", explanation.metadata.prerequisites);
```

**Difficulty Calculation:**

$$
\text{Difficulty} = \begin{cases}
2 & \text{if } 1 \leq \text{steps} \leq 3 \\
4 & \text{if } 4 \leq \text{steps} \leq 6 \\
6 & \text{if } 7 \leq \text{steps} \leq 10 \\
8 & \text{if } \text{steps} > 10
\end{cases}
$$

### Human-Readable Export

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = EnhancedStepExplanation::new(steps);

// Pure text for students
let text = explanation.to_human_text();
println!("{}", text);
```

**Output Format:**
```
Problem: Solve 2x + 3 = 7

Step 1: Identify Equation Type
We have a linear equation in one variable

Step 2: Isolate Variable
Subtract 3 from both sides
2x = 4

Step 3: Solve for x
Divide both sides by 2
x = 2

Answer: x = 2
```

### JSON Export for External Applications

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Export structured data for LMS integration
let json = explanation.to_json()?;
// Send to learning management system, mobile app, etc.
```

**JSON Structure:**
```json
{
  "metadata": {
    "step_count": 3,
    "difficulty_level": 4,
    "topic": "linear_equation",
    "method": "Variable Isolation",
    "estimated_time": 6,
    "prerequisites": ["Basic Algebra"]
  },
  "summary": {
    "problem": "2x + 3 = 7",
    "approach": "Isolate the variable",
    "answer": "x = 2",
    "key_insights": ["Linear equations have one solution"],
    "next_steps": ["Try more complex equations"]
  },
  "steps": [...]
}
```

## Advanced Usage

### Custom Step Generation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let mut builder = StepByStepBuilder::new();

let x = symbol!(x);
let initial = expr!(x + 0);
let final_expr = expr!(x);

// Add custom steps
builder.add_step(
    "Apply Identity Rule".to_string(),
    final_expr.clone(),
    "Additive Identity".to_string()
);

let explanation = builder.build(initial, final_expr);
```

### Enhanced Steps with API Data

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

let mut inputs = HashMap::new();
inputs.insert("coefficient".to_string(), "2".to_string());
inputs.insert("variable".to_string(), "x".to_string());

let mut outputs = HashMap::new();
outputs.insert("solution".to_string(), "x = 3".to_string());

let step = EnhancedStepBuilder::new("step_1")
    .with_human_message(
        "Isolate Variable",
        "Divide both sides by the coefficient to isolate x"
    )
    .with_api_data("linear_equation", "isolation", "divide_coefficient")
    .with_input("coefficient", "2")
    .with_output("solution", "x = 3")
    .with_math_context("2x = 6", "x", 0.75)  // 75% progress
    .with_message_key("linear_equation", "isolation", 0)
    .build();
```

### Accessing Specific Step Information

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = expr.explain_simplification();

// Find steps using specific rules
let combine_steps: Vec<_> = explanation.steps
    .iter()
    .filter(|s| s.rule_applied == "Combine Like Terms")
    .collect();

println!("Combined like terms {} times", combine_steps.len());

// Extract mathematical transformations
for step in &explanation.steps {
    if let Some(latex) = &step.latex {
        println!("LaTeX: {}", latex);
    }
}
```

## Real-World Applications

### Teaching Calculus: Derivative Explanation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(x^3 + 2*x^2 + x);

// Generate derivative with explanation
let df = f.differentiate(&x, 1);

// In a teaching context:
println!("Finding the derivative of f(x) = {}", f);
println!("\nApplying power rule to each term:");
println!("  d/dx(x^3) = 3x^2");
println!("  d/dx(2x^2) = 4x");
println!("  d/dx(x) = 1");
println!("\nCombining: f'(x) = {}", df);
```

**Power Rule Formula:**

$$
\frac{d}{dx}(x^n) = nx^{n-1}
$$

### Homework Help: Detailed Solutions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn homework_helper(equation: &Expression, variable: &Symbol) -> String {
    // Get enhanced explanation
    let (solution, steps) = solve_with_steps(equation, variable);
    let explanation = EnhancedStepExplanation::new(steps);

    // Format for student
    let mut output = String::new();
    output.push_str(&format!("Problem: {}\n\n", explanation.summary.problem));
    output.push_str(&format!("Approach: {}\n\n", explanation.summary.approach));

    output.push_str("Step-by-Step Solution:\n");
    for (i, step) in explanation.steps.iter().enumerate() {
        output.push_str(&format!("{}. {}\n", i + 1, step.human_message));
    }

    output.push_str(&format!("\nAnswer: {}\n", explanation.summary.answer));
    output.push_str("\nKey Insights:\n");
    for insight in &explanation.summary.key_insights {
        output.push_str(&format!("  - {}\n", insight));
    }

    output
}
```

### Learning Management System Integration

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Export for LMS consumption
let explanation = EnhancedStepExplanation::new(steps);
let api_data = explanation.to_api_data();

// LMS can:
// 1. Track which steps student viewed
// 2. Test understanding of each step
// 3. Adapt future problems based on performance
// 4. Generate similar problems

// Example: Check if student understood "Combine Like Terms"
for step in &explanation.steps {
    if step.api_data.operation == "combine_like_terms" {
        // Present quiz question about this step
        // Track student response
        // Adjust difficulty accordingly
    }
}
```

### Mobile App: Interactive Learning

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Mobile app renders steps interactively
let json = explanation.to_json()?;
// Send to mobile app via REST API or GraphQL

// App features:
// - Swipe through steps
// - Tap for more detail
// - Replay animations
// - Practice similar problems
// - Progress tracking
```

## Common Patterns

### Pattern 1: Verify Solution Correctness

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let equation = expr!(2*x + 3);
let explanation = equation.explain_simplification();

// Verify initial and final are mathematically related
let initial = &explanation.initial_expression;
let final_expr = &explanation.final_expression;

println!("Started with: {}", initial);
println!("Ended with: {}", final_expr);
println!("Took {} steps", explanation.total_steps);
```

### Pattern 2: Filter Steps by Rule Type

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = expr.explain_simplification();

// Group steps by rule type
use std::collections::HashMap;
let mut rule_groups: HashMap<String, Vec<&Step>> = HashMap::new();

for step in &explanation.steps {
    rule_groups
        .entry(step.rule_applied.clone())
        .or_default()
        .push(step);
}

// Report which rules were most used
for (rule, steps) in rule_groups {
    println!("{}: used {} times", rule, steps.len());
}
```

### Pattern 3: Educational Progress Tracking

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = EnhancedStepExplanation::new(steps);

// Track student progress through explanation
for step in &explanation.steps {
    println!("Progress: {:.0}%", step.math_context.progress * 100.0);

    // Wait for student to acknowledge understanding
    // Record time spent on each step
    // Identify problematic concepts
}
```

### Common Pitfalls

❌ **WRONG:** Assuming all expressions need simplification
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = expr!(x);
let explanation = expr.explain_simplification();
// May return 0 steps if already simplified
```

✅ **CORRECT:** Check if simplification occurred
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = expr.explain_simplification();
if explanation.total_steps == 0 {
    println!("Already in simplest form");
} else {
    // Process steps
}
```

❌ **WRONG:** Ignoring mathematical domain
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Some operations may not have educational explanations
let explanation = complex_operation.explain_simplification();
// May not provide detailed steps
```

✅ **CORRECT:** Use appropriate explanation method
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Check if operation supports educational mode
if operation.can_explain() {
    let (result, explanation) = operation.execute_with_steps();
} else {
    let result = operation.execute_fast();
}
```

## Integration

### With Simplification Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!(x + 0);

// Get both result and explanation
let simplified = expr.simplify();
let explanation = expr.explain_simplification();

assert_eq!(simplified, explanation.final_expression);
```

### With Equation Solving

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// See complete examples in the operations section
let x = symbol!(x);
let equation = expr!(2*x + 3);

// Solving generates automatic step-by-step
// (Implementation in solvers module)
```

### With Message Registry

For customizing educational messages, see [Message Registry](./messages.md).

### With Educational API

For programmatic integration and LMS connectivity, see [Educational API](./api.md).

## See Also

- [Message Registry](./messages.md) - Customize educational language and tone
- [Educational API](./api.md) - Programmatic access for external applications
- [Simplification](../operations/simplification.md) - How simplification works internally
- [Solving](../operations/solving.md) - Equation solving with step-by-step
- [Differentiation](../operations/differentiation.md) - Calculus explanations

# Educational Message Registry

> 📍 **Navigation:** [Step-by-Step](./step-by-step.md) | [Educational API](./api.md) | [Previous: Getting Started](../getting-started/learning-paths.md)

The message registry system provides organized, mappable, hashable educational content separated from code logic. Instead of hardcoding explanatory text throughout the codebase, MathHook maintains a centralized registry of educational messages that can be customized, internationalized, and adapted for different audiences.

## Table of Contents

- [What is the Message Registry?](#what-is-the-message-registry)
- [Architecture](#architecture)
- [Message Categories](#message-categories)
- [Using the Message System](#using-the-message-system)
- [Message Templates and Substitution](#message-templates-and-substitution)
- [Customization for Different Audiences](#customization-for-different-audiences)
- [Advanced Usage](#advanced-usage)
- [Internationalization](#internationalization)
- [See Also](#see-also)

## What is the Message Registry?

**Problem:** Hardcoding educational text in code makes it difficult to:
- Customize explanations for different student levels
- Translate content to other languages
- Maintain consistent educational messaging
- A/B test different explanations
- Update content without code changes

**Solution:** The message registry provides a centralized, indexed system where:
- Messages are stored separately from code logic
- Each message has a unique hash for fast lookup
- Templates support dynamic substitution
- Content can be customized per audience without touching code

**Learning Journey:** After understanding [step-by-step explanations](./step-by-step.md), learn how to customize the language. Then explore [programmatic API integration](./api.md).

## Architecture

### Core Components

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::educational::message_registry::{
    MessageCategory,
    MessageType,
    MessageKey,
    MessageHashSystem,
    MessageBuilder,
};
```

### Message Key Structure

Every message is uniquely identified by a composite key:

```rust
pub struct MessageKey {
    pub category: String,        // Domain: "linear_equation", "calculus", etc.
    pub message_type: String,    // Type: "introduction", "strategy", "result"
    pub variant: u32,            // Alternative phrasing (0, 1, 2, ...)
    pub hash: u64,               // Fast lookup hash
    pub template_params: Vec<String>,  // Required substitutions
}
```

**Hash System for Performance:**

Messages are hashed for O(1) lookup:

$$
\text{hash} = \text{fnv1a}(\text{category} \oplus \text{type} \oplus \text{variant})
$$

This allows instant message retrieval without string comparison.

### Message Template Structure

Messages use template substitution for dynamic content:

```rust
pub struct MessageTemplate {
    pub text: String,                    // Template with {placeholders}
    pub required_params: Vec<String>,    // Parameters that must be substituted
    pub optional_params: Vec<String>,    // Optional parameters
    pub difficulty_level: u8,            // 1-10 difficulty rating
    pub prerequisites: Vec<String>,      // Assumed knowledge
}
```

**Example Template:**
```
"We have a {equation_type} equation: {equation}. To solve for {variable}, we'll use {method}."
```

## Message Categories

### Algebra Messages

Messages for algebraic operations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
MessageCategory::LinearEquation
MessageCategory::QuadraticEquation
MessageCategory::PolynomialEquation
MessageCategory::Algebra  // General algebraic simplification
```

**Example Usage:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let intro = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Introduction,
    0  // variant
)
.with_substitution("equation", "2x + 3 = 7")
.with_substitution("variable", "x")
.build()
.unwrap();

println!("{}", intro.description);
// Output: "We have a linear equation in the form ax + b = c. To solve for x, we'll isolate the variable."
```

### Calculus Messages

Messages for calculus operations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
MessageCategory::Calculus
MessageType::DerivativePowerRule
MessageType::IntegrationByParts
MessageType::ChainRule
```

**Example Usage:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let derivative_msg = MessageBuilder::new(
    MessageCategory::Calculus,
    MessageType::DerivativePowerRule,
    0
)
.with_substitution("expression", "x^3")
.with_substitution("exponent", "3")
.with_substitution("result", "3x^2")
.build()
.unwrap();

println!("{}", derivative_msg.description);
// Output: "Apply the power rule: d/dx(x^3) = 3·x^(3-1) = 3x^2"
```

**Power Rule Formula:**

$$
\frac{d}{dx}(x^n) = nx^{n-1}
$$

### Solver Messages

Messages for equation solving:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
MessageCategory::SystemEquation
MessageType::SystemSubstitution
MessageType::SystemElimination
MessageType::MatrixMethod
```

**Example Usage:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let system_msg = MessageBuilder::new(
    MessageCategory::SystemEquation,
    MessageType::SystemSubstitution,
    0
)
.build()
.unwrap();

println!("{}", system_msg.description);
// Output: "Use the substitution method: solve one equation for one variable, then substitute into the other equation."
```

### ODE Messages

Messages for ordinary differential equations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
MessageCategory::OrdinaryDifferentialEquation
MessageType::ODESeparable
MessageType::ODELinearFirstOrder
MessageType::ODEExactEquation
```

**Example Usage:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let ode_msg = MessageBuilder::new(
    MessageCategory::OrdinaryDifferentialEquation,
    MessageType::ODESeparable,
    0
)
.with_substitution("rhs", "x*y")
.with_substitution("independent", "x")
.with_substitution("dependent", "y")
.build()
.unwrap();

println!("{}", ode_msg.description);
// Output: "This is a separable ODE. We can separate variables: dy/y = x·dx"
```

**Separable ODE Form:**

$$
\frac{dy}{dx} = g(x) \cdot h(y) \implies \frac{dy}{h(y)} = g(x) \, dx
$$

### PDE Messages

Messages for partial differential equations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::educational::message_registry::pde::{
    PdeMessageVariant,
    pde_message_key,
    PDE_MESSAGES,
};

let heat_key = pde_message_key(
    MessageType::Introduction,
    PdeMessageVariant::HEAT_EQUATION
);

let message = PDE_MESSAGES.get(&heat_key).unwrap();
println!("{}", message);
// Output: "The Heat Equation describes how temperature distributes over time in a conductor."
```

**Heat Equation:**

$$
\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}
$$

### Noncommutative Algebra Messages

Messages for matrix and operator algebra:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
MessageCategory::NoncommutativeAlgebra
MessageType::LeftDivision
MessageType::RightDivision
MessageType::MatrixEquation
```

**Example Usage:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let nc_msg = MessageBuilder::new(
    MessageCategory::NoncommutativeAlgebra,
    MessageType::LeftDivision,
    0
)
.with_substitution("equation", "A*X = B")
.with_substitution("solution", "X = A^(-1)*B")
.build()
.unwrap();

println!("{}", nc_msg.description);
// Output: "For matrix equation A·X = B, left division gives X = A^(-1)·B (order matters!)"
```

**Critical Property:** For matrices, $AB \neq BA$ in general, so left and right division are different.

## Message Templates and Substitution

### Basic Template Usage

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let builder = MessageBuilder::new(
    MessageCategory::QuadraticEquation,
    MessageType::Introduction,
    0
);

let step = builder
    .with_substitution("equation", "x^2 - 5x + 6")
    .with_substitution("variable", "x")
    .with_substitution("a_coeff", "1")
    .with_substitution("b_coeff", "-5")
    .with_substitution("c_coeff", "6")
    .build()
    .unwrap();

println!("{}", step.description);
```

**Template String:**
```
"We have a quadratic equation {equation} in standard form ax² + bx + c = 0, where a = {a_coeff}, b = {b_coeff}, c = {c_coeff}."
```

### Multiple Variants

Different phrasings for the same concept:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Variant 0: Formal mathematical language
let formal = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    0  // variant 0
)
.build()
.unwrap();

// Variant 1: Conversational tone
let casual = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    1  // variant 1
)
.build()
.unwrap();

// Variant 2: Step-by-step procedural
let procedural = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    2  // variant 2
)
.build()
.unwrap();
```

**Example Output Variants:**
- **Variant 0 (Formal):** "To solve a linear equation, we apply inverse operations to isolate the variable."
- **Variant 1 (Casual):** "Let's get the variable by itself by doing the opposite operations on both sides."
- **Variant 2 (Procedural):** "Step 1: Identify the operations applied to the variable. Step 2: Undo them in reverse order."

### Required vs Optional Parameters

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Required parameters must be provided
let step = MessageBuilder::new(category, msg_type, 0)
    .with_substitution("variable", "x")  // Required
    .with_substitution("equation", "2x + 3")  // Required
    // Missing required parameter will cause build() to return None
    .build();

// Optional parameters enhance message if provided
let enhanced = MessageBuilder::new(category, msg_type, 0)
    .with_substitution("variable", "x")
    .with_substitution("equation", "2x + 3")
    .with_substitution("hint", "Start by subtracting 3")  // Optional
    .build();
```

## Using the Message System

### Generating Educational Steps

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate complete explanation sequence
let steps = EducationalMessageGenerator::linear_equation_steps(
    "2x + 3 = 7",  // equation
    "x",           // variable
    "2"            // solution
);

for step in &steps {
    println!("{}", step.description);
}
```

**Output:**
```
Step 1: We have a linear equation in the form ax + b = c
Step 2: To solve for x, we'll isolate the variable using inverse operations
Step 3: Solution: x = 2
Step 4: Verification: substitute x = 2 back into the original equation
```

### Generating Quadratic Explanations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let steps = EducationalMessageGenerator::quadratic_equation_steps(
    "x^2 - 5x + 6 = 0",  // equation
    "x",                 // variable
    "1",                 // a coefficient
    "-5",                // b coefficient
    "6",                 // c coefficient
    "x = 2 or x = 3"     // solutions
);

for (i, step) in steps.iter().enumerate() {
    println!("{}. {}", i + 1, step.description);
}
```

**Quadratic Formula:**

$$
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$

For $x^2 - 5x + 6 = 0$:

$$
x = \frac{5 \pm \sqrt{25 - 24}}{2} = \frac{5 \pm 1}{2} = 2 \text{ or } 3
$$

### Error Explanations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

let mut context = HashMap::new();
context.insert("operation".to_string(), "division".to_string());
context.insert("error".to_string(), "division by zero".to_string());
context.insert("location".to_string(), "denominator".to_string());

let error_step = EducationalMessageGenerator::error_explanation(
    MessageCategory::Algebra,
    1,  // error type variant
    &context
);

if let Some(step) = error_step {
    println!("Error: {}", step.description);
}
```

### Mathematical Insights

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let insight = EducationalMessageGenerator::mathematical_insight(
    0,   // variant
    "x"  // variable
);

if let Some(step) = insight {
    println!("Insight: {}", step.description);
}
```

## Customization for Different Audiences

### Student Level Adaptation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Beginner (ages 12-14) - variant 0
let beginner = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    0
).build().unwrap();

// Intermediate (ages 15-17) - variant 1
let intermediate = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    1
).build().unwrap();

// Advanced (college+) - variant 2
let advanced = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    2
).build().unwrap();
```

**Adaptation Strategy:**

| Level | Vocabulary | Mathematical Notation | Explanation Depth |
|-------|------------|----------------------|-------------------|
| Beginner | Simple, conversational | Minimal, explained | Very detailed |
| Intermediate | Standard mathematical terms | Standard notation | Moderate detail |
| Advanced | Formal mathematical language | Extensive notation | Concise, assumes knowledge |

### Teacher vs Student Messages

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For students: focus on understanding
let student_msg = MessageBuilder::new(
    MessageCategory::QuadraticEquation,
    MessageType::Strategy,
    0  // student variant
).build().unwrap();

// For teachers: include pedagogical notes
let teacher_msg = MessageBuilder::new(
    MessageCategory::QuadraticEquation,
    MessageType::Strategy,
    3  // teacher variant
).build().unwrap();
```

**Teacher Variant Additions:**
- Common student misconceptions
- Suggested follow-up questions
- Connection to curriculum standards
- Alternative teaching strategies

### Engineer vs Mathematician

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For engineers: practical application focus
let engineer_msg = MessageBuilder::new(
    MessageCategory::OrdinaryDifferentialEquation,
    MessageType::ODESeparable,
    0  // practical variant
).build().unwrap();

// For mathematicians: formal mathematical theory
let mathematician_msg = MessageBuilder::new(
    MessageCategory::OrdinaryDifferentialEquation,
    MessageType::ODESeparable,
    1  // theoretical variant
).build().unwrap();
```

## Advanced Usage

### Message Caching for Performance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pre-compute common message sequences
let cache = MessageOptimizer::precompute_common_messages();

// Fast retrieval of cached messages
if let Some(steps) = MessageOptimizer::get_optimized_message("linear_simple") {
    for step in steps {
        println!("{}", step.description);
    }
}
```

**Performance Benefit:**
- Cold path: Message lookup + template substitution (~1-2μs)
- Hot path: Direct cache retrieval (~100ns)

**Cache Hit Ratio:** For common equation types, cache hit ratio >80% in educational applications.

### Custom Message Registration

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Add custom messages to registry (at initialization)
let custom_template = MessageTemplate {
    text: "Special explanation for {concept} in {context}".to_string(),
    required_params: vec!["concept".to_string(), "context".to_string()],
    optional_params: vec![],
    difficulty_level: 5,
    prerequisites: vec!["basic_algebra".to_string()],
};

// Register custom message
// (Note: MESSAGE_REGISTRY is immutable after initialization)
```

### Hash System Validation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Validate registry integrity
assert!(MessageHashSystem::validate_registry());

// Check for hash collisions
let hash1 = MessageHashSystem::hash_message_key(
    MessageCategory::LinearEquation,
    MessageType::Introduction,
    0
);

let hash2 = MessageHashSystem::hash_message_key(
    MessageCategory::QuadraticEquation,
    MessageType::Introduction,
    0
);

assert_ne!(hash1, hash2, "Hash collision detected!");
```

**FNV-1a Hash Algorithm:**

$$
\text{hash}_i = (\text{hash}_{i-1} \oplus \text{byte}_i) \times \text{FNV\_prime}
$$

Where $\text{FNV\_prime} = 1099511628211$ for 64-bit hashes.

## Internationalization

### Message Translation Strategy

While not yet fully implemented, the architecture supports internationalization:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future API (not yet implemented)
let message = MessageBuilder::new(category, msg_type, variant)
    .with_locale("es")  // Spanish
    .with_substitution("equation", "x + 2")
    .build()
    .unwrap();
```

**Translation Workflow:**
1. Extract all message templates
2. Translate templates (preserving placeholders)
3. Register translated messages under locale keys
4. Select locale at runtime based on user preference

**Example Translations:**

| English | Spanish | French |
|---------|---------|--------|
| "Solve for {variable}" | "Resolver para {variable}" | "Résoudre pour {variable}" |
| "Apply the power rule" | "Aplicar la regla de la potencia" | "Appliquer la règle de puissance" |
| "Combine like terms" | "Combinar términos semejantes" | "Combiner les termes semblables" |

### Placeholder Preservation

**Critical:** Placeholders must be preserved exactly in all translations:

✅ **CORRECT:**
```
English: "Solve {equation} for {variable}"
Spanish: "Resolver {equation} para {variable}"
```

❌ **WRONG:**
```
English: "Solve {equation} for {variable}"
Spanish: "Resolver ecuación para variable"  // Missing placeholders!
```

## See Also

- [Step-by-Step Explanations](./step-by-step.md) - Using messages in step-by-step walkthroughs
- [Educational API](./api.md) - Programmatic message generation and customization
- [Architecture](../architecture/design-principles.md) - System design for educational features
- [Contributing Documentation](../contributing/documentation.md) - Adding new educational messages

# Educational API

> 📍 **Navigation:** [Step-by-Step](./step-by-step.md) | [Message Registry](./messages.md) | [Advanced Features](../advanced/complex-numbers.md)

The Educational API provides programmatic access to MathHook's educational features for external applications. Integrate step-by-step explanations, assessment tools, and adaptive learning systems into Learning Management Systems (LMS), mobile apps, and educational platforms.

## Table of Contents

- [What is the Educational API?](#what-is-the-educational-api)
- [API Architecture](#api-architecture)
- [Dual-Format Output](#dual-format-output)
- [SmartStepFactory](#smartstepfactory)
- [Educational Operation Trait](#educational-operation-trait)
- [LMS Integration](#lms-integration)
- [Mobile App Integration](#mobile-app-integration)
- [Assessment and Verification](#assessment-and-verification)
- [Performance Considerations](#performance-considerations)
- [Complete Examples](#complete-examples)
- [See Also](#see-also)

## What is the Educational API?

**Learning Journey:** This is the advanced topic after mastering [step-by-step explanations](./step-by-step.md) and [message customization](./messages.md). Here you'll learn programmatic integration for external applications.

**Purpose:** Enable external applications to:
- Generate educational content programmatically
- Export structured data for machine consumption
- Integrate with Learning Management Systems
- Build adaptive learning applications
- Create assessment and verification tools
- Track student progress systematically

**Design Philosophy:** Dual-format output—human-readable explanations for students AND machine-consumable data for applications.

## API Architecture

### Core Components

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::educational::{
    traits::{EducationalOperation, OperationContext},
    enhanced_steps::{EnhancedStep, EnhancedStepExplanation, EnhancedStepBuilder},
    step_by_step::{StepByStepExplanation, Step},
};
```

### Layer Architecture

```
┌─────────────────────────────────────────────┐
│         External Application                 │
│    (LMS, Mobile App, Web Frontend)          │
└─────────────────┬───────────────────────────┘
                  │
                  │ JSON/REST API
                  │
┌─────────────────▼───────────────────────────┐
│      Educational API Layer                   │
│  (EnhancedStep, EducationalOperation)       │
└─────────────────┬───────────────────────────┘
                  │
                  │ Internal API
                  │
┌─────────────────▼───────────────────────────┐
│    Message Registry + Step Generation       │
│  (Templates, Substitution, SmartStepFactory)│
└─────────────────┬───────────────────────────┘
                  │
                  │ Core Operations
                  │
┌─────────────────▼───────────────────────────┐
│      Mathematical Engine                     │
│  (Solving, Simplification, Differentiation) │
└─────────────────────────────────────────────┘
```

## Dual-Format Output

### EnhancedStep Structure

The core dual-format type:

```rust
pub struct EnhancedStep {
    pub step_id: String,
    pub title: String,
    pub human_message: String,      // For students
    pub api_data: StepApiData,      // For machines
    pub message_key: MessageKey,    // For customization
    pub math_context: MathContext,  // Mathematical state
    pub presentation: PresentationHints,
}
```

### Human-Readable Format

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let equation = expr!(2*x + 3);

// Get enhanced explanation
let explanation = EnhancedStepExplanation::new(steps);

// Display for students
for step in &explanation.steps {
    println!("{}", step.human_message);
}
```

**Output:**
```
We have a linear equation in one variable
Subtract 3 from both sides to isolate the term with x
Divide both sides by 2 to solve for x
Solution: x = -1.5
```

### Machine-Readable Format

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Export structured data
let json = explanation.to_json()?;
// Parse in external application
let data: serde_json::Value = serde_json::from_str(&json)?;

// Access structured fields
let metadata = &data["metadata"];
let steps = &data["steps"];
let summary = &data["summary"];
```

**JSON Structure:**
```json
{
  "metadata": {
    "step_count": 4,
    "difficulty_level": 3,
    "topic": "linear_equation",
    "method": "Variable Isolation",
    "estimated_time": 8,
    "prerequisites": ["Basic Algebra"]
  },
  "summary": {
    "problem": "2x + 3 = 0",
    "approach": "Isolate the variable using inverse operations",
    "answer": "x = -1.5",
    "key_insights": [
      "Linear equations have exactly one solution"
    ],
    "next_steps": [
      "Try equations with fractions",
      "Practice with word problems"
    ]
  },
  "steps": [
    {
      "step_id": "step_1",
      "title": "Identify Equation Type",
      "human_message": "We have a linear equation...",
      "api_data": {
        "category": "linear_equation",
        "step_type": "identification",
        "operation": "classify_equation",
        "inputs": {
          "equation": "2x + 3 = 0",
          "variable": "x"
        },
        "outputs": {
          "equation_type": "linear",
          "standard_form": "ax + b = 0"
        }
      },
      "message_key": {
        "category": "linear_equation",
        "message_type": "introduction",
        "variant": 0,
        "hash": 12345678901234567
      },
      "math_context": {
        "equation": "2x + 3 = 0",
        "variable": "x",
        "current_state": "2x + 3 = 0",
        "coefficients": {
          "a": "2",
          "b": "3"
        },
        "progress": 0.25,
        "equation_type": "linear"
      }
    }
  ]
}
```

### API Data Structure

```rust
pub struct StepApiData {
    pub category: String,                    // "linear_equation", "calculus", etc.
    pub step_type: String,                   // "identification", "transformation", etc.
    pub operation: String,                   // "subtract_constant", "divide_coefficient", etc.
    pub inputs: HashMap<String, String>,     // Operation inputs
    pub outputs: HashMap<String, String>,    // Operation results
    pub properties: HashMap<String, serde_json::Value>,  // Additional metadata
}
```

**Mathematical Context:**

```rust
pub struct MathContext {
    pub equation: String,                    // Current equation state
    pub variable: String,                    // Variable being solved for
    pub current_state: String,               // Expression at this step
    pub coefficients: HashMap<String, String>,  // Extracted coefficients
    pub progress: f64,                       // 0.0 to 1.0
    pub equation_type: String,               // Classification
}
```

## SmartStepFactory

The `SmartStepFactory` generates contextual educational steps based on operation type and difficulty.

### Basic Usage

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate introduction step for linear equation
let intro_step = EnhancedStepBuilder::new("step_1")
    .with_human_message(
        "Identify Equation Type",
        "We have a linear equation in one variable"
    )
    .with_api_data("linear_equation", "identification", "classify")
    .with_input("equation", "2x + 3 = 0")
    .with_input("variable", "x")
    .with_output("equation_type", "linear")
    .with_output("degree", "1")
    .with_math_context("2x + 3 = 0", "x", 0.25)
    .with_message_key("linear_equation", "introduction", 0)
    .build();
```

### Contextual Step Generation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Factory determines appropriate messaging based on context
fn generate_solving_step(
    equation: &Expression,
    variable: &Symbol,
    operation: &str,
    progress: f64
) -> EnhancedStep {
    let step_id = format!("step_{}", (progress * 10.0) as usize);

    EnhancedStepBuilder::new(&step_id)
        .with_human_message(
            "Apply Operation",
            &format!("Apply {} to both sides", operation)
        )
        .with_api_data("equation_solving", "transformation", operation)
        .with_input("current_equation", &equation.to_string())
        .with_input("operation", operation)
        .with_output("transformed_equation", &transform(equation).to_string())
        .with_math_context(&equation.to_string(), variable.name(), progress)
        .build()
}
```

### Difficulty Level Adaptation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Adapt explanation depth based on student level
fn generate_adaptive_steps(
    equation: &Expression,
    difficulty: u8
) -> Vec<EnhancedStep> {
    match difficulty {
        1..=3 => generate_beginner_steps(equation),      // Detailed explanations
        4..=6 => generate_intermediate_steps(equation),  // Standard detail
        7..=10 => generate_advanced_steps(equation),     // Concise, assumes knowledge
        _ => generate_intermediate_steps(equation),
    }
}
```

## Educational Operation Trait

Implement this trait to add educational capabilities to any mathematical operation.

### Trait Definition

```rust
pub trait EducationalOperation {
    type Output;

    /// Execute with full educational explanation
    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation);

    /// Get operation metadata
    fn educational_context(&self) -> OperationContext;

    /// Fast execution without explanation (default implementation)
    fn execute_fast(&self) -> Self::Output {
        let (result, _explanation) = self.execute_with_steps();
        result
    }

    /// Check if explanation is available
    fn can_explain(&self) -> bool {
        true
    }

    /// Estimate number of steps
    fn estimated_steps(&self) -> Option<usize> {
        None
    }
}
```

### Example Implementation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
struct LinearEquationSolver {
    equation: Expression,
    variable: Symbol,
}

impl EducationalOperation for LinearEquationSolver {
    type Output = Vec<Expression>;

    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation) {
        let mut steps = Vec::new();

        // Step 1: Identify equation type
        steps.push(Step::new(
            "Identify Equation Type",
            "This is a linear equation ax + b = 0"
        ));

        // Step 2: Isolate variable term
        steps.push(Step::new(
            "Isolate Variable Term",
            "Subtract constant from both sides"
        ));

        // Step 3: Solve for variable
        steps.push(Step::new(
            "Solve for Variable",
            "Divide both sides by coefficient"
        ));

        // Perform actual solving
        let solution = self.solve_internal();

        let explanation = StepByStepExplanation::new(steps);
        (solution, explanation)
    }

    fn educational_context(&self) -> OperationContext {
        OperationContext::equation_solving(3)  // difficulty level 3
    }

    fn execute_fast(&self) -> Self::Output {
        // Optimized path without explanation generation
        self.solve_internal()
    }

    fn estimated_steps(&self) -> Option<usize> {
        Some(3)  // Known step count for linear equations
    }
}

impl LinearEquationSolver {
    fn solve_internal(&self) -> Vec<Expression> {
        // Actual solving logic
        vec![/* solutions */]
    }
}
```

### Using Educational Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solver = LinearEquationSolver {
    equation: expr!(2*x + 3),
    variable: symbol!(x),
};

// Educational mode
let (solution, explanation) = solver.execute_with_steps();
println!("Solution: {:?}", solution);
println!("Steps: {}", explanation.steps.len());

// Performance mode
let fast_solution = solver.execute_fast();
```

## LMS Integration

### Export for LMS Consumption

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate explanation
let explanation = EnhancedStepExplanation::new(steps);

// Export to JSON
let json = explanation.to_json()?;

// Send to LMS via REST API
let client = reqwest::Client::new();
let response = client
    .post("https://lms.example.com/api/lessons")
    .json(&serde_json::from_str::<serde_json::Value>(&json)?)
    .send()
    .await?;
```

### LMS Feature Integration

**1. Progress Tracking:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Track which steps student has viewed
for step in &explanation.steps {
    let viewed = lms_api.mark_step_viewed(
        student_id,
        lesson_id,
        &step.step_id
    ).await?;

    // Record time spent on step
    lms_api.record_time_on_step(
        student_id,
        &step.step_id,
        time_spent_seconds
    ).await?;
}
```

**2. Comprehension Testing:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate quiz questions based on steps
for step in &explanation.steps {
    if step.api_data.operation == "combine_like_terms" {
        let quiz = generate_quiz_question(&step);
        lms_api.add_quiz_question(lesson_id, quiz).await?;
    }
}
```

**3. Adaptive Difficulty:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Adjust difficulty based on student performance
let performance = lms_api.get_student_performance(student_id).await?;

let difficulty = if performance.success_rate < 0.6 {
    2  // Easier explanations
} else if performance.success_rate > 0.9 {
    5  // More advanced
} else {
    3  // Standard
};

// Generate content at appropriate level
let steps = generate_adaptive_steps(equation, difficulty);
```

**4. Similar Problem Generation:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Use API data to generate similar problems
let problem_template = &step.api_data;

let similar_problems = generate_similar_problems(
    &problem_template.category,
    &problem_template.inputs,
    difficulty_level
);

for problem in similar_problems {
    lms_api.add_practice_problem(lesson_id, problem).await?;
}
```

## Mobile App Integration

### REST API Design

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Backend API endpoint
#[post("/api/solve")]
async fn solve_endpoint(equation: Json<EquationRequest>) -> Json<EducationalResponse> {
    let solver = create_solver(&equation.expression, &equation.variable);
    let (solution, steps) = solver.execute_with_steps();

    let explanation = EnhancedStepExplanation::new(steps);

    Json(EducationalResponse {
        solution: solution.to_string(),
        explanation: explanation.to_json().unwrap(),
        metadata: explanation.metadata,
    })
}
```

### Mobile Client Integration

```swift
// iOS Swift client
struct EducationalAPI {
    func getSolution(equation: String, variable: String) async throws -> EducationalResponse {
        let endpoint = "https://api.example.com/api/solve"
        let body = ["expression": equation, "variable": variable]

        let response = try await URLSession.shared.post(endpoint, json: body)
        return try JSONDecoder().decode(EducationalResponse.self, from: response)
    }
}

// Usage in SwiftUI
struct SolverView: View {
    @State private var explanation: EducationalExplanation?

    var body: some View {
        VStack {
            ForEach(explanation?.steps ?? []) { step in
                StepCard(step: step)
                    .onTapGesture {
                        // Show detailed view
                        showStepDetail(step)
                    }
            }
        }
    }
}
```

### React Native Integration

```javascript
// React Native client
import { EducationalAPI } from './api';

const SolverScreen = () => {
  const [explanation, setExplanation] = useState(null);

  const solveEquation = async (equation, variable) => {
    const response = await EducationalAPI.solve(equation, variable);
    setExplanation(response.explanation);
  };

  return (
    <ScrollView>
      {explanation?.steps.map((step, index) => (
        <StepCard
          key={step.step_id}
          step={step}
          onPress={() => navigateToDetail(step)}
        />
      ))}
    </ScrollView>
  );
};
```

## Assessment and Verification

### Automatic Answer Verification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Verify student's answer against expected solution
fn verify_answer(
    student_answer: &str,
    expected_solution: &Expression,
    variable: &Symbol
) -> VerificationResult {
    let student_expr = parse_latex(student_answer)?;

    // Substitute student's answer into original equation
    let substituted = original_equation.substitute(variable, &student_expr);
    let simplified = substituted.simplify();

    VerificationResult {
        correct: simplified == Expression::integer(0),
        student_expression: student_expr,
        substituted_form: substituted,
        explanation: generate_verification_explanation(&simplified),
    }
}
```

### Step-by-Step Verification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Check student's work at each step
fn verify_student_work(
    expected_steps: &[EnhancedStep],
    student_steps: &[StudentStep]
) -> StepVerification {
    let mut results = Vec::new();

    for (expected, student) in expected_steps.iter().zip(student_steps.iter()) {
        let correct = compare_expressions(
            &expected.math_context.current_state,
            &student.expression
        );

        results.push(StepResult {
            step_id: expected.step_id.clone(),
            correct,
            feedback: generate_feedback(expected, student, correct),
        });
    }

    StepVerification {
        overall_correct: results.iter().all(|r| r.correct),
        step_results: results,
    }
}
```

### Partial Credit Calculation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Award partial credit based on correct steps
fn calculate_partial_credit(verification: &StepVerification) -> f64 {
    let total_steps = verification.step_results.len() as f64;
    let correct_steps = verification.step_results
        .iter()
        .filter(|r| r.correct)
        .count() as f64;

    let base_score = correct_steps / total_steps;

    // Bonus for getting final answer correct
    let final_bonus = if verification.overall_correct { 0.1 } else { 0.0 };

    (base_score + final_bonus).min(1.0)
}
```

## Performance Considerations

### Fast Path vs Educational Path

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Choose execution path based on needs
if educational_mode_enabled {
    let (result, explanation) = operation.execute_with_steps();
    // 10-100μs overhead for explanation generation
    send_to_frontend(result, explanation);
} else {
    let result = operation.execute_fast();
    // Minimal overhead, 0.1-1μs
    send_to_frontend_minimal(result);
}
```

### Caching Educational Content

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Cache common explanations
static EXPLANATION_CACHE: Lazy<HashMap<String, EnhancedStepExplanation>> =
    Lazy::new(|| {
        let mut cache = HashMap::new();
        // Pre-compute common patterns
        cache.insert("linear_simple".to_string(), precompute_linear_simple());
        cache.insert("quadratic_std".to_string(), precompute_quadratic_std());
        cache
    });

// Fast retrieval
fn get_cached_explanation(pattern: &str) -> Option<&EnhancedStepExplanation> {
    EXPLANATION_CACHE.get(pattern)
}
```

**Performance Metrics:**

| Operation | Without Educational | With Educational | Overhead |
|-----------|-------------------|------------------|----------|
| Linear solve | 1-2μs | 15-25μs | ~20μs |
| Quadratic solve | 5-10μs | 50-100μs | ~50μs |
| Simplification | 2-5μs | 30-60μs | ~40μs |
| Differentiation | 3-8μs | 40-80μs | ~50μs |

**Cache Performance:**
- Cache hit: 100-200ns (instant retrieval)
- Cache miss: Full generation time
- Cache hit ratio for common problems: 70-90%

### Lazy Explanation Generation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate only when needed
struct LazyExplanation {
    operation: Box<dyn EducationalOperation<Output = Expression>>,
    cached_explanation: OnceCell<StepByStepExplanation>,
}

impl LazyExplanation {
    fn get_explanation(&self) -> &StepByStepExplanation {
        self.cached_explanation.get_or_init(|| {
            let (_result, explanation) = self.operation.execute_with_steps();
            explanation
        })
    }
}
```

## Complete Examples

### Example 1: Complete LMS Integration

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
async fn create_lms_lesson(equation_str: &str, variable_str: &str) -> Result<LessonId, Error> {
    // Parse and solve
    let equation = parse_latex(equation_str)?;
    let variable = symbol!(variable_str);

    let solver = LinearEquationSolver::new(equation, variable);
    let (solution, steps) = solver.execute_with_steps();

    // Generate enhanced explanation
    let explanation = EnhancedStepExplanation::new(steps);

    // Create LMS lesson
    let lesson = LmsLesson {
        title: format!("Solving {}", equation_str),
        difficulty: explanation.metadata.difficulty_level,
        estimated_time: explanation.metadata.estimated_time,
        prerequisites: explanation.metadata.prerequisites.clone(),
        content: explanation.to_json()?,
    };

    // Upload to LMS
    let lesson_id = lms_api.create_lesson(lesson).await?;

    // Add quiz questions
    for step in &explanation.steps {
        if let Some(quiz) = generate_quiz_from_step(step) {
            lms_api.add_quiz_to_lesson(lesson_id, quiz).await?;
        }
    }

    Ok(lesson_id)
}
```

### Example 2: Mobile App with Offline Support

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate offline content package
fn generate_offline_package(topics: &[Topic]) -> OfflinePackage {
    let mut lessons = Vec::new();

    for topic in topics {
        for example in &topic.examples {
            let solver = create_solver_for_example(example);
            let (solution, steps) = solver.execute_with_steps();
            let explanation = EnhancedStepExplanation::new(steps);

            lessons.push(OfflineLesson {
                id: generate_id(),
                title: example.title.clone(),
                explanation: explanation.to_json().unwrap(),
                practice_problems: generate_similar_problems(example),
            });
        }
    }

    OfflinePackage {
        version: "1.0".to_string(),
        lessons,
        last_updated: Utc::now(),
    }
}
```

### Example 3: Real-Time Tutoring Assistant

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Interactive tutoring session
async fn tutoring_session(student_id: StudentId, equation: Expression) {
    let solver = create_solver(&equation);
    let (solution, steps) = solver.execute_with_steps();
    let explanation = EnhancedStepExplanation::new(steps);

    // Present steps one at a time
    for (i, step) in explanation.steps.iter().enumerate() {
        // Show step to student
        display_step(step);

        // Wait for student to attempt
        let attempt = wait_for_student_input().await;

        // Verify attempt
        let correct = verify_step_attempt(&step, &attempt);

        if !correct {
            // Provide hint
            display_hint(&step, &attempt);

            // Record struggle for adaptive learning
            record_difficulty(student_id, &step.step_id).await;
        } else {
            // Record success
            record_mastery(student_id, &step.step_id).await;
        }
    }

    // Final assessment
    let final_score = calculate_session_score(student_id).await;
    update_student_progress(student_id, final_score).await;
}
```

## See Also

- [Step-by-Step Explanations](./step-by-step.md) - Core educational feature basics
- [Message Registry](./messages.md) - Customizing educational language
- [Architecture](../architecture/design-principles.md) - System design and patterns
- [Python Bindings](../bindings/python.md) - Using educational API from Python
- [Node.js Bindings](../bindings/nodejs.md) - Using educational API from JavaScript/TypeScript

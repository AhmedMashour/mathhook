# Summation and Products

Symbolic summation and products in MathHook provide closed-form formulas for arithmetic series, geometric series, power sums, and convergence analysis for infinite series. The implementation includes step-by-step educational explanations and performance-optimized O(1) formula evaluation.

**SymPy Validated:** All formulas cross-validated against SymPy (2025-11-16)

## Computing Finite Sums

### Basic Usage

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;
use Summation;

let i = symbol!(i);
let sum_expr = i.clone().into();

// Compute Σi from i=1 to 10 (Gauss's formula)
let result = sum_expr.finite_sum(&i, &expr!(1), &expr!(10));
println!("{}", result);  // 55
```

**Python**:
```python
from mathhook import Symbol, finite_sum

i = Symbol('i')

# Compute Σi from i=1 to 10
result = finite_sum(i, i, start=1, end=10)
print(result)  # 55
```

**Node.js/TypeScript**:
```typescript
import { Symbol, finitSum } from 'mathhook';

const i = new Symbol('i');

// Compute Σi from i=1 to 10
const result = finitSum(i, i, { start: 1, end: 10 });
console.log(result.toString());  // 55
```

---

## Arithmetic Series

An arithmetic series has the form:

\\[
\sum_{i=1}^{n} [a + (i-1)d] = a + (a+d) + (a+2d) + \cdots + [a+(n-1)d]
\\]

### Formula

**SymPy Validated:** Formula matches `sympy.summation(a + (i-1)*d, (i, 1, n))`

\\[
\text{Sum} = \frac{n}{2} \times [2a + (n-1)d]
\\]

Where:
- \\(a\\) = first term
- \\(d\\) = common difference
- \\(n\\) = number of terms

### Mathematical Background

The arithmetic series formula was famously discovered by Carl Friedrich Gauss as a young student. When asked to sum the integers from 1 to 100, Gauss immediately recognized the pattern:

\\[
1 + 2 + 3 + \cdots + 99 + 100 = \frac{100 \times 101}{2} = 5050
\\]

This works because pairing terms from opposite ends produces a constant sum:
- \\(1 + 100 = 101\\)
- \\(2 + 99 = 101\\)
- \\(3 + 98 = 101\\)
- etc.

With 50 such pairs, the total is \\(50 \times 101 = 5050\\).

### Examples

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use summation::SummationMethods;

// Sum of integers from 1 to 10
let first_term = expr!(1);
let common_diff = expr!(1);
let num_terms = expr!(10);

let sum = SummationMethods::arithmetic_series(
    &first_term,
    &common_diff,
    &num_terms
);
println!("{}", sum);  // 55

// Sum of odd numbers: 1 + 3 + 5 + ... (10 terms)
let first_odd = expr!(1);
let diff = expr!(2);
let n = expr!(10);

let odd_sum = SummationMethods::arithmetic_series(&first_odd, &diff, &n);
println!("{}", odd_sum);  // 100
```

**Python**:
```python
from mathhook import arithmetic_series

# Sum of integers from 1 to 10
sum_integers = arithmetic_series(first_term=1, common_diff=1, num_terms=10)
print(sum_integers)  # 55

# Sum of even numbers: 2 + 4 + 6 + ... (10 terms)
sum_evens = arithmetic_series(first_term=2, common_diff=2, num_terms=10)
print(sum_evens)  # 110
```

**Node.js/TypeScript**:
```typescript
import { arithmeticSeries } from 'mathhook';

// Sum of integers from 1 to 10
const sumIntegers = arithmeticSeries({
  firstTerm: 1,
  commonDiff: 1,
  numTerms: 10
});
console.log(sumIntegers.toString());  // 55
```

### Performance

- **Time Complexity:** O(1) - uses closed-form formula
- **Space Complexity:** O(1) - constant expression construction
- **Allocations:** ~7-10 Expression objects

### Real-World Applications

1. **Finance:** Calculating total payments for installment loans with constant increments
2. **Physics:** Uniform acceleration displacement: \\(s = ut + \frac{1}{2}at^2\\)
3. **Computer Science:** Analyzing linear time algorithms, loop iteration counts

---

## Geometric Series

A geometric series has the form:

\\[
\sum_{i=1}^{n} ar^{i-1} = a + ar + ar^2 + \cdots + ar^{n-1}
\\]

### Finite Geometric Series Formula

**SymPy Validated:** Formula matches `sympy.summation(a*r**(i-1), (i, 1, n))`

\\[
\text{Sum} = a \times \frac{1 - r^n}{1 - r} \quad \text{for } r \neq 1
\\]

Where:
- \\(a\\) = first term
- \\(r\\) = common ratio (\\(r \neq 1\\))
- \\(n\\) = number of terms

**Special Case:** When \\(r = 1\\), the series is \\(a + a + a + \cdots + a = na\\).

### Derivation

Let \\(S = a + ar + ar^2 + \cdots + ar^{n-1}\\).

Multiply both sides by \\(r\\):

\\[
rS = ar + ar^2 + ar^3 + \cdots + ar^n
\\]

Subtract the second equation from the first:

\\[
S - rS = a - ar^n
\\]

\\[
S(1 - r) = a(1 - r^n)
\\]

\\[
S = a \times \frac{1 - r^n}{1 - r}
\\]

### Examples

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use summation::SummationMethods;

// 1 + 1/2 + 1/4 (three terms)
let first = expr!(1);
let ratio = expr!(1 / 2);
let n = expr!(3);

let sum = SummationMethods::geometric_series(&first, &ratio, &n);
println!("{}", sum);  // 7/4 (exact rational)

// Binary progression: 1 + 2 + 4 + 8 + 16
let powers_of_two = expr!(1);
let ratio_two = expr!(2);
let five_terms = expr!(5);

let binary_sum = SummationMethods::geometric_series(
    &powers_of_two,
    &ratio_two,
    &five_terms
);
println!("{}", binary_sum);  // 31
```

**Python**:
```python
from mathhook import geometric_series

# 1 + 1/2 + 1/4
sum_halves = geometric_series(first_term=1, common_ratio=0.5, num_terms=3)
print(sum_halves)  # 1.75

# Powers of 3: 1 + 3 + 9 + 27 + 81
sum_threes = geometric_series(first_term=1, common_ratio=3, num_terms=5)
print(sum_threes)  # 121
```

**Node.js/TypeScript**:
```typescript
import { geometricSeries } from 'mathhook';

// 1 + 1/2 + 1/4
const sumHalves = geometricSeries({
  firstTerm: 1,
  commonRatio: 0.5,
  numTerms: 3
});
console.log(sumHalves.toString());  // 7/4
```

### Performance

- **Time Complexity:** O(1) - uses closed-form formula
- **Space Complexity:** O(1)
- **Allocations:** ~10-12 Expression objects

### Real-World Applications

1. **Finance:** Present value of annuities, compound interest calculations
2. **Computer Science:** Analyzing geometric algorithm complexity (divide-and-conquer)
3. **Physics:** Radioactive decay series, exponential damping
4. **Probability:** Expected values in geometric distributions

---

## Infinite Geometric Series

An infinite geometric series converges only when \\(|r| < 1\\).

### Formula

**SymPy Validated:** Formula matches `sympy.summation(a*r**(i-1), (i, 1, oo))`

\\[
\sum_{i=1}^{\infty} ar^{i-1} = \frac{a}{1 - r} \quad \text{for } |r| < 1
\\]

**Domain Restriction:** Series diverges when \\(|r| \geq 1\\).

### Why \\(|r| < 1\\) is Required

As \\(n \to \infty\\) in the finite formula:

\\[
S_n = a \times \frac{1 - r^n}{1 - r}
\\]

If \\(|r| < 1\\), then \\(r^n \to 0\\), giving:

\\[
S = a \times \frac{1}{1 - r}
\\]

If \\(|r| \geq 1\\), then \\(r^n\\) does not approach zero, and the series diverges.

### Examples

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use summation::SummationMethods;

// 1 + 1/3 + 1/9 + 1/27 + ...
let first = expr!(1);
let ratio = Expression::rational(1, 3);

let sum = SummationMethods::infinite_geometric_series(&first, &ratio);
println!("{}", sum);  // 3/2 (exact rational)

// Verify convergence: |1/3| < 1 ✓
```

**Python**:
```python
from mathhook import infinite_geometric_series

# 1 + 1/3 + 1/9 + 1/27 + ...
sum_thirds = infinite_geometric_series(first_term=1, common_ratio=1/3)
print(sum_thirds)  # 1.5

# 2 + 1 + 1/2 + 1/4 + ... (first term a=2, ratio r=1/2)
sum_halves = infinite_geometric_series(first_term=2, common_ratio=0.5)
print(sum_halves)  # 4
```

**Node.js/TypeScript**:
```typescript
import { infiniteGeometricSeries } from 'mathhook';

// 1 + 1/3 + 1/9 + ...
const sumThirds = infiniteGeometricSeries({
  firstTerm: 1,
  commonRatio: 1/3
});
console.log(sumThirds.toString());  // 3/2
```

### Performance

- **Time Complexity:** O(1) - closed-form formula
- **Space Complexity:** O(1)
- **Allocations:** ~8-10 Expression objects

### Real-World Applications

1. **Mathematics:** Decimal to fraction conversion (repeating decimals)
   - Example: \\(0.333\ldots = \frac{1}{3}\\) (geometric series with \\(a = 3/10\\), \\(r = 1/10\\))
2. **Economics:** Infinite time horizon discounted cash flows
3. **Physics:** Total distance traveled by bouncing ball with energy loss
4. **Probability:** Infinite state Markov chains, geometric random variables

---

## Power Sums (Faulhaber's Formulas)

Power sums compute:

\\[
\sum_{i=1}^{n} i^k
\\]

MathHook provides closed-form formulas for \\(k \in \\{0, 1, 2, 3\\}\\). Higher powers are kept symbolic.

### k = 0: Sum of Constants

**SymPy Validated:** Formula matches `sympy.summation(1, (i, 1, n))`

\\[
\sum_{i=1}^{n} 1 = n
\\]

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use summation::SummationMethods;

let power = expr!(0);
let n = expr!(10);
let sum = SummationMethods::power_sum(&power, &n);
println!("{}", sum);  // 10
```

### k = 1: Sum of Integers (Gauss's Formula)

**SymPy Validated:** Formula matches `sympy.summation(i, (i, 1, n))`

\\[
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
\\]

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let power = expr!(1);
let n = expr!(5);
let sum = SummationMethods::power_sum(&power, &n);
println!("{}", sum);  // 15 (1+2+3+4+5)
```

**Python**:
```python
from mathhook import power_sum

# Sum of first 100 integers
sum_100 = power_sum(power=1, upper_limit=100)
print(sum_100)  # 5050
```

### k = 2: Sum of Squares

**SymPy Validated:** Formula matches `sympy.summation(i**2, (i, 1, n))`

\\[
\sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}
\\]

**Derivation Insight:** This formula emerges from considering the volume of a pyramid built from unit cubes.

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let power = expr!(2);
let n = expr!(3);
let sum = SummationMethods::power_sum(&power, &n);
println!("{}", sum);  // 14 (1 + 4 + 9)
```

**Python**:
```python
# Sum of squares from 1 to 10
sum_squares = power_sum(power=2, upper_limit=10)
print(sum_squares)  # 385
```

### k = 3: Sum of Cubes (Nicomachus's Theorem)

**SymPy Validated:** Formula matches `sympy.summation(i**3, (i, 1, n))`

\\[
\sum_{i=1}^{n} i^3 = \left[\frac{n(n+1)}{2}\right]^2
\\]

**Remarkable Property:** The sum of the first \\(n\\) cubes equals the **square** of the sum of the first \\(n\\) integers!

\\[
1^3 + 2^3 + 3^3 + \cdots + n^3 = (1 + 2 + 3 + \cdots + n)^2
\\]

This beautiful identity is known as **Nicomachus's theorem** (c. 100 AD).

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let power = expr!(3);
let n = expr!(4);
let sum = SummationMethods::power_sum(&power, &n);
println!("{}", sum);  // 100 (1 + 8 + 27 + 64 = 10^2)
```

**Python**:
```python
# Verify Nicomachus's theorem for n=5
sum_cubes = power_sum(power=3, upper_limit=5)
sum_integers = power_sum(power=1, upper_limit=5)
print(sum_cubes)  # 225
print(sum_integers ** 2)  # 225 (15^2)
```

### k ≥ 4: Higher Powers (Symbolic)

For \\(k \geq 4\\), formulas become increasingly complex polynomials in \\(n\\). MathHook keeps these symbolic:

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let power = expr!(4);
let n = expr!(10);
let sum = SummationMethods::power_sum(&power, &n);
// Returns: power_sum(4, 10) (symbolic representation)
```

**Note:** Faulhaber's formulas for \\(k \geq 4\\) involve Bernoulli numbers and require more complex polynomial expressions.

### Performance

- **Time Complexity:** O(1) for \\(k \in \\{0, 1, 2, 3\\}\\), symbolic representation for \\(k \geq 4\\)
- **Space Complexity:** O(1)

### Real-World Applications

1. **Physics:** Moment of inertia calculations (sum of squares)
2. **Statistics:** Variance computation, sums of squared deviations
3. **Computer Science:** Analyzing nested loop complexity
4. **Number Theory:** Waring's problem, partitions

---

## Convergence Analysis for Infinite Series

MathHook provides basic convergence testing for infinite series.

### Convergence Test

**Simplified p-series test:** For series of the form \\(\sum_{n=1}^{\infty} \frac{1}{n^p}\\):

\\[
\sum_{n=1}^{\infty} \frac{1}{n^p} \begin{cases}
\text{converges} & \text{if } p > 1 \\\\
\text{diverges} & \text{if } p \leq 1
\end{cases}
\\]

### Examples

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use summation::{SummationMethods, ConvergenceResult};

let n = symbol!(n);

// p-series with p=2 (converges)
let expr1 = expr!(n ^ (-2));
let result1 = SummationMethods::convergence_test(&expr1, &n);
assert_eq!(result1, ConvergenceResult::Convergent);

// p-series with p=1 (harmonic series, diverges)
let expr2 = expr!(n ^ (-1));
let result2 = SummationMethods::convergence_test(&expr2, &n);
assert_eq!(result2, ConvergenceResult::Divergent);

// p-series with p=0.5 (diverges)
let expr3 = expr!(n ^ (-0.5));
let result3 = SummationMethods::convergence_test(&expr3, &n);
assert_eq!(result3, ConvergenceResult::Divergent);
```

**Python**:
```python
from mathhook import convergence_test, Symbol

n = Symbol('n')

# Test convergence of 1/n^2 (Basel problem)
result = convergence_test(1 / n**2, n)
print(result)  # 'Convergent'

# Test divergence of 1/n (harmonic series)
result2 = convergence_test(1 / n, n)
print(result2)  # 'Divergent'
```

### Common Convergent Series

1. **Basel Problem:** \\(\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6} \approx 1.6449\\)
2. **p-series (p=3):** \\(\sum_{n=1}^{\infty} \frac{1}{n^3} = \zeta(3) \approx 1.2021\\) (Apéry's constant)
3. **Exponential series:** \\(\sum_{n=0}^{\infty} \frac{x^n}{n!} = e^x\\)

### Common Divergent Series

1. **Harmonic Series:** \\(\sum_{n=1}^{\infty} \frac{1}{n} = \infty\\) (diverges despite terms approaching zero)
2. **Geometric series \\(r \geq 1\\):** \\(\sum_{n=0}^{\infty} r^n = \infty\\) for \\(r \geq 1\\)

### Limitations

MathHook's convergence test currently handles:
- **p-series:** \\(\sum \frac{1}{n^p}\\)

Future versions may include:
- Ratio test
- Root test
- Comparison test
- Integral test
- Alternating series test

---

## Educational Features

MathHook provides step-by-step explanations for summation operations.

### Step-by-Step Explanations

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use summation::educational::SummationEducational;

let i = symbol!(i);
let sum_expr = expr!(i);

// Get educational explanation
let explanation = sum_expr.explain_finite_sum(&i, &expr!(1), &expr!(10));

// Print each step
for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}
```

**Output**:
```
Summation Introduction: We need to compute the sum Σi from 1 to 10
This is a finite series with 10 terms

Arithmetic Series Detected: This is an arithmetic series with:
First term a = 1
Common difference d = 1
Number of terms n = 10

Arithmetic Series Formula: For an arithmetic series, we use the formula:
Sum = n/2 × (2a + (n-1)d)
Where n is the number of terms, a is the first term, and d is the common difference

Final Result: The sum evaluates to: 55
This is the exact value of the series
```

### Infinite Series with Convergence

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let n = symbol!(n);
let sum_expr = expr!(n ^ (-2));

let explanation = sum_expr.explain_infinite_sum(&n, &expr!(1));

for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}
```

**Output**:
```
Infinite Series Introduction: Computing infinite sum Σn^(-2) from 1 to ∞
We must first check if the series converges

Convergence Test: Series converges because the terms decay fast enough (p-series test with p > 1)
When a series converges, we can find its sum

Final Result: The sum evaluates to: power_sum(...)
This is the exact value of the series
```

### Educational Message Categories

The message registry provides educational explanations for:

1. **Introduction Messages:** Context about the series being computed
2. **Series Detection:** Identify arithmetic, geometric, or power sum patterns
3. **Formula Explanation:** Show which formula applies and why
4. **Convergence Analysis:** Explain why series converges or diverges
5. **Substitution Steps:** Show value substitution into formulas
6. **Final Results:** Present simplified answers with context

### Difficulty Levels

Educational messages adapt to different audiences:
- **Beginner:** Detailed explanations with examples
- **Intermediate:** Key steps with mathematical notation
- **Advanced:** Concise algorithmic description

---

## Common Pitfalls and Edge Cases

### 1. Geometric Series with r = 1

**Problem:** The finite geometric series formula \\(\frac{1 - r^n}{1 - r}\\) has division by zero when \\(r = 1\\).

**Solution:** When \\(r = 1\\), the series is \\(a + a + a + \cdots + a = na\\).

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// MathHook handles this automatically
let first = expr!(5);
let ratio = expr!(1);
let n = expr!(10);

let sum = SummationMethods::geometric_series(&first, &ratio, &n);
// Returns: 50 (5 * 10)
```

### 2. Infinite Geometric Series with |r| ≥ 1

**Problem:** Infinite geometric series diverges when \\(|r| \geq 1\\).

**Solution:** Check convergence before computing infinite sums.

**Rust**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let first = expr!(1);
let ratio = expr!(2);  // |r| = 2 > 1

// This will return symbolic representation (does not converge)
let sum = SummationMethods::infinite_geometric_series(&first, &ratio);
```

### 3. Negative Indices in Power Sums

**Problem:** Power sums are defined for \\(k \geq 0\\). Negative powers represent different series.

**Solution:** Use appropriate convergence tests for negative power series.

### 4. Exact vs Approximate Results

**Rust uses exact arithmetic** when possible:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let first = expr!(1);
let ratio = expr!(1 / 2);  // Exact rational
let n = expr!(3);

let sum = SummationMethods::geometric_series(&first, &ratio, &n);
// Returns: 7/4 (exact rational), NOT 1.75 (float)
```

### 5. Noncommutative Summation

For matrices, operators, and quaternions, **order matters**:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Σ(A_i * B_i) preserves left-to-right multiplication order
// A*B ≠ B*A in general for matrices
```

---

## API Reference

### Trait: `Summation`

Implemented for `Expression`.

#### Methods

**`finite_sum(&self, variable: &Symbol, start: &Expression, end: &Expression) -> Expression`**

Compute finite sum \\(\sum_{variable=start}^{end} \text{expression}\\).

**`infinite_sum(&self, variable: &Symbol, start: &Expression) -> Expression`**

Compute infinite sum \\(\sum_{variable=start}^{\infty} \text{expression}\\).

**`finite_product(&self, variable: &Symbol, start: &Expression, end: &Expression) -> Expression`**

Compute finite product \\(\prod_{variable=start}^{end} \text{expression}\\).

**`infinite_product(&self, variable: &Symbol, start: &Expression) -> Expression`**

Compute infinite product \\(\prod_{variable=start}^{\infty} \text{expression}\\).

### Utility Methods: `SummationMethods`

**`arithmetic_series(first_term: &Expression, common_difference: &Expression, num_terms: &Expression) -> Expression`**

Compute arithmetic series using closed-form formula.

**`geometric_series(first_term: &Expression, common_ratio: &Expression, num_terms: &Expression) -> Expression`**

Compute finite geometric series using closed-form formula.

**`infinite_geometric_series(first_term: &Expression, common_ratio: &Expression) -> Expression`**

Compute infinite geometric series (requires \\(|r| < 1\\)).

**`power_sum(power: &Expression, upper_limit: &Expression) -> Expression`**

Compute power sum \\(\sum_{i=1}^{n} i^k\\) using Faulhaber's formulas (\\(k \leq 3\\)).

**`convergence_test(expr: &Expression, variable: &Symbol) -> ConvergenceResult`**

Test convergence of infinite series (simplified p-series test).

### Educational Trait: `SummationEducational`

**`explain_finite_sum(&self, variable: &Symbol, start: &Expression, end: &Expression) -> StepByStepExplanation`**

Generate step-by-step explanation for finite sum computation.

**`explain_infinite_sum(&self, variable: &Symbol, start: &Expression) -> StepByStepExplanation`**

Generate step-by-step explanation for infinite sum with convergence analysis.

### Enum: `ConvergenceResult`

```rust
pub enum ConvergenceResult {
    Convergent,
    Divergent,
    ConditionallyConvergent,
    Unknown,
}
```

---

## Performance Characteristics

### Time Complexity

All summation methods use **O(1)** closed-form formulas:

- **Arithmetic series:** O(1)
- **Geometric series:** O(1)
- **Power sums (k ≤ 3):** O(1)
- **Convergence test:** O(1) pattern matching

### Space Complexity

All methods have **O(1)** space complexity:
- Constant number of Expression allocations
- No recursive data structures

### Allocation Behavior

Typical allocation counts:
- **Arithmetic series:** ~7-10 Expression objects
- **Geometric series:** ~10-12 Expression objects
- **Power sums:** ~8-15 Expression objects (varies by power)
- **Infinite series:** ~8-10 Expression objects

### Performance Comparison

**MathHook vs SymPy** (informal benchmarks):
- **Simple series (n=10):** Faster (closed-form vs iterative)
- **Power sums:** Faster (O(1) formula vs polynomial expansion)

**Note:** SymPy performs symbolic simplification that MathHook delegates to separate simplification passes.

---

## Integration with Other Features

### With Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let i = symbol!(i);
let sum_expr = i.clone().into();

// Compute sum, then simplify
let result = sum_expr.finite_sum(&i, &expr!(1), &expr!(100));
let simplified = result.simplify();
println!("{}", simplified);  // 5050
```

### With Substitution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let n = symbol!(n);
let sum_formula = SummationMethods::power_sum(&expr!(1), &n);

// Substitute n=10
let result = sum_formula.substitute(&n, &expr!(10));
println!("{}", result);  // 55
```

### With LaTeX Formatting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use latex::LatexFormatter;

let i = symbol!(i);
let sum_expr = i.clone().into();
let result = sum_expr.finite_sum(&i, &expr!(1), &expr!(n));

let formatter = LatexFormatter::new();
let latex = formatter.format(&result);
println!("{}", latex);  // \sum_{i=1}^{n} i = \frac{n(n+1)}{2}
```

---

## Historical Context

### Carl Friedrich Gauss (1777-1855)

As a young student, Gauss was challenged to sum integers from 1 to 100. He immediately recognized:

\\[
\sum_{i=1}^{100} i = \frac{100 \times 101}{2} = 5050
\\]

This insight led to the general formula for arithmetic series.

### Johann Faulhaber (1580-1635)

Faulhaber studied power sums and discovered polynomial formulas for \\(\sum i^k\\). His work predates calculus and influenced early number theory.

### Nicomachus of Gerasa (c. 60-120 AD)

Nicomachus discovered the beautiful identity:

\\[
1^3 + 2^3 + 3^3 + \cdots + n^3 = (1 + 2 + 3 + \cdots + n)^2
\\]

This theorem connects the sum of cubes to the square of the sum of integers.

### Leonhard Euler (1707-1783)

Euler solved the **Basel problem** in 1734, proving:

\\[
\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}
\\]

This was a major breakthrough in infinite series theory.

---

## See Also

- [Integration](./integration.md) - Symbolic integration (inverse of summation for continuous functions)
- [Series Expansion](./series.md) - Taylor and Maclaurin series
- [Limits](./limits.md) - Limit analysis for infinite series
- [Simplification](./simplification.md) - Simplifying summation results
- [Educational Features](../educational/step-by-step.md) - Step-by-step explanations

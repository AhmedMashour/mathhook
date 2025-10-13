# Differentiation

Symbolic differentiation in MathHook uses automatic differentiation with the chain rule, product rule, quotient rule, and function-specific derivative rules.

## Computing Derivatives

### Rust

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

// First derivative: 3x^2
let derivative = expr.derivative(&x, 1);
println!("{}", derivative);

// Second derivative: 6x
let second_derivative = expr.derivative(&x, 2);
println!("{}", second_derivative);
```

### Python

```python
from mathhook import Symbol, derivative

x = Symbol('x')
expr = x**3

# First derivative
df = derivative(expr, x)
print(df)  # 3*x^2

# Second derivative
d2f = derivative(expr, x, order=2)
print(d2f)  # 6*x
```

### Node.js/TypeScript

```typescript
import { Symbol, parse, derivative } from 'mathhook';

const x = new Symbol('x');
const expr = parse('x^3');

// First derivative
const df = derivative(expr, x);
console.log(df.toString());  // 3*x^2

// Second derivative
const d2f = derivative(expr, x, { order: 2 });
console.log(d2f.toString());  // 6*x
```

---

## Differentiation Rules

### Power Rule

\\[
\frac{d}{dx} x^n = n x^{n-1}
\\]

**Rust**:
```rust
let x = symbol!(x);
let expr = expr!(x ^ 5);
let deriv = expr.derivative(&x, 1);
// Result: 5 * x^4
```

**Python**:
```python
x = Symbol('x')
expr = x**5
deriv = derivative(expr, x)
# Result: 5 * x^4
```

**Node.js**:
```typescript
const x = new Symbol('x');
const expr = parse('x^5');
const deriv = derivative(expr, x);
// Result: 5 * x^4
```

### Product Rule

\\[
\frac{d}{dx} [f(x) \cdot g(x)] = f'(x) \cdot g(x) + f(x) \cdot g'(x)
\\]

**Rust**:
```rust
let x = symbol!(x);
let f = expr!(x ^ 2);
let g = expr!(x ^ 3);
let product = expr!(mul: f, g);  // x^2 * x^3

let deriv = product.derivative(&x, 1);
// Result: 2*x * x^3 + x^2 * 3*x^2 = 5*x^4
```

**Python**:
```python
x = Symbol('x')
f = x**2
g = x**3
product = f * g  # x^2 * x^3

deriv = derivative(product, x)
# Result: 5*x^4
```

**Node.js**:
```typescript
const x = new Symbol('x');
const expr = parse('x^2 * x^3');
const deriv = derivative(expr, x);
// Result: 5*x^4
```

### Quotient Rule

\\[
\frac{d}{dx} \frac{f(x)}{g(x)} = \frac{f'(x) \cdot g(x) - f(x) \cdot g'(x)}{[g(x)]^2}
\\]

**Rust**:
```rust
let x = symbol!(x);
let numerator = expr!(x ^ 2);
let denominator = expr!(x + 1);
let quotient = expr!(div: numerator, denominator);

let deriv = quotient.derivative(&x, 1);
// Result: (2*x*(x+1) - x^2*1) / (x+1)^2
```

**Python**:
```python
x = Symbol('x')
numerator = x**2
denominator = x + 1
quotient = numerator / denominator

deriv = derivative(quotient, x)
# Result: (2*x*(x+1) - x^2) / (x+1)^2
```

**Node.js**:
```typescript
const x = new Symbol('x');
const expr = parse('x^2 / (x + 1)');
const deriv = derivative(expr, x);
// Result: (2*x*(x+1) - x^2) / (x+1)^2
```

### Chain Rule

\\[
\frac{d}{dx} f(g(x)) = f'(g(x)) \cdot g'(x)
\\]

**Rust**:
```rust
let x = symbol!(x);
let inner = expr!(x ^ 2);
let outer = expr!(sin(inner));  // sin(x^2)

let deriv = outer.derivative(&x, 1);
// Result: cos(x^2) * 2*x
```

**Python**:
```python
from mathhook import sin

x = Symbol('x')
inner = x**2
outer = sin(inner)  # sin(x^2)

deriv = derivative(outer, x)
# Result: cos(x^2) * 2*x
```

**Node.js**:
```typescript
const x = new Symbol('x');
const expr = parse('sin(x^2)');
const deriv = derivative(expr, x);
// Result: cos(x^2) * 2*x
```

---

## Trigonometric Derivatives

| Function | Derivative |
|----------|-----------|
| \\(\sin(x)\\) | \\(\cos(x)\\) |
| \\(\cos(x)\\) | \\(-\sin(x)\\) |
| \\(\tan(x)\\) | \\(\sec^2(x)\\) |
| \\(\cot(x)\\) | \\(-\csc^2(x)\\) |
| \\(\sec(x)\\) | \\(\sec(x) \tan(x)\\) |
| \\(\csc(x)\\) | \\(-\csc(x) \cot(x)\\) |

**Example**:

**Rust**:
```rust
let x = symbol!(x);
let expr = expr!(sin(x));
let deriv = expr.derivative(&x, 1);
// Result: cos(x)
```

**Python**:
```python
from mathhook import sin

x = Symbol('x')
expr = sin(x)
deriv = derivative(expr, x)
# Result: cos(x)
```

**Node.js**:
```typescript
const x = new Symbol('x');
const expr = parse('sin(x)');
const deriv = derivative(expr, x);
// Result: cos(x)
```

---

## Exponential and Logarithmic Derivatives

| Function | Derivative |
|----------|-----------|
| \\(e^x\\) | \\(e^x\\) |
| \\(a^x\\) | \\(a^x \ln(a)\\) |
| \\(\ln(x)\\) | \\(\frac{1}{x}\\) |
| \\(\log_a(x)\\) | \\(\frac{1}{x \ln(a)}\\) |

**Example**:

**Rust**:
```rust
let x = symbol!(x);
let expr = expr!(exp(x));
let deriv = expr.derivative(&x, 1);
// Result: exp(x)

let ln_expr = expr!(log(x));
let ln_deriv = ln_expr.derivative(&x, 1);
// Result: 1/x
```

**Python**:
```python
from mathhook import exp, log

x = Symbol('x')

# Exponential
expr = exp(x)
deriv = derivative(expr, x)
# Result: exp(x)

# Logarithm
ln_expr = log(x)
ln_deriv = derivative(ln_expr, x)
# Result: 1/x
```

**Node.js**:
```typescript
const x = new Symbol('x');

// Exponential
const expr = parse('exp(x)');
const deriv = derivative(expr, x);
// Result: exp(x)

// Logarithm
const lnExpr = parse('log(x)');
const lnDeriv = derivative(lnExpr, x);
// Result: 1/x
```

---

## Partial Derivatives

For multivariable functions, compute partial derivatives with respect to each variable.

**Rust**:
```rust
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x ^ 2) * y);

// Partial derivative with respect to x
let df_dx = expr.derivative(&x, 1);
// Result: 2*x*y

// Partial derivative with respect to y
let df_dy = expr.derivative(&y, 1);
// Result: x^2
```

**Python**:
```python
x = Symbol('x')
y = Symbol('y')
expr = x**2 * y

# Partial derivative with respect to x
df_dx = derivative(expr, x)
# Result: 2*x*y

# Partial derivative with respect to y
df_dy = derivative(expr, y)
# Result: x^2
```

**Node.js**:
```typescript
const x = new Symbol('x');
const y = new Symbol('y');
const expr = parse('x^2 * y');

// Partial derivative with respect to x
const df_dx = derivative(expr, x);
// Result: 2*x*y

// Partial derivative with respect to y
const df_dy = derivative(expr, y);
// Result: x^2
```

---

## Higher-Order Derivatives

Compute second, third, or nth order derivatives by specifying the order parameter.

**Rust**:
```rust
let x = symbol!(x);
let expr = expr!(x ^ 4);

// First derivative: 4*x^3
let first = expr.derivative(&x, 1);

// Second derivative: 12*x^2
let second = expr.derivative(&x, 2);

// Third derivative: 24*x
let third = expr.derivative(&x, 3);

// Fourth derivative: 24
let fourth = expr.derivative(&x, 4);
```

**Python**:
```python
x = Symbol('x')
expr = x**4

# First derivative: 4*x^3
first = derivative(expr, x, order=1)

# Second derivative: 12*x^2
second = derivative(expr, x, order=2)

# Third derivative: 24*x
third = derivative(expr, x, order=3)

# Fourth derivative: 24
fourth = derivative(expr, x, order=4)
```

**Node.js**:
```typescript
const x = new Symbol('x');
const expr = parse('x^4');

// First derivative: 4*x^3
const first = derivative(expr, x, { order: 1 });

// Second derivative: 12*x^2
const second = derivative(expr, x, { order: 2 });

// Third derivative: 24*x
const third = derivative(expr, x, { order: 3 });

// Fourth derivative: 24
const fourth = derivative(expr, x, { order: 4 });
```

---

## Real-World Examples

### Example 1: Velocity and Acceleration

Position function: \\(s(t) = t^3 - 6t^2 + 9t\\)

Velocity (first derivative): \\(v(t) = 3t^2 - 12t + 9\\)

Acceleration (second derivative): \\(a(t) = 6t - 12\\)

**Rust**:
```rust
let t = symbol!(t);
let position = expr!((t ^ 3) - (6 * (t ^ 2)) + (9 * t));

let velocity = position.derivative(&t, 1);
println!("Velocity: {}", velocity);  // 3*t^2 - 12*t + 9

let acceleration = position.derivative(&t, 2);
println!("Acceleration: {}", acceleration);  // 6*t - 12
```

**Python**:
```python
t = Symbol('t')
position = t**3 - 6*t**2 + 9*t

velocity = derivative(position, t)
print(f"Velocity: {velocity}")  # 3*t^2 - 12*t + 9

acceleration = derivative(position, t, order=2)
print(f"Acceleration: {acceleration}")  # 6*t - 12
```

**Node.js**:
```typescript
const t = new Symbol('t');
const position = parse('t^3 - 6*t^2 + 9*t');

const velocity = derivative(position, t);
console.log(`Velocity: ${velocity.toString()}`);  // 3*t^2 - 12*t + 9

const acceleration = derivative(position, t, { order: 2 });
console.log(`Acceleration: ${acceleration.toString()}`);  // 6*t - 12
```

### Example 2: Gradient (Multivariable)

For function \\(f(x, y) = x^2 + y^2\\), compute gradient \\(\nabla f = [\frac{\partial f}{\partial x}, \frac{\partial f}{\partial y}]\\)

**Rust**:
```rust
let x = symbol!(x);
let y = symbol!(y);
let f = expr!((x ^ 2) + (y ^ 2));

let df_dx = f.derivative(&x, 1);  // 2*x
let df_dy = f.derivative(&y, 1);  // 2*y

// Gradient: [2*x, 2*y]
```

**Python**:
```python
x = Symbol('x')
y = Symbol('y')
f = x**2 + y**2

df_dx = derivative(f, x)  # 2*x
df_dy = derivative(f, y)  # 2*y

# Gradient: [2*x, 2*y]
gradient = [df_dx, df_dy]
```

**Node.js**:
```typescript
const x = new Symbol('x');
const y = new Symbol('y');
const f = parse('x^2 + y^2');

const df_dx = derivative(f, x);  // 2*x
const df_dy = derivative(f, y);  // 2*y

// Gradient: [2*x, 2*y]
const gradient = [df_dx, df_dy];
```

---

## Performance Considerations

1. **Simplify After Differentiation**: Derivatives can produce complex expressions. Simplify for cleaner results.

   ```rust
   let deriv = expr.derivative(&x, 1);
   let simplified = deriv.simplify();
   ```

2. **Cache Derivatives**: For repeated use, compute once and reuse.

3. **Use Numerical Derivatives for Approximation**: For very complex expressions where symbolic derivatives are slow, consider numerical approximation (not yet in MathHook core, but can be built on top).

---

## Common Errors and Pitfalls

### Undefined Derivatives

Some functions have undefined derivatives at certain points:

- \\(\frac{d}{dx} |x| \\) is undefined at \\(x = 0\\)
- \\(\frac{d}{dx} \sqrt{x}\\) is undefined at \\(x = 0\\)

MathHook will produce symbolic results; domain checking happens during evaluation.

### Non-Differentiable Functions

Functions must be differentiable. MathHook assumes smoothness unless explicitly handling piecewise functions.

---

## Next Steps

- [Integration](./integration.md) - Symbolic integration
- [Limits](./limits.md) - Computing limits
- [Series Expansion](./series.md) - Taylor and Maclaurin series
- [Educational Features](../educational/step-by-step.md) - Step-by-step derivative explanations

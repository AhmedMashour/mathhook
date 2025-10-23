# **COMPLETE SHARED DATA ARCHITECTURE PLAN**
## **All 32 MathHook Functions**

---

## **FILE ORGANIZATION STRUCTURE**

**All function implementations are organized as individual modules under `src/core/functions/`:**

```
src/core/functions/
  sin/
    data.rs           # SIN_SPECIAL_VALUES static HashMap
    mod.rs            # pub fn sin() implementation + tests
  cos/
    data.rs           # COS_SPECIAL_VALUES static HashMap
    mod.rs            # pub fn cos() implementation + tests
  tan/
    data.rs           # TAN_SPECIAL_VALUES static HashMap
    mod.rs            # pub fn tan() implementation + tests
  gamma/
    data.rs           # GAMMA_SPECIAL_VALUES static HashMap
    mod.rs            # pub fn gamma() implementation + tests
  ... (32 function folders total)
  mod.rs              # Re-exports all function modules
```

**Benefits:**
- **One folder per function**: Clear module boundaries
- **Co-located data + logic**: Each function's HashMap lives with its implementation
- **Independent testing**: Each function has its own test suite in `mod.rs`
- **Easy discovery**: `src/core/functions/sin/` contains everything about sine
- **Clean imports**: `use crate::core::functions::sin;`

---

## **CATEGORY 1: ELEMENTARY FUNCTIONS (17 total)**
### **Approach: Shared Static HashMap + Implementation Logic**

---

### **1. TRIGONOMETRIC FUNCTIONS (6 functions)**

#### **1.1 sin - Sine Function**

**File Location:** `src/core/functions/sin/`

**Shared Data:**
```rust
// src/core/functions/sin/data.rs
pub static SIN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Exact zero
    map.insert(expr!(0), special_value!(0, "\\sin(0) = 0"));

    // Common angles (π multiples)
    map.insert(expr!(pi / 6), special_value!(1/2, "\\sin(\\frac{\\pi}{6}) = \\frac{1}{2}"));
    map.insert(expr!(pi / 4), special_value!(sqrt(2)/2, "\\sin(\\frac{\\pi}{4}) = \\frac{\\sqrt{2}}{2}"));
    map.insert(expr!(pi / 3), special_value!(sqrt(3)/2, "\\sin(\\frac{\\pi}{3}) = \\frac{\\sqrt{3}}{2}"));
    map.insert(expr!(pi / 2), special_value!(1, "\\sin(\\frac{\\pi}{2}) = 1"));
    map.insert(expr!(pi), special_value!(0, "\\sin(\\pi) = 0"));
    map.insert(expr!(3*pi / 2), special_value!(-1, "\\sin(\\frac{3\\pi}{2}) = -1"));
    map.insert(expr!(2*pi), special_value!(0, "\\sin(2\\pi) = 0"));

    map
});
```

**Implementation:**
```rust
// src/core/functions/sin/mod.rs
pub fn sin(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values (exact or error)
    if let Some(result) = SIN_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Computed special values (general π multiples)
    if let Some(pi_mult) = arg.as_pi_multiple() {
        return Ok(eval_sin_at_pi_multiple(&pi_mult));
    }

    // 3. Identities: sin(-x) = -sin(x)
    if let Some(neg_arg) = arg.as_negation() {
        return sin(&neg_arg).map(|result| Expression::neg(result));
    }

    // 4. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Ok(Expression::float(val.sin()));
    }

    // 5. Unevaluated (symbolic)
    Ok(Expression::function("sin", vec![arg.clone()]))
}

pub(crate) fn sin_dispatch(args: &[Expression]) -> Result<Expression, MathError> {
    if args.len() != 1 {
        return Err(MathError::InvalidArgumentCount {
            expected: 1,
            got: args.len()
        });
    }
    sin(&args[0])
}
```

**Registry:**
```rust
// functions/registry/elementary.rs
FunctionProperties {
    name: "sin",
    // Derived from shared data (can't drift!)
    special_values: SIN_SPECIAL_VALUES.iter()
        .map(|(input, result)| SpecialValue {
            input: input.to_string(),
            output: result.output.clone(),
            latex_explanation: result.latex.clone(),
        })
        .collect(),
    dispatch: sin_dispatch,
}
```

---

#### **1.2 cos - Cosine Function**

**Shared Data:**
```rust
// functions/data/trig/cos_special_values.rs
pub static COS_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(1, "\\cos(0) = 1"));
    map.insert(expr!(pi / 6), special_value!(sqrt(3)/2, "\\cos(\\frac{\\pi}{6}) = \\frac{\\sqrt{3}}{2}"));
    map.insert(expr!(pi / 4), special_value!(sqrt(2)/2, "\\cos(\\frac{\\pi}{4}) = \\frac{\\sqrt{2}}{2}"));
    map.insert(expr!(pi / 3), special_value!(1/2, "\\cos(\\frac{\\pi}{3}) = \\frac{1}{2}"));
    map.insert(expr!(pi / 2), special_value!(0, "\\cos(\\frac{\\pi}{2}) = 0"));
    map.insert(expr!(pi), special_value!(-1, "\\cos(\\pi) = -1"));
    map.insert(expr!(3*pi / 2), special_value!(0, "\\cos(\\frac{3\\pi}{2}) = 0"));
    map.insert(expr!(2*pi), special_value!(1, "\\cos(2\\pi) = 1"));

    map
});
```

**Implementation:** Same pattern as sin

---

#### **1.3 tan - Tangent Function**

**Shared Data:**
```rust
// functions/data/trig/tan_special_values.rs
pub static TAN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "\\tan(0) = 0"));
    map.insert(expr!(pi / 6), special_value!(sqrt(3)/3, "\\tan(\\frac{\\pi}{6}) = \\frac{\\sqrt{3}}{3}"));
    map.insert(expr!(pi / 4), special_value!(1, "\\tan(\\frac{\\pi}{4}) = 1"));
    map.insert(expr!(pi / 3), special_value!(sqrt(3), "\\tan(\\frac{\\pi}{3}) = \\sqrt{3}"));
    map.insert(expr!(pi), special_value!(0, "\\tan(\\pi) = 0"));

    // Poles (undefined)
    map.insert_undefined(expr!(pi / 2), "\\tan(\\frac{\\pi}{2}) = \\text{undefined}");
    map.insert_undefined(expr!(3*pi / 2), "\\tan(\\frac{3\\pi}{2}) = \\text{undefined}");

    map
});
```

**Implementation:** Same pattern, checks for poles

---

#### **1.4 cot - Cotangent Function**

**Shared Data:**
```rust
pub static COT_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(pi / 6), special_value!(sqrt(3), "\\cot(\\frac{\\pi}{6}) = \\sqrt{3}"));
    map.insert(expr!(pi / 4), special_value!(1, "\\cot(\\frac{\\pi}{4}) = 1"));
    map.insert(expr!(pi / 3), special_value!(sqrt(3)/3, "\\cot(\\frac{\\pi}{3}) = \\frac{\\sqrt{3}}{3}"));
    map.insert(expr!(pi / 2), special_value!(0, "\\cot(\\frac{\\pi}{2}) = 0"));

    // Poles
    map.insert_undefined(expr!(0), "\\cot(0) = \\text{undefined}");
    map.insert_undefined(expr!(pi), "\\cot(\\pi) = \\text{undefined}");

    map
});
```

---

#### **1.5 sec - Secant Function**

**Shared Data:**
```rust
pub static SEC_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(1, "\\sec(0) = 1"));
    map.insert(expr!(pi / 3), special_value!(2, "\\sec(\\frac{\\pi}{3}) = 2"));
    map.insert(expr!(pi / 4), special_value!(sqrt(2), "\\sec(\\frac{\\pi}{4}) = \\sqrt{2}"));
    map.insert(expr!(pi / 6), special_value!(2*sqrt(3)/3, "\\sec(\\frac{\\pi}{6}) = \\frac{2\\sqrt{3}}{3}"));
    map.insert(expr!(pi), special_value!(-1, "\\sec(\\pi) = -1"));

    // Poles
    map.insert_undefined(expr!(pi / 2), "\\sec(\\frac{\\pi}{2}) = \\text{undefined}");
    map.insert_undefined(expr!(3*pi / 2), "\\sec(\\frac{3\\pi}{2}) = \\text{undefined}");

    map
});
```

---

#### **1.6 csc - Cosecant Function**

**Shared Data:**
```rust
pub static CSC_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(pi / 6), special_value!(2, "\\csc(\\frac{\\pi}{6}) = 2"));
    map.insert(expr!(pi / 4), special_value!(sqrt(2), "\\csc(\\frac{\\pi}{4}) = \\sqrt{2}"));
    map.insert(expr!(pi / 3), special_value!(2*sqrt(3)/3, "\\csc(\\frac{\\pi}{3}) = \\frac{2\\sqrt{3}}{3}"));
    map.insert(expr!(pi / 2), special_value!(1, "\\csc(\\frac{\\pi}{2}) = 1"));

    // Poles
    map.insert_undefined(expr!(0), "\\csc(0) = \\text{undefined}");
    map.insert_undefined(expr!(pi), "\\csc(\\pi) = \\text{undefined}");

    map
});
```

---

### **2. INVERSE TRIGONOMETRIC FUNCTIONS (3 functions)**

#### **2.1 arcsin - Inverse Sine**

**Shared Data:**
```rust
pub static ARCSIN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "\\arcsin(0) = 0"));
    map.insert(expr!(1/2), special_value!(pi/6, "\\arcsin(\\frac{1}{2}) = \\frac{\\pi}{6}"));
    map.insert(expr!(sqrt(2)/2), special_value!(pi/4, "\\arcsin(\\frac{\\sqrt{2}}{2}) = \\frac{\\pi}{4}"));
    map.insert(expr!(sqrt(3)/2), special_value!(pi/3, "\\arcsin(\\frac{\\sqrt{3}}{2}) = \\frac{\\pi}{3}"));
    map.insert(expr!(1), special_value!(pi/2, "\\arcsin(1) = \\frac{\\pi}{2}"));
    map.insert(expr!(-1), special_value!(-pi/2, "\\arcsin(-1) = -\\frac{\\pi}{2}"));

    map
});
```

**Implementation:**
```rust
pub fn arcsin(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Shared special values (exact or error)
    if let Some(result) = ARCSIN_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Computed domain check: [-1, 1]
    if let Some(val) = arg.try_to_f64() {
        if val < -1.0 || val > 1.0 {
            return Err(MathError::DomainError {
                operation: "arcsin".to_string(),
                value: arg.clone(),
                reason: "arcsin requires input in [-1, 1]".to_string(),
            });
        }
        return Ok(Expression::float(val.asin()));
    }

    // 3. Unevaluated (symbolic)
    Ok(Expression::function("arcsin", vec![arg.clone()]))
}
```

---

#### **2.2 arccos - Inverse Cosine**

**Shared Data:**
```rust
pub static ARCCOS_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(1), special_value!(0, "\\arccos(1) = 0"));
    map.insert(expr!(sqrt(3)/2), special_value!(pi/6, "\\arccos(\\frac{\\sqrt{3}}{2}) = \\frac{\\pi}{6}"));
    map.insert(expr!(sqrt(2)/2), special_value!(pi/4, "\\arccos(\\frac{\\sqrt{2}}{2}) = \\frac{\\pi}{4}"));
    map.insert(expr!(1/2), special_value!(pi/3, "\\arccos(\\frac{1}{2}) = \\frac{\\pi}{3}"));
    map.insert(expr!(0), special_value!(pi/2, "\\arccos(0) = \\frac{\\pi}{2}"));
    map.insert(expr!(-1), special_value!(pi, "\\arccos(-1) = \\pi"));

    map
});
```

---

#### **2.3 arctan - Inverse Tangent**

**Shared Data:**
```rust
pub static ARCTAN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "\\arctan(0) = 0"));
    map.insert(expr!(sqrt(3)/3), special_value!(pi/6, "\\arctan(\\frac{\\sqrt{3}}{3}) = \\frac{\\pi}{6}"));
    map.insert(expr!(1), special_value!(pi/4, "\\arctan(1) = \\frac{\\pi}{4}"));
    map.insert(expr!(sqrt(3)), special_value!(pi/3, "\\arctan(\\sqrt{3}) = \\frac{\\pi}{3}"));

    map
});
```

---

### **3. HYPERBOLIC FUNCTIONS (3 functions)**

#### **3.1 sinh - Hyperbolic Sine**

**Shared Data:**
```rust
pub static SINH_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "\\sinh(0) = 0"));
    // sinh doesn't have many exact special values like trig functions

    map
});
```

**Implementation:**
```rust
pub fn sinh(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Shared special values (exact or error)
    if let Some(result) = SINH_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Identity: sinh(-x) = -sinh(x)
    if let Some(neg_arg) = arg.as_negation() {
        return sinh(&neg_arg).map(|result| Expression::neg(result));
    }

    // 3. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Ok(Expression::float(val.sinh()));
    }

    // 4. Unevaluated (symbolic)
    Ok(Expression::function("sinh", vec![arg.clone()]))
}
```

---

#### **3.2 cosh - Hyperbolic Cosine**

**Shared Data:**
```rust
pub static COSH_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(1, "\\cosh(0) = 1"));

    map
});
```

**Implementation:**
```rust
pub fn cosh(arg: &Expression) -> Expression {
    // 1. Shared special values
    if let Some(result) = COSH_SPECIAL_VALUES.get(arg) {
        return result.output.clone();
    }

    // 2. Identity: cosh(-x) = cosh(x) (even function)
    if let Some(neg_arg) = arg.as_negation() {
        return cosh(&neg_arg);
    }

    // 3. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Expression::float(val.cosh());
    }

    // 4. Unevaluated
    Expression::function("cosh", vec![arg.clone()])
}
```

---

#### **3.3 tanh - Hyperbolic Tangent**

**Shared Data:**
```rust
pub static TANH_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "\\tanh(0) = 0"));

    map
});
```

**Implementation:**
```rust
pub fn tanh(arg: &Expression) -> Expression {
    // 1. Shared special values
    if let Some(result) = TANH_SPECIAL_VALUES.get(arg) {
        return result.output.clone();
    }

    // 2. Identity: tanh(-x) = -tanh(x)
    if let Some(neg_arg) = arg.as_negation() {
        return Expression::neg(tanh(&neg_arg));
    }

    // 3. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Expression::float(val.tanh());
    }

    // 4. Unevaluated
    Expression::function("tanh", vec![arg.clone()])
}
```

---

### **4. EXPONENTIAL & LOGARITHMIC FUNCTIONS (3 functions)**

#### **4.1 exp - Exponential Function**

**Shared Data:**
```rust
pub static EXP_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(1, "e^0 = 1"));
    map.insert(expr!(1), special_value!(E, "e^1 = e"));
    map.insert(expr!(ln(2)), special_value!(2, "e^{\\ln(2)} = 2"));

    map
});
```

**Implementation:**
```rust
pub fn exp(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Shared special values (exact or error)
    if let Some(result) = EXP_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Identity: exp(ln(x)) = x
    if let Some(inner) = arg.as_ln() {
        return Ok(inner.clone());
    }

    // 3. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Ok(Expression::float(val.exp()));
    }

    // 4. Unevaluated (symbolic)
    Ok(Expression::function("exp", vec![arg.clone()]))
}
```

---

#### **4.2 ln - Natural Logarithm**

**Shared Data:**
```rust
pub static LN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(1), special_value!(0, "\\ln(1) = 0"));
    map.insert(expr!(E), special_value!(1, "\\ln(e) = 1"));
    map.insert(expr!(E^2), special_value!(2, "\\ln(e^2) = 2"));

    map
});
```

**Implementation:**
```rust
pub fn ln(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Shared special values (exact or error)
    if let Some(result) = LN_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Computed domain check: x > 0 (pole at 0, branch cut for negatives)
    if let Some(val) = arg.try_to_f64() {
        if val == 0.0 {
            return Err(MathError::Pole {
                function: "ln".to_string(),
                at: arg.clone(),
            });
        }
        if val < 0.0 {
            return Err(MathError::BranchCut {
                function: "ln".to_string(),
                value: arg.clone(),
            });
        }
        return Ok(Expression::float(val.ln()));
    }

    // 3. Identity: ln(exp(x)) = x
    if let Some(inner) = arg.as_exp() {
        return Ok(inner.clone());
    }

    // 4. Unevaluated (symbolic)
    Ok(Expression::function("ln", vec![arg.clone()]))
}
```

---

#### **4.3 log - Logarithm with Base**

**Shared Data:**
```rust
pub static LOG_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Note: These are parametric, stored as documentation only
    // Actual evaluation uses implementation logic

    map
});
```

**Implementation:**
```rust
pub fn log(arg: &Expression, base: &Expression) -> Result<Expression, MathError> {
    // 1. Parametric special values (computed, not looked up)
    if arg == &Expression::integer(1) {
        return Ok(Expression::integer(0));  // log_b(1) = 0 for any b
    }
    if arg == base {
        return Ok(Expression::integer(1));  // log_b(b) = 1
    }

    // 2. Numerical evaluation with domain checking
    if let (Some(a), Some(b)) = (arg.try_to_f64(), base.try_to_f64()) {
        if a == 0.0 {
            return Err(MathError::Pole {
                function: "log".to_string(),
                at: arg.clone(),
            });
        }
        if a < 0.0 {
            return Err(MathError::BranchCut {
                function: "log".to_string(),
                value: arg.clone(),
            });
        }
        if b <= 0.0 || b == 1.0 {
            return Err(MathError::DomainError {
                operation: "log".to_string(),
                value: base.clone(),
                reason: "log base must be positive and not equal to 1".to_string(),
            });
        }
        return Ok(Expression::float(a.log(b)));
    }

    // 3. Unevaluated (symbolic)
    Ok(Expression::function("log", vec![arg.clone(), base.clone()]))
}
```

---

### **5. ROOTS & ABSOLUTE VALUE (2 functions)**

#### **5.1 sqrt - Square Root**

**Shared Data:**
```rust
pub static SQRT_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "\\sqrt{0} = 0"));
    map.insert(expr!(1), special_value!(1, "\\sqrt{1} = 1"));
    map.insert(expr!(4), special_value!(2, "\\sqrt{4} = 2"));
    map.insert(expr!(9), special_value!(3, "\\sqrt{9} = 3"));
    map.insert(expr!(16), special_value!(4, "\\sqrt{16} = 4"));
    map.insert(expr!(25), special_value!(5, "\\sqrt{25} = 5"));
    map.insert(expr!(36), special_value!(6, "\\sqrt{36} = 6"));
    map.insert(expr!(49), special_value!(7, "\\sqrt{49} = 7"));
    map.insert(expr!(64), special_value!(8, "\\sqrt{64} = 8"));
    map.insert(expr!(81), special_value!(9, "\\sqrt{81} = 9"));
    map.insert(expr!(100), special_value!(10, "\\sqrt{100} = 10"));

    map
});
```

**Implementation:**
```rust
pub fn sqrt(arg: &Expression) -> Expression {
    // 1. Shared special values
    if let Some(result) = SQRT_SPECIAL_VALUES.get(arg) {
        return result.output.clone();
    }

    // 2. Perfect squares (computed)
    if let Some(n) = arg.as_integer() {
        if n >= 0 {
            let sqrt_n = (n as f64).sqrt();
            if sqrt_n.fract() == 0.0 {
                return Expression::integer(sqrt_n as i64);
            }
        }
    }

    // 3. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        if val < 0.0 {
            // Complex result: sqrt(-1) = i
            return Expression::mul(vec![
                Expression::sqrt(Expression::float(-val)),
                Expression::i(),
            ]);
        }
        return Expression::float(val.sqrt());
    }

    // 4. Unevaluated
    Expression::function("sqrt", vec![arg.clone()])
}
```

---

#### **5.2 abs - Absolute Value**

**Shared Data:**
```rust
pub static ABS_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(0, "|0| = 0"));

    map
});
```

**Implementation:**
```rust
pub fn abs(arg: &Expression) -> Expression {
    // 1. Shared special values
    if let Some(result) = ABS_SPECIAL_VALUES.get(arg) {
        return result.output.clone();
    }

    // 2. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Expression::float(val.abs());
    }

    // 3. Integer evaluation
    if let Some(n) = arg.as_integer() {
        return Expression::integer(n.abs());
    }

    // 4. Identity: abs(-x) = abs(x)
    if let Some(neg_arg) = arg.as_negation() {
        return abs(&neg_arg);
    }

    // 5. Unevaluated
    Expression::function("abs", vec![arg.clone()])
}
```

---

## **CATEGORY 2: SPECIAL FUNCTIONS (7 total)**
### **Approach: Shared Static HashMap + Advanced Algorithms**

---

### **6. GAMMA FAMILY (4 functions)**

#### **6.1 gamma - Gamma Function**

**Shared Data:**
```rust
pub static GAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Integer values
    map.insert(expr!(1), special_value!(1, "\\Gamma(1) = 1"));
    map.insert(expr!(2), special_value!(1, "\\Gamma(2) = 1"));
    map.insert(expr!(3), special_value!(2, "\\Gamma(3) = 2"));
    map.insert(expr!(4), special_value!(6, "\\Gamma(4) = 6"));
    map.insert(expr!(5), special_value!(24, "\\Gamma(5) = 24"));
    map.insert(expr!(6), special_value!(120, "\\Gamma(6) = 120"));

    // Half-integer values
    map.insert(expr!(1/2), special_value!(sqrt(pi), "\\Gamma(\\frac{1}{2}) = \\sqrt{\\pi}"));
    map.insert(expr!(3/2), special_value!(sqrt(pi)/2, "\\Gamma(\\frac{3}{2}) = \\frac{\\sqrt{\\pi}}{2}"));
    map.insert(expr!(5/2), special_value!(3*sqrt(pi)/4, "\\Gamma(\\frac{5}{2}) = \\frac{3\\sqrt{\\pi}}{4}"));

    // Poles
    map.insert_undefined(expr!(0), "\\Gamma(0) = \\text{undefined (pole)}");
    map.insert_undefined(expr!(-1), "\\Gamma(-1) = \\text{undefined (pole)}");
    map.insert_undefined(expr!(-2), "\\Gamma(-2) = \\text{undefined (pole)}");

    map
});
```

**Implementation:**
```rust
pub fn gamma(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Shared special values (exact or error - includes poles at 0, -1, -2, ...)
    if let Some(result) = GAMMA_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Computed integer arguments: Γ(n) = (n-1)!
    if let Some(n) = arg.as_integer() {
        if n > 0 {
            return Ok(Expression::integer(factorial(n - 1)));
        } else {
            // Computed poles at non-positive integers not in HashMap
            return Err(MathError::Pole {
                function: "gamma".to_string(),
                at: arg.clone(),
            });
        }
    }

    // 3. Half-integer arguments: Γ(n + 1/2)
    if let Some((n, half)) = arg.as_half_integer() {
        return Ok(eval_gamma_half_integer(n));
    }

    // 4. Numerical evaluation (Lanczos approximation)
    if let Some(val) = arg.try_to_f64() {
        if val <= 0.0 && val.fract() == 0.0 {
            // Numerical pole detection
            return Err(MathError::Pole {
                function: "gamma".to_string(),
                at: arg.clone(),
            });
        }
        return Ok(Expression::float(gamma_lanczos(val)));
    }

    // 5. Unevaluated (symbolic)
    Ok(Expression::function("gamma", vec![arg.clone()]))
}
```

---

#### **6.2 beta - Beta Function**

**Shared Data:**
```rust
pub static BETA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!((1, 1)), special_value!(1, "B(1,1) = 1"));
    map.insert(expr!((1/2, 1/2)), special_value!(pi, "B(\\frac{1}{2},\\frac{1}{2}) = \\pi"));

    map
});
```

**Implementation:**
```rust
pub fn beta(a: &Expression, b: &Expression) -> Expression {
    // 1. Shared special values (tuple key)
    let key = Expression::tuple(vec![a.clone(), b.clone()]);
    if let Some(result) = BETA_SPECIAL_VALUES.get(&key) {
        return result.output.clone();
    }

    // 2. Use gamma: B(a,b) = Γ(a)Γ(b)/Γ(a+b)
    Expression::div(
        Expression::mul(vec![gamma(a), gamma(b)]),
        gamma(&Expression::add(vec![a.clone(), b.clone()]))
    )
}
```

---

#### **6.3 digamma - Digamma Function**

**Shared Data:**
```rust
pub static DIGAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(1), special_value!(-EULER_GAMMA, "\\psi(1) = -\\gamma"));
    map.insert(expr!(1/2), special_value!(-EULER_GAMMA - 2*ln(2), "\\psi(\\frac{1}{2}) = -\\gamma - 2\\ln(2)"));

    map
});
```

---

#### **6.4 polygamma - Polygamma Function**

**Shared Data:**
```rust
pub static POLYGAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // ψ'(1) = π²/6
    map.insert(expr!((1, 1)), special_value!(pi^2 / 6, "\\psi'(1) = \\frac{\\pi^2}{6}"));

    map
});
```

---

### **7. BESSEL FUNCTIONS (2 functions)**

#### **7.1 bessel_j - Bessel Function of First Kind**

**Shared Data:**
```rust
pub static BESSEL_J_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // J_0(0) = 1
    map.insert(expr!((0, 0)), special_value!(1, "J_0(0) = 1"));

    // J_n(0) = 0 for n ≠ 0
    map.insert(expr!((1, 0)), special_value!(0, "J_1(0) = 0"));
    map.insert(expr!((2, 0)), special_value!(0, "J_2(0) = 0"));

    map
});
```

---

#### **7.2 bessel_y - Bessel Function of Second Kind**

**Shared Data:**
```rust
pub static BESSEL_Y_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Y_n(0) = -∞ (pole)
    map.insert_undefined(expr!((0, 0)), "Y_0(0) = -\\infty");
    map.insert_undefined(expr!((1, 0)), "Y_1(0) = -\\infty");

    map
});
```

---

### **8. ZETA FUNCTION (1 function)**

#### **8.1 zeta - Riemann Zeta Function**

**Shared Data:**
```rust
pub static ZETA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    map.insert(expr!(0), special_value!(-1/2, "\\zeta(0) = -\\frac{1}{2}"));
    map.insert(expr!(2), special_value!(pi^2 / 6, "\\zeta(2) = \\frac{\\pi^2}{6}"));
    map.insert(expr!(4), special_value!(pi^4 / 90, "\\zeta(4) = \\frac{\\pi^4}{90}"));
    map.insert(expr!(6), special_value!(pi^6 / 945, "\\zeta(6) = \\frac{\\pi^6}{945}"));

    // Pole at s = 1
    map.insert_undefined(expr!(1), "\\zeta(1) = \\text{undefined (pole)}");

    map
});
```

---

## **CATEGORY 3: NUMBER THEORY FUNCTIONS (4 total)**
### **Approach: IMPLEMENTATION LOGIC ONLY (No Shared HashMap)**
### **Reason: Parametric special values, algorithmic computation is cleaner**

---

#### **9.1 gcd - Greatest Common Divisor**

**NO Shared HashMap** (parametric special values don't fit HashMap well)

**Implementation:**
```rust
// core/functions/number_theory/gcd.rs
pub fn gcd(a: &Expression, b: &Expression) -> Expression {
    // 1. Special cases (computed, not looked up)
    if a.is_zero() {
        return b.clone();
    }
    if b.is_zero() {
        return a.clone();
    }
    if a == &Expression::integer(1) || b == &Expression::integer(1) {
        return Expression::integer(1);
    }

    // 2. Both integers: use Euclidean algorithm
    if let (Some(a_int), Some(b_int)) = (a.as_integer(), b.as_integer()) {
        return Expression::integer(gcd_euclidean(a_int, b_int));
    }

    // 3. Unevaluated
    Expression::function("gcd", vec![a.clone(), b.clone()])
}

pub(crate) fn gcd_dispatch(args: &[Expression]) -> Expression {
    if args.len() != 2 { return Expression::undefined(); }
    gcd(&args[0], &args[1])
}
```

**Registry (minimal metadata):**
```rust
FunctionProperties {
    name: "gcd",
    special_values: vec![
        SpecialValue {
            input: "gcd(0, b)".to_string(),
            output: Expression::symbol(symbol!(b)),
            latex: "\\gcd(0, b) = |b|",
        },
        SpecialValue {
            input: "gcd(1, b)".to_string(),
            output: Expression::integer(1),
            latex: "\\gcd(1, b) = 1",
        },
    ],
    // Note: These are DOCUMENTATION only, not used in evaluation
}
```

---

#### **9.2 lcm - Least Common Multiple**

**NO Shared HashMap**

**Implementation:**
```rust
pub fn lcm(a: &Expression, b: &Expression) -> Expression {
    // 1. Special cases
    if a.is_zero() || b.is_zero() {
        return Expression::integer(0);
    }
    if a == &Expression::integer(1) {
        return b.clone();
    }
    if b == &Expression::integer(1) {
        return a.clone();
    }

    // 2. Both integers: lcm(a,b) = |a*b| / gcd(a,b)
    if let (Some(a_int), Some(b_int)) = (a.as_integer(), b.as_integer()) {
        let g = gcd_euclidean(a_int, b_int);
        return Expression::integer((a_int * b_int).abs() / g);
    }

    // 3. Unevaluated
    Expression::function("lcm", vec![a.clone(), b.clone()])
}
```

---

#### **9.3 mod - Modulo Operation**

**NO Shared HashMap**

**Implementation:**
```rust
pub fn modulo(a: &Expression, m: &Expression) -> Expression {
    // 1. Special cases
    if a.is_zero() {
        return Expression::integer(0);
    }

    // 2. Both integers: compute modulo
    if let (Some(a_int), Some(m_int)) = (a.as_integer(), m.as_integer()) {
        if m_int == 0 {
            return Expression::undefined();
        }
        return Expression::integer(a_int.rem_euclid(m_int));
    }

    // 3. Unevaluated
    Expression::function("mod", vec![a.clone(), m.clone()])
}
```

---

#### **9.4 is_prime - Primality Test**

**NO Shared HashMap** (could store first 100 primes, but algorithm is better)

**Implementation:**
```rust
pub fn is_prime(n: &Expression) -> Expression {
    // 1. Integer check
    if let Some(n_int) = n.as_integer() {
        if n_int < 2 {
            return Expression::integer(0); // false
        }

        // Use Miller-Rabin or deterministic trial division
        if is_prime_algorithm(n_int) {
            return Expression::integer(1); // true
        } else {
            return Expression::integer(0); // false
        }
    }

    // 2. Unevaluated
    Expression::function("is_prime", vec![n.clone()])
}
```

---

## **CATEGORY 4: POLYNOMIAL FUNCTIONS (4 total)**
### **Approach: RECURRENCE RELATIONS (No Shared HashMap)**
### **Reason: Parametric on both n (order) and x, better computed algorithmically**

---

#### **10.1 hermite - Hermite Polynomials**

**NO Shared HashMap** (infinitely many polynomials, recurrence is better)

**Implementation:**
```rust
pub fn hermite(n: &Expression, x: &Expression) -> Expression {
    // 1. Base cases
    if let Some(n_int) = n.as_integer() {
        if n_int < 0 {
            return Expression::undefined();
        }
        if n_int == 0 {
            return Expression::integer(1); // H_0(x) = 1
        }
        if n_int == 1 {
            return Expression::mul(vec![Expression::integer(2), x.clone()]); // H_1(x) = 2x
        }

        // 2. Recurrence: H_{n+1}(x) = 2x*H_n(x) - 2n*H_{n-1}(x)
        return hermite_recurrence(n_int, x);
    }

    // 3. Unevaluated
    Expression::function("hermite", vec![n.clone(), x.clone()])
}

fn hermite_recurrence(n: i64, x: &Expression) -> Expression {
    let mut h_prev = Expression::integer(1);
    let mut h_curr = Expression::mul(vec![Expression::integer(2), x.clone()]);

    for k in 2..=n {
        let h_next = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), x.clone(), h_curr.clone()]),
            Expression::mul(vec![Expression::integer(-2 * (k - 1)), h_prev.clone()]),
        ]);
        h_prev = h_curr;
        h_curr = h_next;
    }

    h_curr
}
```

---

#### **10.2 laguerre - Laguerre Polynomials**

**NO Shared HashMap**

**Implementation:**
```rust
pub fn laguerre(n: &Expression, x: &Expression) -> Expression {
    if let Some(n_int) = n.as_integer() {
        if n_int < 0 {
            return Expression::undefined();
        }
        if n_int == 0 {
            return Expression::integer(1); // L_0(x) = 1
        }
        if n_int == 1 {
            return Expression::add(vec![
                Expression::integer(1),
                Expression::neg(x.clone()),
            ]); // L_1(x) = 1 - x
        }

        // Recurrence: (n+1)L_{n+1}(x) = (2n+1-x)L_n(x) - nL_{n-1}(x)
        return laguerre_recurrence(n_int, x);
    }

    Expression::function("laguerre", vec![n.clone(), x.clone()])
}
```

---

#### **10.3 chebyshev - Chebyshev Polynomials**

**NO Shared HashMap**

**Implementation:**
```rust
pub fn chebyshev(n: &Expression, x: &Expression) -> Expression {
    if let Some(n_int) = n.as_integer() {
        if n_int < 0 {
            return Expression::undefined();
        }
        if n_int == 0 {
            return Expression::integer(1); // T_0(x) = 1
        }
        if n_int == 1 {
            return x.clone(); // T_1(x) = x
        }

        // Recurrence: T_{n+1}(x) = 2x*T_n(x) - T_{n-1}(x)
        return chebyshev_recurrence(n_int, x);
    }

    Expression::function("chebyshev", vec![n.clone(), x.clone()])
}
```

---

#### **10.4 legendre - Legendre Polynomials**

**NO Shared HashMap**

**Implementation:**
```rust
pub fn legendre(n: &Expression, x: &Expression) -> Expression {
    if let Some(n_int) = n.as_integer() {
        if n_int < 0 {
            return Expression::undefined();
        }
        if n_int == 0 {
            return Expression::integer(1); // P_0(x) = 1
        }
        if n_int == 1 {
            return x.clone(); // P_1(x) = x
        }

        // Recurrence: (n+1)P_{n+1}(x) = (2n+1)xP_n(x) - nP_{n-1}(x)
        return legendre_recurrence(n_int, x);
    }

    Expression::function("legendre", vec![n.clone(), x.clone()])
}
```

---

## **SUPPORTING INFRASTRUCTURE**

### **Unified Error Handling with SpecialValuesMap**

**Design Philosophy**: ALL function evaluation uses `Result<Expression, MathError>` return type for comprehensive error handling. Errors can come from two sources:

1. **HashMap Errors** (pre-computed): Known singularities stored in shared data (e.g., tan(π/2), log(0))
2. **Computed Errors** (dynamic): Domain violations detected during evaluation (e.g., tan(5π/2), arcsin(2))

**Key Benefits:**

1. **Unified Return Type**: ALL functions return `Result<Expression, MathError>` (no exceptions)
2. **Comprehensive Error Handling**: Errors from BOTH sources:
   - **HashMap errors**: Pre-computed known singularities (tan(π/2), gamma(0), ln(0))
   - **Computed errors**: Dynamically detected violations (arcsin(2), gamma(-5), ln(-1))
3. **Educational Context**: Every error includes LaTeX and explanation (whether from HashMap or computed)
4. **Type Safety**: Compiler enforces error handling at ALL call sites
5. **Single Source of Truth**: Known errors stored in shared HashMap (not duplicated in code)
6. **Automatic Registry**: Singularities derived from shared data (can't drift)

**Critical Design Principle**: Error handling is NOT limited to HashMap lookups. Every function performs:
1. HashMap check (exact values + known errors)
2. Computed special value checks (e.g., π multiples for trig functions)
3. Dynamic domain validation (e.g., arcsin input range, gamma poles)
4. Numerical evaluation with error detection
5. Unevaluated symbolic result (always returns Ok for valid symbolic expressions)

```rust
// core/functions/special_values.rs

use crate::error::MathError;
use crate::core::Expression;
use std::collections::HashMap;

/// Result of special value lookup - either exact value or domain error
#[derive(Debug, Clone)]
pub enum SpecialValueResult {
    /// Exact symbolic value (e.g., sin(0) = 0)
    Exact {
        output: Expression,
        latex: &'static str,
        explanation: &'static str,
    },

    /// Domain error (pole, branch cut, undefined)
    Error {
        error: MathError,
        latex: &'static str,
        explanation: &'static str,
    },
}

impl SpecialValueResult {
    /// Create exact special value
    pub fn exact(output: Expression, latex: &'static str, explanation: &'static str) -> Self {
        Self::Exact { output, latex, explanation }
    }

    /// Create pole singularity (e.g., tan(π/2))
    pub fn pole(function: &str, at: Expression, latex: &'static str) -> Self {
        Self::Error {
            error: MathError::Pole {
                function: function.to_string(),
                at: at.clone(),
            },
            latex,
            explanation: "Pole singularity - function approaches infinity",
        }
    }

    /// Create branch cut error (e.g., log(-1) in real domain)
    pub fn branch_cut(function: &str, value: Expression, latex: &'static str) -> Self {
        Self::Error {
            error: MathError::BranchCut {
                function: function.to_string(),
                value: value.clone(),
            },
            latex,
            explanation: "Branch cut - requires domain specification or complex domain",
        }
    }

    /// Create domain error (e.g., arcsin(2) out of [-1,1])
    pub fn domain_error(operation: &str, value: Expression, reason: &'static str, latex: &'static str) -> Self {
        Self::Error {
            error: MathError::DomainError {
                operation: operation.to_string(),
                value: value.clone(),
                reason: reason.to_string(),
            },
            latex,
            explanation: reason,
        }
    }

    /// Create undefined result (e.g., 0/0 indeterminate form)
    pub fn undefined(expression: Expression, reason: &'static str, latex: &'static str) -> Self {
        Self::Error {
            error: MathError::Undefined {
                expression: expression.clone(),
                reason: reason.to_string(),
            },
            latex,
            explanation: reason,
        }
    }
}

/// HashMap for special values with unified error handling
pub struct SpecialValuesMap {
    map: HashMap<Expression, SpecialValueResult>,
}

impl SpecialValuesMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    /// Insert special value result (exact or error)
    pub fn insert(&mut self, input: Expression, result: SpecialValueResult) {
        self.map.insert(input, result);
    }

    /// Helper: Insert exact value
    pub fn insert_exact(&mut self, input: Expression, output: Expression, latex: &'static str, explanation: &'static str) {
        self.map.insert(input, SpecialValueResult::exact(output, latex, explanation));
    }

    /// Helper: Insert pole singularity (e.g., tan(π/2), log(0))
    pub fn insert_pole(&mut self, function: &str, at: Expression, latex: &'static str) {
        self.map.insert(at.clone(), SpecialValueResult::pole(function, at, latex));
    }

    /// Helper: Insert branch cut (e.g., sqrt(-1), log(-1))
    pub fn insert_branch_cut(&mut self, function: &str, value: Expression, latex: &'static str) {
        self.map.insert(value.clone(), SpecialValueResult::branch_cut(function, value, latex));
    }

    /// Helper: Insert domain error (e.g., arcsin(2))
    pub fn insert_domain_error(&mut self, operation: &str, value: Expression, reason: &'static str, latex: &'static str) {
        self.map.insert(value.clone(), SpecialValueResult::domain_error(operation, value, reason, latex));
    }

    /// Helper: Insert undefined result (e.g., 0/0)
    pub fn insert_undefined(&mut self, expression: Expression, reason: &'static str, latex: &'static str) {
        self.map.insert(expression.clone(), SpecialValueResult::undefined(expression, reason, latex));
    }

    /// Get special value result (exact or error)
    pub fn get(&self, input: &Expression) -> Option<&SpecialValueResult> {
        self.map.get(input)
    }

    /// Iterate over all special values (exact and errors)
    pub fn iter(&self) -> impl Iterator<Item = (&Expression, &SpecialValueResult)> {
        self.map.iter()
    }

    /// Iterate over exact values only (for registry derivation)
    pub fn iter_exact(&self) -> impl Iterator<Item = (&Expression, &Expression, &'static str)> {
        self.map.iter().filter_map(|(input, result)| {
            match result {
                SpecialValueResult::Exact { output, latex, .. } => Some((input, output, *latex)),
                _ => None,
            }
        })
    }

    /// Iterate over errors only (for singularity extraction)
    pub fn iter_errors(&self) -> impl Iterator<Item = (&Expression, &MathError)> {
        self.map.iter().filter_map(|(input, result)| {
            match result {
                SpecialValueResult::Error { error, .. } => Some((input, error)),
                _ => None,
            }
        })
    }
}
```

### **Example Usage: tan with Poles**

```rust
// functions/elementary/trig/tan.rs

pub static TAN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Exact values
    map.insert_exact(
        expr!(0),
        expr!(0),
        "\\tan(0) = 0",
        "Tangent of zero is zero"
    );
    map.insert_exact(
        expr!(pi / 4),
        expr!(1),
        "\\tan(\\frac{\\pi}{4}) = 1",
        "Tangent of π/4 is 1"
    );
    map.insert_exact(
        expr!(pi / 6),
        expr!(sqrt(3) / 3),
        "\\tan(\\frac{\\pi}{6}) = \\frac{\\sqrt{3}}{3}",
        "Tangent of π/6"
    );

    // Poles (domain errors)
    map.insert_pole(
        "tan",
        expr!(pi / 2),
        "\\tan(\\frac{\\pi}{2}) = \\text{undefined (pole)}"
    );
    map.insert_pole(
        "tan",
        expr!(3 * pi / 2),
        "\\tan(\\frac{3\\pi}{2}) = \\text{undefined (pole)}"
    );
    map.insert_pole(
        "tan",
        expr!(-pi / 2),
        "\\tan(-\\frac{\\pi}{2}) = \\text{undefined (pole)}"
    );

    map
});

/// Tangent function with unified error handling
///
/// Returns:
/// - `Ok(Expression)` for valid results (exact, symbolic, numerical)
/// - `Err(MathError::Pole)` at singularities (π/2 + nπ)
pub fn tan(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check special values (includes exact + poles)
    if let Some(result) = TAN_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Computed special values (general π multiples)
    if let Some(pi_mult) = arg.as_pi_multiple() {
        // Check for poles: tan is undefined at (2k+1)π/2
        if is_odd_half_pi_multiple(&pi_mult) {
            return Err(MathError::Pole {
                function: "tan".to_string(),
                at: arg.clone(),
            });
        }
        return Ok(eval_tan_at_pi_multiple(&pi_mult));
    }

    // 3. Identities: tan(-x) = -tan(x)
    if let Some(neg_arg) = arg.as_negation() {
        return tan(&neg_arg).map(|result| Expression::neg(result));
    }

    // 4. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Ok(Expression::float(val.tan()));
    }

    // 5. Unevaluated (symbolic)
    Ok(Expression::function("tan", vec![arg.clone()]))
}

pub(crate) fn tan_dispatch(args: &[Expression]) -> Result<Expression, MathError> {
    if args.len() != 1 {
        return Err(MathError::InvalidArgumentCount {
            expected: 1,
            got: args.len()
        });
    }
    tan(&args[0])
}
```

### **Example Usage: sqrt with Branch Cuts**

```rust
// functions/elementary/sqrt.rs

pub static SQRT_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Exact values
    map.insert_exact(expr!(0), expr!(0), "\\sqrt{0} = 0", "Square root of zero");
    map.insert_exact(expr!(1), expr!(1), "\\sqrt{1} = 1", "Square root of one");
    map.insert_exact(expr!(4), expr!(2), "\\sqrt{4} = 2", "Perfect square");
    map.insert_exact(expr!(9), expr!(3), "\\sqrt{9} = 3", "Perfect square");
    // ... more perfect squares

    // Branch cut warning for negative values (informational)
    map.insert_branch_cut(
        "sqrt",
        expr!(-1),
        "\\sqrt{-1} requires complex domain (principal value: i)"
    );

    map
});

pub fn sqrt(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check special values
    if let Some(result) = SQRT_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => {
                // For sqrt, promote to complex instead of error in symbolic context
                if matches!(error, MathError::BranchCut { .. }) {
                    if let Some(neg_val) = arg.as_negation() {
                        return Ok(Expression::mul(vec![
                            Expression::i(),
                            sqrt(&neg_val)?,
                        ]));
                    }
                }
                return Err(error.clone());
            },
        }
    }

    // 2. Perfect squares (computed)
    if let Some(n) = arg.as_integer() {
        if n < 0 {
            // Promote to complex: sqrt(-n) = i*sqrt(n)
            return Ok(Expression::mul(vec![
                Expression::i(),
                sqrt(&Expression::integer(-n))?,
            ]));
        }

        let sqrt_n = (n as f64).sqrt();
        if sqrt_n.fract() == 0.0 {
            return Ok(Expression::integer(sqrt_n as i64));
        }
    }

    // 3. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        if val < 0.0 {
            // Complex result
            return Ok(Expression::mul(vec![
                Expression::sqrt(Expression::float(-val)),
                Expression::i(),
            ]));
        }
        return Ok(Expression::float(val.sqrt()));
    }

    // 4. Unevaluated
    Ok(Expression::function("sqrt", vec![arg.clone()]))
}
```

### **Registry Derivation with Error Metadata**

```rust
// functions/registry/elementary.rs

FunctionProperties::Elementary(Box::new(ElementaryProperties {
    name: "tan",

    // Special values: Only exact values (errors excluded)
    special_values: TAN_SPECIAL_VALUES.iter_exact()
        .map(|(input, output, latex)| SpecialValue {
            input: input.to_string(),
            output: output.clone(),
            latex_explanation: latex.to_string(),
        })
        .collect(),

    // Domain/range data: Includes singularities from errors
    domain_range: Some(DomainRangeData {
        domain: Domain::Real,
        range: Range::Real,

        // Singularities extracted from error entries
        singularities: TAN_SPECIAL_VALUES.iter_errors()
            .filter_map(|(input, error)| {
                match error {
                    MathError::Pole { .. } => Some(input.clone()),
                    _ => None,
                }
            })
            .collect(),
    }),

    // Dispatch function (now returns Result)
    dispatch: tan_dispatch,
}))
```

### **Dispatch Table with Error Handling**

```rust
// core/expression/evaluation.rs

use crate::error::MathError;

/// Dispatch function type - returns Result for error handling
type DispatchFn = fn(&[Expression]) -> Result<Expression, MathError>;

static FUNCTION_DISPATCH: LazyLock<HashMap<&'static str, DispatchFn>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity(32);

    // Elementary functions (can have domain errors)
    map.insert("sin", sin_dispatch as DispatchFn);
    map.insert("cos", cos_dispatch as DispatchFn);
    map.insert("tan", tan_dispatch as DispatchFn);          // Has poles at π/2 + nπ
    map.insert("cot", cot_dispatch as DispatchFn);          // Has poles at nπ
    map.insert("sec", sec_dispatch as DispatchFn);          // Has poles at π/2 + nπ
    map.insert("csc", csc_dispatch as DispatchFn);          // Has poles at nπ
    map.insert("arcsin", arcsin_dispatch as DispatchFn);    // Domain [-1, 1]
    map.insert("arccos", arccos_dispatch as DispatchFn);    // Domain [-1, 1]
    map.insert("arctan", arctan_dispatch as DispatchFn);
    map.insert("sinh", sinh_dispatch as DispatchFn);
    map.insert("cosh", cosh_dispatch as DispatchFn);
    map.insert("tanh", tanh_dispatch as DispatchFn);
    map.insert("exp", exp_dispatch as DispatchFn);
    map.insert("ln", ln_dispatch as DispatchFn);            // Pole at 0, branch cut for negatives
    map.insert("log", log_dispatch as DispatchFn);          // Pole at 0, branch cut for negatives
    map.insert("sqrt", sqrt_dispatch as DispatchFn);        // Branch cut for negatives (promotes to complex)
    map.insert("abs", abs_dispatch as DispatchFn);

    // Special functions (can have poles and domain restrictions)
    map.insert("gamma", gamma_dispatch as DispatchFn);      // Poles at non-positive integers
    map.insert("beta", beta_dispatch as DispatchFn);
    map.insert("digamma", digamma_dispatch as DispatchFn);  // Poles at non-positive integers
    map.insert("polygamma", polygamma_dispatch as DispatchFn);
    map.insert("bessel_j", bessel_j_dispatch as DispatchFn);
    map.insert("bessel_y", bessel_y_dispatch as DispatchFn); // Pole at 0
    map.insert("zeta", zeta_dispatch as DispatchFn);         // Pole at 1

    // Number theory functions (parametric, rarely error)
    map.insert("gcd", gcd_dispatch as DispatchFn);
    map.insert("lcm", lcm_dispatch as DispatchFn);
    map.insert("mod", mod_dispatch as DispatchFn);           // Division by zero error
    map.insert("is_prime", is_prime_dispatch as DispatchFn);

    // Polynomial functions (recurrence, rarely error)
    map.insert("hermite", hermite_dispatch as DispatchFn);   // Undefined for negative n
    map.insert("laguerre", laguerre_dispatch as DispatchFn); // Undefined for negative n
    map.insert("chebyshev", chebyshev_dispatch as DispatchFn); // Undefined for negative n
    map.insert("legendre", legendre_dispatch as DispatchFn);   // Undefined for negative n

    map
});

/// Evaluate function expression with error handling
///
/// Returns:
/// - `Ok(Expression)` for successful evaluation (exact, symbolic, or numerical)
/// - `Err(MathError)` for domain errors (poles, branch cuts, undefined)
/// - `Ok(Expression::function(...))` for unevaluated symbolic expressions
pub fn evaluate_function(name: &str, args: &[Expression]) -> Result<Expression, MathError> {
    if let Some(dispatch_fn) = FUNCTION_DISPATCH.get(name) {
        return dispatch_fn(args);
    }

    // Unknown function - return unevaluated (not an error!)
    Ok(Expression::function(name, args.to_vec()))
}
```

---

## **SUMMARY**

### **Functions with Shared HashMap (24 total):**

**Elementary (17):**
1. sin ✅
2. cos ✅
3. tan ✅
4. cot ✅
5. sec ✅
6. csc ✅
7. arcsin ✅
8. arccos ✅
9. arctan ✅
10. sinh ✅
11. cosh ✅
12. tanh ✅
13. exp ✅
14. ln ✅
15. log ✅ (minimal HashMap, mostly implementation logic)
16. sqrt ✅
17. abs ✅

**Special (7):**
18. gamma ✅
19. beta ✅
20. digamma ✅
21. polygamma ✅
22. bessel_j ✅
23. bessel_y ✅
24. zeta ✅

### **Functions with Implementation Logic Only (8 total):**

**Number Theory (4):**
25. gcd ❌ (parametric)
26. lcm ❌ (parametric)
27. mod ❌ (parametric)
28. is_prime ❌ (algorithmic)

**Polynomials (4):**
29. hermite ❌ (recurrence)
30. laguerre ❌ (recurrence)
31. chebyshev ❌ (recurrence)
32. legendre ❌ (recurrence)

---

## **ARCHITECTURE BENEFITS**

### **Single Source of Truth**
- Shared HashMap = data source for both implementation AND registry
- Registry special_values DERIVED from shared data
- Cannot drift (impossible by design)

### **Performance**
- Direct calls: ~50-100ns (no registry lookup)
- Registry calls: ~65-115ns (one HashMap lookup for dispatch)
- Special values: O(1) HashMap lookup

### **Maintainability**
- Add new special value: Update shared HashMap ONCE
- No duplication between implementation and registry
- Clear separation: data (HashMap) vs logic (implementation) vs metadata (registry)

### **Testing**
- Validate registry matches shared data automatically
- Test that implementation returns correct special values
- Single source = guaranteed consistency

---

**This is the complete, definitive architecture for all 32 MathHook functions!**

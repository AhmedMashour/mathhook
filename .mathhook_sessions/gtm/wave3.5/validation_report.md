# SymPy Comparison Validation Report

## Summary

- **Total Tests**: 11
- **Passed**: 3 (27.3%)
- **Failed**: 0 (0.0%)
- **Errors**: 8

## Performance Summary

- **Min Speedup**: 1043.10x
- **Max Speedup**: 11370.31x
- **Average Speedup**: 4844.30x
- **10-100x Claim**: NOT VALIDATED (avg: 4844.30x)

## Derivatives

- Tests: 8
- Passed: 0
- Failed: 0
- Errors: 8

### Test Cases

| Test | Correctness | SymPy Time | MathHook Time | Speedup |
|------|-------------|------------|---------------|----------|
| d/dx(x^2) | FAIL | 89792792ns | 0ns | 0.00x |
| d/dx(x^3) | FAIL | 6810625ns | 0ns | 0.00x |
| d/dx(sin(x)) | FAIL | 9242584ns | 0ns | 0.00x |
| d/dx(cos(x)) | FAIL | 6508750ns | 0ns | 0.00x |
| d/dx(exp(x)) | FAIL | 5548833ns | 0ns | 0.00x |
| d/dx(log(x)) | FAIL | 6268500ns | 0ns | 0.00x |
| d/dx(x*sin(x)) | FAIL | 16731375ns | 0ns | 0.00x |
| d/dx(x^2+2x+1) | FAIL | 13479084ns | 0ns | 0.00x |

### Failed Tests (Detail)

**d/dx(x^2)**

- Input: `x^2`
- SymPy: `2*x`
- MathHook: `ERROR: No output`

**d/dx(x^3)**

- Input: `x^3`
- SymPy: `3*x**2`
- MathHook: `ERROR: No output`

**d/dx(sin(x))**

- Input: `sin(x)`
- SymPy: `cos(x)`
- MathHook: `ERROR: No output`

**d/dx(cos(x))**

- Input: `cos(x)`
- SymPy: `-sin(x)`
- MathHook: `ERROR: No output`

**d/dx(exp(x))**

- Input: `exp(x)`
- SymPy: `exp(x)`
- MathHook: `ERROR: No output`

**d/dx(log(x))**

- Input: `log(x)`
- SymPy: `1/x`
- MathHook: `ERROR: No output`

**d/dx(x*sin(x))**

- Input: `x*sin(x)`
- SymPy: `x*cos(x) + sin(x)`
- MathHook: `ERROR: No output`

**d/dx(x^2+2x+1)**

- Input: `x^2 + 2*x + 1`
- SymPy: `2*x + 2`
- MathHook: `ERROR: No output`


## Simplify

- Tests: 3
- Passed: 3
- Failed: 0
- Errors: 0
- Average Speedup: 4844.30x

### Test Cases

| Test | Correctness | SymPy Time | MathHook Time | Speedup |
|------|-------------|------------|---------------|----------|
| simplify(x+x) | PASS | 238776459ns | 21000ns | 11370.31x |
| simplify(2x+3x) | PASS | 5828583ns | 2750ns | 2119.48x |
| simplify(x^2-x^2) | PASS | 2389750ns | 2291ns | 1043.10x |


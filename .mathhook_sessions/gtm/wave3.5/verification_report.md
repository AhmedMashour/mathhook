# SymPy Comparison Validation Report - Phase 2

## Summary

- **Total Tests**: 15
- **Passed**: 13 (86.7%)
- **Failed**: 2 (13.3%)
- **Errors**: 0

## Performance Summary

- **Min Speedup**: 19.86x
- **Max Speedup**: 1580.51x
- **Average Speedup**: 234.76x
- **10-100x Claim**: NOT VALIDATED (avg: 234.76x)

## Derivatives

- Tests: 15
- Passed: 13
- Failed: 2
- Errors: 0
- Average Speedup: 234.76x

### Test Cases

| Test | Correctness | SymPy Time | MathHook Time | Speedup |
|------|-------------|------------|---------------|----------|
| d/dx(x^2) | PASS | 42871417ns | 27125ns | 1580.51x |
| d/dx(x^3) | PASS | 3307917ns | 11167ns | 296.22x |
| d/dx(x^4) | PASS | 3445042ns | 19208ns | 179.35x |
| d/dx(sin(x)) | PASS | 1353417ns | 30084ns | 44.99x |
| d/dx(cos(x)) | PASS | 2443834ns | 27792ns | 87.93x |
| d/dx(tan(x)) | PASS | 4560875ns | 36500ns | 124.96x |
| d/dx(exp(x)) | PASS | 2487000ns | 36042ns | 69.00x |
| d/dx(log(x)) | FAIL | 1119792ns | 56375ns | 19.86x |
| d/dx(x*sin(x))_product_rule | PASS | 2639708ns | 53792ns | 49.07x |
| d/dx(sin(x)/x)_quotient_rule | PASS | 3451083ns | 102875ns | 33.55x |
| d/dx(sin(x^2))_chain_rule | PASS | 1966209ns | 66167ns | 29.72x |
| d/dx(x^2+2x+1) | PASS | 3315833ns | 13041ns | 254.26x |
| d/dx(x^2*exp(x)) | PASS | 5799625ns | 150292ns | 38.59x |
| d/dx(1/x) | PASS | 3826792ns | 5583ns | 685.44x |
| d/dx(sqrt(x)) | FAIL | 4761750ns | 170500ns | 27.93x |

### Failed Tests (Detail)

**d/dx(log(x))**

- Input: `log(x)`
- SymPy: `1/x`
- MathHook: `ln(Integer(10))^Integer(-1) / x`

**d/dx(sqrt(x))**

- Input: `sqrt(x)`
- SymPy: `1/(2*sqrt(x))`
- MathHook: `Rational(Ratio { numer: 1, denom: 2 }) * x^Rational(Ratio { numer: -1, denom: 2 })`



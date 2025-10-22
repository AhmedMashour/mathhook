# SymPy Comparison Validation Report - Phase 2

## Summary

- **Total Tests**: 15
- **Passed**: 13 (86.7%)
- **Failed**: 2 (13.3%)
- **Errors**: 0

## Performance Summary

- **Min Speedup**: 17.87x
- **Max Speedup**: 1580.47x
- **Average Speedup**: 179.05x
- **10-100x Claim**: NOT VALIDATED (avg: 179.05x)

## Derivatives

- Tests: 15
- Passed: 13
- Failed: 2
- Errors: 0
- Average Speedup: 179.05x

### Test Cases

| Test | Correctness | SymPy Time | MathHook Time | Speedup |
|------|-------------|------------|---------------|----------|
| d/dx(x^2) | PASS | 54855125ns | 34708ns | 1580.47x |
| d/dx(x^3) | PASS | 2523708ns | 19208ns | 131.39x |
| d/dx(x^4) | PASS | 3302584ns | 17458ns | 189.17x |
| d/dx(sin(x)) | PASS | 1639750ns | 48334ns | 33.93x |
| d/dx(cos(x)) | PASS | 1592458ns | 38041ns | 41.86x |
| d/dx(tan(x)) | PASS | 3472250ns | 34500ns | 100.64x |
| d/dx(exp(x)) | PASS | 1755166ns | 48958ns | 35.85x |
| d/dx(log(x)) | FAIL | 1106292ns | 57125ns | 19.37x |
| d/dx(x*sin(x))_product_rule | PASS | 2642625ns | 78541ns | 33.65x |
| d/dx(sin(x)/x)_quotient_rule | PASS | 7291042ns | 129167ns | 56.45x |
| d/dx(sin(x^2))_chain_rule | PASS | 2429875ns | 42459ns | 57.23x |
| d/dx(x^2+2x+1) | PASS | 2744917ns | 14416ns | 190.41x |
| d/dx(x^2*exp(x)) | PASS | 3277833ns | 106458ns | 30.79x |
| d/dx(1/x) | PASS | 986000ns | 5917ns | 166.64x |
| d/dx(sqrt(x)) | FAIL | 3827375ns | 214125ns | 17.87x |

### Failed Tests (Detail)

**d/dx(log(x))**

- Input: `log(x)`
- SymPy: `1/x`
- MathHook: `ln(Integer(10))^Integer(-1) / x`

**d/dx(sqrt(x))**

- Input: `sqrt(x)`
- SymPy: `1/(2*sqrt(x))`
- MathHook: `Rational(Ratio { numer: 1, denom: 2 }) * x^Rational(Ratio { numer: -1, denom: 2 })`



# SymPy Comparison Validation Report - Phase 2

## Summary

- **Total Tests**: 15
- **Passed**: 13 (86.7%)
- **Failed**: 2 (13.3%)
- **Errors**: 0

## Performance Summary

- **Min Speedup**: 8.07x
- **Max Speedup**: 3768.55x
- **Average Speedup**: 342.17x
- **10-100x Claim**: NOT VALIDATED (avg: 342.17x)

## Derivatives

- Tests: 15
- Passed: 13
- Failed: 2
- Errors: 0
- Average Speedup: 342.17x

### Test Cases

| Test | Correctness | SymPy Time | MathHook Time | Speedup |
|------|-------------|------------|---------------|----------|
| d/dx(x^2) | PASS | 102067459ns | 27084ns | 3768.55x |
| d/dx(x^3) | PASS | 3453334ns | 18625ns | 185.41x |
| d/dx(x^4) | PASS | 4240500ns | 17417ns | 243.47x |
| d/dx(sin(x)) | PASS | 1554584ns | 33750ns | 46.06x |
| d/dx(cos(x)) | PASS | 2841625ns | 47416ns | 59.93x |
| d/dx(tan(x)) | PASS | 4434500ns | 51666ns | 85.83x |
| d/dx(exp(x)) | PASS | 1281584ns | 30959ns | 41.40x |
| d/dx(log(x)) | FAIL | 1131250ns | 94208ns | 12.01x |
| d/dx(x*sin(x))_product_rule | PASS | 4133541ns | 88250ns | 46.84x |
| d/dx(sin(x)/x)_quotient_rule | PASS | 6995167ns | 121583ns | 57.53x |
| d/dx(sin(x^2))_chain_rule | PASS | 2255375ns | 57375ns | 39.31x |
| d/dx(x^2+2x+1) | PASS | 5455666ns | 18667ns | 292.26x |
| d/dx(x^2*exp(x)) | PASS | 4695417ns | 203792ns | 23.04x |
| d/dx(1/x) | PASS | 1364584ns | 6125ns | 222.79x |
| d/dx(sqrt(x)) | FAIL | 4315667ns | 534833ns | 8.07x |

### Failed Tests (Detail)

**d/dx(log(x))**

- Input: `log(x)`
- SymPy: `1/x`
- MathHook: `ln(Integer(10))^Integer(-1) / x`

**d/dx(sqrt(x))**

- Input: `sqrt(x)`
- SymPy: `1/(2*sqrt(x))`
- MathHook: `Rational(Ratio { numer: 1, denom: 2 }) * x^Rational(Ratio { numer: -1, denom: 2 })`



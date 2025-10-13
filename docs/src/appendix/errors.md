# Error Messages

Common error messages and their solutions.

## Parse Errors

### "Unexpected token"

**Cause**: Invalid syntax in input expression.

**Solution**: Check for typos, missing parentheses, or unsupported syntax.

### "Implicit multiplication ambiguity"

**Cause**: Parser cannot determine multiplication intent.

**Solution**: Add explicit `*` operator or use parentheses.

## Domain Errors

### "sqrt of negative number"

**Cause**: Attempting to compute square root of negative number in real domain.

**Solution**: Use complex domain or ensure argument is non-negative.

### "Division by zero"

**Cause**: Expression evaluates to division by zero.

**Solution**: Check for zero denominators before evaluation.

### "log of non-positive number"

**Cause**: Logarithm of zero or negative number in real domain.

**Solution**: Ensure argument is positive or use complex domain.

## Solver Errors

### "No solution exists"

**Cause**: Equation has no solution in the given domain.

**Solution**: Check equation for contradictions or domain restrictions.

### "Cannot solve - equation too complex"

**Cause**: Solver cannot handle this equation type yet.

**Solution**: Try simplifying equation or use numerical methods.

## For More Help

- [GitHub Issues](https://github.com/ahmedmashhour/mathhook/issues)
- [API Documentation](https://docs.rs/mathhook-core)

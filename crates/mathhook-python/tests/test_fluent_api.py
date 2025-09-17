"""
Fluent API Integration Tests

Tests the fluent API methods (add, subtract, multiply, divide, pow) with auto-conversion
from Python primitives (int, float) to Expression objects. This enables method chaining
for more readable mathematical expressions.
"""

from mathhook import PyExpression as Expression


def test_add_with_number_autoconversion():
    """Test add() with number auto-conversion"""
    x = Expression.symbol('x')

    add1 = x.add(Expression.integer(2))
    add2 = x.add(2)

    assert add1.to_simple() == add2.to_simple(), f"Expected {add1.to_simple()}, got {add2.to_simple()}"
    print(f"✓ Test 1: Add with number auto-conversion")
    print(f"  x.add(Expression.integer(2)): {add1.to_simple()}")
    print(f"  x.add(2): {add2.to_simple()}")


def test_multiply_with_number_autoconversion():
    """Test multiply() with number auto-conversion"""
    x = Expression.symbol('x')

    mul1 = x.multiply(Expression.integer(3))
    mul2 = x.multiply(3)

    assert mul1.to_simple() == mul2.to_simple(), f"Expected {mul1.to_simple()}, got {mul2.to_simple()}"
    print(f"\n✓ Test 2: Multiply with number auto-conversion")
    print(f"  x.multiply(Expression.integer(3)): {mul1.to_simple()}")
    print(f"  x.multiply(3): {mul2.to_simple()}")


def test_pow_with_number_autoconversion():
    """Test pow() with number auto-conversion"""
    x = Expression.symbol('x')

    pow1 = x.pow(Expression.integer(2))
    pow2 = x.pow(2)

    assert pow1.to_simple() == pow2.to_simple(), f"Expected {pow1.to_simple()}, got {pow2.to_simple()}"
    print(f"\n✓ Test 3: Power with number auto-conversion")
    print(f"  x.pow(Expression.integer(2)): {pow1.to_simple()}")
    print(f"  x.pow(2): {pow2.to_simple()}")


def test_subtract_with_number_autoconversion():
    """Test subtract() with number auto-conversion"""
    x = Expression.symbol('x')

    sub1 = x.subtract(Expression.integer(5))
    sub2 = x.subtract(5)

    assert sub1.to_simple() == sub2.to_simple(), f"Expected {sub1.to_simple()}, got {sub2.to_simple()}"
    print(f"\n✓ Test 4: Subtract with number auto-conversion")
    print(f"  x.subtract(Expression.integer(5)): {sub1.to_simple()}")
    print(f"  x.subtract(5): {sub2.to_simple()}")


def test_divide_with_number_autoconversion():
    """Test divide() with number auto-conversion"""
    x = Expression.symbol('x')

    div1 = x.divide(Expression.integer(2))
    div2 = x.divide(2)

    assert div1.to_simple() == div2.to_simple(), f"Expected {div1.to_simple()}, got {div2.to_simple()}"
    print(f"\n✓ Test 5: Divide with number auto-conversion")
    print(f"  x.divide(Expression.integer(2)): {div1.to_simple()}")
    print(f"  x.divide(2): {div2.to_simple()}")


def test_fluent_chaining_with_numbers():
    """Test fluent chaining with numbers"""
    x = Expression.symbol('x')

    chain = x.add(2).multiply(3).pow(2)
    print(f"\n✓ Test 6: Fluent chaining with numbers")
    print(f"  x.add(2).multiply(3).pow(2): {chain.to_simple()}")

    chain_simplified = chain.simplify()
    print(f"  Simplified: {chain_simplified.to_simple()}")


def test_complex_expression_with_mixed_types():
    """Test complex expression with mixed types"""
    x = Expression.symbol('x')

    expr = x.pow(2).add(x.multiply(2)).add(1)
    print(f"\n✓ Test 7: Complex expression with mixed types")
    print(f"  x.pow(2).add(x.multiply(2)).add(1): {expr.to_simple()}")

    expr_simplified = expr.simplify()
    print(f"  Simplified: {expr_simplified.to_simple()}")


def test_float_conversion():
    """Test float conversion (should remain float)"""
    x = Expression.symbol('x')

    float_expr = x.multiply(3.14)
    print(f"\n✓ Test 8: Float conversion (3.14 should remain float)")
    print(f"  x.multiply(3.14): {float_expr.to_simple()}")


def test_integer_conversion():
    """Test integer conversion (2.0 should become integer)"""
    x = Expression.symbol('x')

    int_expr = x.multiply(2.0)
    print(f"\n✓ Test 9: Integer conversion (2.0 should become integer)")
    print(f"  x.multiply(2.0): {int_expr.to_simple()}")


def test_negate_expression():
    """Test negate() expression"""
    x = Expression.symbol('x')

    neg = x.negate()
    print(f"\n✓ Test 10: Negate expression")
    print(f"  x.negate(): {neg.to_simple()}")


def test_quadratic_formula_example():
    """Test quadratic formula example: (-b ± sqrt(b^2 - 4ac)) / 2a"""
    a = Expression.symbol('a')
    b = Expression.symbol('b')
    c = Expression.symbol('c')

    discriminant = b.pow(2).subtract(a.multiply(4).multiply(c))
    sqrt_disc = Expression.function('sqrt', [discriminant])
    numerator1 = b.negate().add(sqrt_disc)
    numerator2 = b.negate().subtract(sqrt_disc)
    denom = a.multiply(2)
    solution1 = numerator1.divide(denom)
    solution2 = numerator2.divide(denom)

    print(f"\n✓ Test 11: Quadratic formula example")
    print(f"  Discriminant: {discriminant.to_simple()}")
    print(f"  Solution 1: {solution1.to_simple()}")
    print(f"  Solution 2: {solution2.to_simple()}")


def test_mixed_operators_and_methods():
    """Test mixing operators and methods"""
    x = Expression.symbol('x')

    expr1 = x + 2
    expr2 = expr1.multiply(3)
    expr3 = expr2 ** 2

    print(f"\n✓ Test 12: Mixing operators and methods")
    print(f"  (x + 2).multiply(3) ** 2: {expr3.to_simple()}")

    alt_expr = x.add(2).multiply(3).pow(2)
    assert expr3.to_simple() == alt_expr.to_simple(), \
        f"Operator and method mixing should be equivalent"
    print(f"  Alternative (all methods): {alt_expr.to_simple()}")


if __name__ == '__main__':
    print("=== Fluent API Tests ===\n")

    test_add_with_number_autoconversion()
    test_multiply_with_number_autoconversion()
    test_pow_with_number_autoconversion()
    test_subtract_with_number_autoconversion()
    test_divide_with_number_autoconversion()
    test_fluent_chaining_with_numbers()
    test_complex_expression_with_mixed_types()
    test_float_conversion()
    test_integer_conversion()
    test_negate_expression()
    test_quadratic_formula_example()
    test_mixed_operators_and_methods()

    print("\n=== All Fluent API Tests Complete ===")

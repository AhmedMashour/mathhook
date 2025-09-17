"""Test symbols() helper function

This test suite verifies that the symbols() function correctly creates
multiple symbols with various syntaxes: space-separated, comma-separated,
and range syntax.
"""
import mathhook
from mathhook import symbols


def test_space_separated():
    """Test space-separated symbol creation: 'x y z'"""
    x, y, z = symbols('x y z')
    assert str(x) == 'x'
    assert str(y) == 'y'
    assert str(z) == 'z'


def test_comma_separated():
    """Test comma-separated symbol creation: 'a,b,c'"""
    a, b, c = symbols('a,b,c')
    assert str(a) == 'a'
    assert str(b) == 'b'
    assert str(c) == 'c'


def test_comma_separated_with_spaces():
    """Test comma-separated with spaces: 'a, b, c'"""
    a, b, c = symbols('a, b, c')
    assert str(a) == 'a'
    assert str(b) == 'b'
    assert str(c) == 'c'


def test_mixed_whitespace():
    """Test handling of extra whitespace"""
    theta, phi = symbols('theta   phi')  # Multiple spaces
    assert str(theta) == 'theta'
    assert str(phi) == 'phi'


def test_range_syntax_from_zero():
    """Test range syntax starting from 0: 'x0:3'"""
    x0, x1, x2 = symbols('x0:3')
    assert str(x0) == 'x0'
    assert str(x1) == 'x1'
    assert str(x2) == 'x2'


def test_range_syntax_from_one():
    """Test range syntax starting from 1: 'x1:5'"""
    x1, x2, x3, x4 = symbols('x1:5')
    assert str(x1) == 'x1'
    assert str(x2) == 'x2'
    assert str(x3) == 'x3'
    assert str(x4) == 'x4'


def test_range_without_number():
    """Test range syntax without starting number: 'x:3'"""
    x0, x1, x2 = symbols('x:3')
    assert str(x0) == 'x0'
    assert str(x1) == 'x1'
    assert str(x2) == 'x2'


def test_single_symbol_space():
    """Test single symbol with space syntax"""
    result = symbols('x')
    assert len(result) == 1
    x = result[0]
    assert str(x) == 'x'


def test_single_symbol_unpacking():
    """Test single symbol with tuple unpacking"""
    x, = symbols('x')  # Note: trailing comma for unpacking
    assert str(x) == 'x'


def test_greek_letters():
    """Test Greek letter symbol names"""
    alpha, beta, gamma = symbols('alpha beta gamma')
    assert str(alpha) == 'alpha'
    assert str(beta) == 'beta'
    assert str(gamma) == 'gamma'


def test_long_names():
    """Test symbols with longer names"""
    velocity, acceleration = symbols('velocity acceleration')
    assert str(velocity) == 'velocity'
    assert str(acceleration) == 'acceleration'


def test_subscript_style():
    """Test symbols with subscript-style names"""
    x_1, x_2, x_3 = symbols('x_1 x_2 x_3')
    assert str(x_1) == 'x_1'
    assert str(x_2) == 'x_2'
    assert str(x_3) == 'x_3'


def test_invalid_range_too_many_colons():
    """Test that invalid range syntax raises ValueError"""
    try:
        symbols('x1:2:3')  # Too many colons
        assert False, "Should raise ValueError"
    except ValueError:
        pass


def test_invalid_range_non_numeric():
    """Test that non-numeric range end raises ValueError"""
    try:
        symbols('x:abc')  # Non-numeric end
        assert False, "Should raise ValueError"
    except ValueError:
        pass


def test_empty_string():
    """Test that empty string raises ValueError"""
    try:
        symbols('')
        assert False, "Should raise ValueError"
    except ValueError:
        pass


def test_only_spaces():
    """Test that string with only spaces raises ValueError"""
    try:
        symbols('   ')
        assert False, "Should raise ValueError"
    except ValueError:
        pass


def test_usage_in_expression():
    """Test that symbols work in actual expressions"""
    x, y = symbols('x y')
    expr = x + 2*y
    expr_str = str(expr)
    assert 'x' in expr_str and 'y' in expr_str


def test_quadratic_expression():
    """Test symbols in quadratic expression: x^2 + 2xy + y^2"""
    x, y = symbols('x y')
    expr = x**2 + 2*x*y + y**2
    expr_str = str(expr)
    assert 'x' in expr_str and 'y' in expr_str


def test_polynomial_coefficients():
    """Test range syntax for polynomial coefficients"""
    c0, c1, c2, c3 = symbols('c0:4')
    x, = symbols('x')

    # Build polynomial: c0 + c1*x + c2*x^2 + c3*x^3
    poly = c0 + c1*x + c2*x**2 + c3*x**3
    poly_str = str(poly)
    assert 'x' in poly_str
    assert 'c0' in poly_str or 'c1' in poly_str or 'c2' in poly_str


def test_many_symbols_space():
    """Test creating many symbols at once (space-separated)"""
    result = symbols('a b c d e f g h i j')
    assert len(result) == 10
    assert str(result[0]) == 'a'
    assert str(result[9]) == 'j'


def test_many_symbols_comma():
    """Test creating many symbols at once (comma-separated)"""
    result = symbols('a,b,c,d,e,f,g,h,i,j')
    assert len(result) == 10
    assert str(result[0]) == 'a'
    assert str(result[9]) == 'j'


def test_many_symbols_range():
    """Test creating many symbols with range syntax"""
    result = symbols('x0:10')
    assert len(result) == 10
    assert str(result[0]) == 'x0'
    assert str(result[9]) == 'x9'


def test_range_large_numbers():
    """Test range syntax with larger numbers"""
    x10, x11, x12 = symbols('x10:13')
    assert str(x10) == 'x10'
    assert str(x11) == 'x11'
    assert str(x12) == 'x12'


def test_symbols_are_distinct():
    """Test that each symbol is a distinct object"""
    x, y = symbols('x y')
    # Symbols with different names should have different string representations
    assert str(x) != str(y)


def test_symbols_in_multiple_expressions():
    """Test using symbols in multiple independent expressions"""
    x, y, z = symbols('x y z')

    expr1 = x + y
    expr2 = y + z
    expr3 = x + z

    # All expressions should be valid
    assert str(expr1)
    assert str(expr2)
    assert str(expr3)


def test_trailing_comma_space():
    """Test handling of trailing comma in space-separated list"""
    # Should handle gracefully even with extra spaces
    result = symbols('x y z  ')
    assert len(result) == 3


def test_trailing_comma_comma_sep():
    """Test handling of trailing comma in comma-separated list"""
    result = symbols('x,y,z,')
    assert len(result) == 3
    assert str(result[2]) == 'z'

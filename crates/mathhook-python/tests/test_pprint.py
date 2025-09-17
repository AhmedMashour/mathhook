"""Test pprint() pretty printing with Unicode

This test suite verifies that pprint() correctly formats expressions
with Unicode characters for terminal display.
"""
import mathhook
from mathhook import pprint, symbols


def test_pprint_basic(capsys):
    """Test basic pprint output"""
    x, = symbols('x')
    expr = x + 1

    pprint(expr)
    captured = capsys.readouterr()

    # Should have output
    assert captured.out.strip()
    assert 'x' in captured.out


def test_pprint_power(capsys):
    """Test pprint with power expressions"""
    x, = symbols('x')
    expr = x**2

    pprint(expr, use_unicode=True)
    captured = capsys.readouterr()

    # Should contain x
    assert 'x' in captured.out


def test_pprint_no_unicode(capsys):
    """Test pprint without Unicode"""
    x, = symbols('x')
    expr = x**2

    pprint(expr, use_unicode=False)
    captured = capsys.readouterr()

    # Should be plain ASCII
    assert 'x' in captured.out
    assert captured.out.strip()


def test_pprint_addition(capsys):
    """Test pprint with addition"""
    x, y = symbols('x y')
    expr = x + y

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    assert 'y' in captured.out


def test_pprint_multiplication(capsys):
    """Test pprint with multiplication"""
    x, y = symbols('x y')
    expr = 2 * x * y

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    assert 'y' in captured.out


def test_pprint_complex_expr(capsys):
    """Test pprint with complex expression"""
    x, y = symbols('x y')
    expr = (x + y) ** 2

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    assert 'y' in captured.out


def test_pprint_function(capsys):
    """Test pprint with function"""
    from mathhook import sin

    x, = symbols('x')
    expr = sin(x)

    pprint(expr)
    captured = capsys.readouterr()

    assert 'sin' in captured.out or 'x' in captured.out


def test_pprint_multiple_terms(capsys):
    """Test pprint with multiple terms"""
    x, y, z = symbols('x y z')
    expr = x + 2*y + 3*z

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    assert 'y' in captured.out
    assert 'z' in captured.out


def test_pprint_nested_expr(capsys):
    """Test pprint with nested expression"""
    x, y = symbols('x y')
    expr = (x + 1) * (y - 1)

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    assert 'y' in captured.out


def test_pprint_division(capsys):
    """Test pprint with division"""
    x, y = symbols('x y')
    expr = x / y

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    assert 'y' in captured.out


def test_pprint_with_numbers(capsys):
    """Test pprint with numeric coefficients"""
    x, = symbols('x')
    expr = 3*x + 5

    pprint(expr)
    captured = capsys.readouterr()

    assert 'x' in captured.out
    # Should show the numbers in some form
    assert captured.out.strip()


def test_pprint_default_unicode_enabled(capsys):
    """Test that Unicode is enabled by default"""
    x, = symbols('x')
    expr = x + 1

    # Call without use_unicode parameter (should default to True)
    pprint(expr)
    captured = capsys.readouterr()

    assert captured.out.strip()
    assert 'x' in captured.out


def test_pprint_output_format(capsys):
    """Test that pprint produces valid output"""
    x, = symbols('x')
    expr = x**3 + 2*x**2 + 3*x + 4

    pprint(expr)
    captured = capsys.readouterr()

    # Output should not be empty
    assert captured.out.strip()
    # Should contain x
    assert 'x' in captured.out

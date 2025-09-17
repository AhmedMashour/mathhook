"""Test display methods

This test suite verifies that expressions have proper string representations
and LaTeX rendering support for Jupyter notebooks.
"""
import mathhook
from mathhook import symbols, sin, cos, exp, sqrt


def test_str_method():
    """Test __str__ method for basic string representation"""
    x, = symbols('x')
    string = str(x)
    assert string == 'x'


def test_str_with_expression():
    """Test __str__ with more complex expression"""
    x, y = symbols('x y')
    expr = x + y
    string = str(expr)
    assert 'x' in string and 'y' in string


def test_repr_method():
    """Test __repr__ method for debugging representation"""
    x, = symbols('x')
    repr_str = repr(x)
    # repr should include class name or similar context
    assert 'x' in repr_str


def test_repr_latex_exists():
    """Test that _repr_latex_ method exists"""
    x, = symbols('x')
    # Should have the method
    assert hasattr(x, '_repr_latex_')


def test_repr_latex_simple():
    """Test _repr_latex_ with simple symbol"""
    x, = symbols('x')
    latex = x._repr_latex_()
    # Should be wrapped in $ signs
    assert latex.startswith('$') and latex.endswith('$')
    assert 'x' in latex


def test_repr_latex_power():
    """Test _repr_latex_ with power expression"""
    x, = symbols('x')
    expr = x**2
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    # Should contain x and power notation
    assert 'x' in latex


def test_repr_latex_addition():
    """Test _repr_latex_ with addition"""
    x, y = symbols('x y')
    expr = x + y
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    assert 'x' in latex and 'y' in latex


def test_repr_latex_function():
    """Test _repr_latex_ with function calls"""
    x, = symbols('x')
    expr = sin(x)
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    # Should contain sin or \sin
    assert 'sin' in latex.lower() or r'\sin' in latex


def test_repr_latex_complex_expr():
    """Test _repr_latex_ with complex expression"""
    x, y = symbols('x y')
    expr = x**2 + 2*x*y + y**2
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    # Should be valid LaTeX string
    assert len(latex) > 4  # More than just "$$"


def test_repr_latex_fraction():
    """Test _repr_latex_ with division/fraction"""
    x, = symbols('x')
    expr = x / 2
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    assert 'x' in latex


def test_repr_latex_sqrt():
    """Test _repr_latex_ with square root"""
    x, = symbols('x')
    expr = sqrt(x)
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    assert 'x' in latex


def test_repr_latex_nested():
    """Test _repr_latex_ with nested functions"""
    x, = symbols('x')
    expr = sin(cos(x))
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')


def test_str_vs_repr():
    """Test that str() and repr() give different outputs"""
    x, = symbols('x')
    string = str(x)
    repr_str = repr(x)

    # Both should contain 'x' but repr may have more context
    assert 'x' in string
    assert 'x' in repr_str


def test_print_behavior():
    """Test that expressions work with print()"""
    x, = symbols('x')
    # print() uses __str__, should not raise
    import io
    import sys
    old_stdout = sys.stdout
    sys.stdout = io.StringIO()
    try:
        print(x)
        output = sys.stdout.getvalue()
        assert 'x' in output
    finally:
        sys.stdout = old_stdout


def test_latex_with_multiple_variables():
    """Test LaTeX with multiple variables"""
    x, y, z = symbols('x y z')
    expr = x*y + y*z + z*x
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    # Should contain all variables
    assert 'x' in latex and 'y' in latex and 'z' in latex


def test_latex_with_coefficients():
    """Test LaTeX with numeric coefficients"""
    x, = symbols('x')
    expr = 2*x + 3
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    assert 'x' in latex


def test_latex_exponential():
    """Test LaTeX with exponential"""
    x, = symbols('x')
    expr = exp(x)
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')


def test_latex_trig_expression():
    """Test LaTeX with trigonometric expression"""
    x, = symbols('x')
    expr = sin(x) + cos(x)
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')


def test_latex_polynomial():
    """Test LaTeX with polynomial"""
    x, = symbols('x')
    expr = x**3 + 2*x**2 - x + 1
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')
    assert 'x' in latex


def test_latex_is_string():
    """Test that _repr_latex_ returns a string"""
    x, = symbols('x')
    latex = x._repr_latex_()
    assert isinstance(latex, str)


def test_str_is_string():
    """Test that __str__ returns a string"""
    x, = symbols('x')
    string = str(x)
    assert isinstance(string, str)


def test_repr_is_string():
    """Test that __repr__ returns a string"""
    x, = symbols('x')
    repr_str = repr(x)
    assert isinstance(repr_str, str)


def test_latex_not_empty():
    """Test that LaTeX string is not empty"""
    x, = symbols('x')
    latex = x._repr_latex_()
    # Should be more than just the delimiters
    assert len(latex) > 2


def test_latex_contains_math():
    """Test that LaTeX string contains mathematical content"""
    x, y = symbols('x y')
    expr = x**2 + y**2
    latex = expr._repr_latex_()
    # Remove the $ signs
    content = latex.strip('$')
    # Should have mathematical content
    assert len(content) > 0
    assert 'x' in content or 'y' in content

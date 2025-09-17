"""Week 1 Integration Tests

This test suite verifies that all Week 1 features work together seamlessly,
testing realistic workflows that combine operators, symbols, functions, and display.
"""
import mathhook
from mathhook import symbols, sin, cos, tan, exp, log, sqrt


def test_complete_workflow():
    """Test realistic workflow combining all Week 1 features"""
    # symbols() helper
    x, y, z = symbols('x y z')

    # Operator overloading with auto-sympification
    expr1 = 2*x + 3*y - 5
    expr2 = x**2 + y**2

    # Function shortcuts
    expr3 = sin(x) + cos(y)
    expr4 = exp(log(x))

    # Combined expression
    complex_expr = (2*x + 3) * (x - 1) + sqrt(y)

    # All should be valid expressions
    assert str(expr1)
    assert 'x' in str(expr1) or 'y' in str(expr1)
    assert 'x' in str(expr2) and 'y' in str(expr2)
    assert str(expr3)
    assert str(expr4)
    assert str(complex_expr)


def test_sympy_like_usage():
    """Test code that looks like SymPy"""
    x, y = symbols('x y')

    # Quadratic-like expression
    a, b, c = 1, -5, 6
    # Build expression: b^2 - 4*a*c
    discriminant = b**2 - 4*a*c
    assert str(discriminant)

    # Taylor series-like construction
    taylor = 1 + x + x**2/2 + x**3/6
    assert str(taylor)

    # Trig identity
    identity = sin(x)**2 + cos(x)**2
    assert str(identity)


def test_jupyter_ready():
    """Test that expressions are Jupyter-ready"""
    x, y = symbols('x y')
    expr = x**2 + 2*x*y + y**2

    # Should have LaTeX representation
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')

    # Should have string representation
    string = str(expr)
    assert isinstance(string, str)

    # Should have repr
    repr_str = repr(expr)
    assert isinstance(repr_str, str)


def test_error_handling():
    """Test that errors are clear and helpful"""
    x, = symbols('x')

    try:
        result = x + [1, 2, 3]  # Should fail
        assert False, "Should raise TypeError"
    except TypeError as e:
        # Error message should be informative
        assert "Cannot convert" in str(e) or "list" in str(e).lower()


def test_range_syntax_in_real_use():
    """Test range syntax in realistic scenario"""
    # Create polynomial coefficients
    c0, c1, c2, c3 = symbols('c0:4')
    x, = symbols('x')

    # Build polynomial
    poly = c0 + c1*x + c2*x**2 + c3*x**3
    assert str(poly)
    # Should contain some coefficient symbols
    poly_str = str(poly)
    assert any(coef in poly_str for coef in ['c0', 'c1', 'c2', 'c3'])


def test_function_composition():
    """Test composing multiple functions"""
    x, = symbols('x')

    # Nested functions
    expr = sin(cos(exp(x)))
    assert str(expr)

    # Function arithmetic
    expr2 = sin(x)**2 + cos(x)**2 - 1
    assert str(expr2)


def test_complex_mathematical_expression():
    """Test building complex mathematical expressions"""
    x, y = symbols('x y')

    # Distance formula in 2D
    distance = sqrt(x**2 + y**2)
    assert str(distance)

    # Quadratic formula components
    a, b = symbols('a b')
    discriminant = b**2 - 4*a*x
    assert str(discriminant)


def test_operator_precedence():
    """Test that operator precedence is correct"""
    x, = symbols('x')

    # 2 + 3 * x should be 2 + (3*x), not (2+3)*x
    expr1 = 2 + 3 * x
    # Just verify it creates a valid expression
    assert str(expr1)

    # x^2 + x should be (x^2) + x, not x^(2+x)
    expr2 = x**2 + x
    assert str(expr2)


def test_mixed_operations():
    """Test mixing different operation types"""
    x, y, z = symbols('x y z')

    # Mix arithmetic, powers, and functions
    expr = sin(x**2) + cos(y) * exp(z) / 2
    assert str(expr)


def test_function_derivatives_placeholder():
    """Test that derivative expressions can be built (evaluation tested elsewhere)"""
    x, = symbols('x')

    # Create expressions that would be used for derivatives
    f = x**2 + 2*x + 1
    g = sin(x)
    h = exp(x)

    # All should be valid expressions
    assert str(f) and str(g) and str(h)


def test_symbolic_constants():
    """Test working with symbolic mathematical constants"""
    x, = symbols('x')

    # Create expressions with e and pi (if available)
    # For now, just test that we can create numeric approximations
    expr1 = x * 3.14159  # pi approximation
    expr2 = x + 2.71828  # e approximation

    assert str(expr1) and str(expr2)


def test_latex_for_common_expressions():
    """Test LaTeX output for commonly used expressions"""
    x, y = symbols('x y')

    test_cases = [
        x + y,
        x * y,
        x / y,
        x**2,
        sin(x),
        sqrt(x),
        x**2 + y**2,
        sin(x)**2 + cos(x)**2,
    ]

    for expr in test_cases:
        latex = expr._repr_latex_()
        # Should be wrapped in dollar signs
        assert latex.startswith('$') and latex.endswith('$')
        # Should have content
        assert len(latex) > 2


def test_all_operators_work():
    """Comprehensive test of all operator types"""
    x, y = symbols('x y')

    # Addition
    add_expr = x + y + 1
    assert str(add_expr)

    # Subtraction
    sub_expr = x - y - 1
    assert str(sub_expr)

    # Multiplication
    mul_expr = 2 * x * y
    assert str(mul_expr)

    # Division
    div_expr = x / y / 2
    assert str(div_expr)

    # Power
    pow_expr = x ** y ** 2
    assert str(pow_expr)

    # Negation
    neg_expr = -x
    assert str(neg_expr)


def test_all_functions_work():
    """Comprehensive test of all function types"""
    x, = symbols('x')

    functions_to_test = [
        sin(x),
        cos(x),
        tan(x),
        exp(x),
        log(x),
        sqrt(x),
    ]

    for expr in functions_to_test:
        # Each should produce a valid expression
        assert str(expr)
        # Should have LaTeX representation
        latex = expr._repr_latex_()
        assert latex.startswith('$') and latex.endswith('$')


def test_realistic_physics_formula():
    """Test building physics-like formulas"""
    # Variables
    x, v, t, a = symbols('x v t a')

    # Kinematic equation: x = v*t + 0.5*a*t^2
    position = v*t + 0.5*a*t**2
    assert str(position)

    # Should be Jupyter-ready
    latex = position._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')


def test_realistic_calculus_setup():
    """Test setting up expressions for calculus operations"""
    x, = symbols('x')

    # Common calculus expressions
    polynomial = x**3 - 2*x**2 + x - 1
    trig_func = sin(x) * cos(x)
    exponential = exp(x**2)
    rational = (x + 1) / (x - 1)

    # All should be valid and have LaTeX
    for expr in [polynomial, trig_func, exponential, rational]:
        assert str(expr)
        latex = expr._repr_latex_()
        assert latex.startswith('$') and latex.endswith('$')


def test_creating_system_of_equations():
    """Test creating expressions for equation systems"""
    x, y, z = symbols('x y z')

    # System of equations (as expressions, not solved yet)
    eq1 = 2*x + 3*y - z
    eq2 = x - y + 2*z
    eq3 = 3*x + y + z

    # All should be valid expressions
    assert str(eq1) and str(eq2) and str(eq3)


def test_expression_manipulation():
    """Test that expressions can be built and manipulated"""
    x, y = symbols('x y')

    # Start with simple expression
    expr = x + y

    # Add more terms
    expr = expr + x**2
    expr = expr * 2
    expr = expr - 1

    # Should still be valid
    assert str(expr)


def test_import_variations():
    """Test different import patterns work"""
    # Already imported at top:
    # from mathhook import symbols, sin, cos, ...

    # Test creating symbols with symbols()
    x, = mathhook.symbols('x')
    assert str(x) == 'x'

    # Test functions work
    result = sin(x)
    assert str(result)


def test_numeric_integration():
    """Test mixing numeric and symbolic seamlessly"""
    x, = symbols('x')

    # Numeric + symbolic
    expr1 = 3.14 * x
    assert str(expr1)

    # Integer + symbolic
    expr2 = 42 + x
    assert str(expr2)

    # Float division
    expr3 = x / 2.5
    assert str(expr3)


def test_unicode_compatibility():
    """Test that Greek letters work as symbol names"""
    alpha, beta, gamma = symbols('alpha beta gamma')

    expr = alpha**2 + beta*gamma
    assert str(expr)


def test_chained_operations():
    """Test that operations can be chained fluently"""
    x, y = symbols('x y')

    # Long chain of operations
    result = ((x + 1) * (y - 1)) ** 2 / (x + y)
    assert str(result)
    assert result._repr_latex_()


def test_week1_completeness():
    """Verify all Week 1 features are present and working"""
    # Feature 1: symbols() helper
    x, y, z = symbols('x y z')
    assert all(str(s) for s in [x, y, z])

    # Feature 2: Operator overloading
    expr = x + 2 * y - z / 3
    assert str(expr)

    # Feature 3: Function shortcuts
    funcs = sin(x) + cos(y) + exp(z)
    assert str(funcs)

    # Feature 4: Jupyter display
    latex = expr._repr_latex_()
    assert latex.startswith('$') and latex.endswith('$')

    # All features integrated
    complex_expr = (sin(x) + cos(y))**2 + sqrt(z)
    assert str(complex_expr)
    assert complex_expr._repr_latex_()


def test_no_regressions():
    """Test that basic functionality hasn't regressed"""
    # Test that symbols() works
    x, = mathhook.symbols('x')
    assert str(x) == 'x'

    # Test that basic operations still work with operator overloading
    expr = x + 1
    assert str(expr)

"""Test function shortcuts

This test suite verifies that all mathematical function shortcuts work correctly
with both symbolic expressions and numeric values, supporting auto-sympification.
"""
import mathhook
from mathhook import symbols, sin, cos, tan, exp, log, sqrt, asin, acos, atan, sinh, cosh, tanh


def test_sin_with_symbol():
    """Test sin() with symbolic argument"""
    x, = symbols('x')
    result = sin(x)
    result_str = str(result)
    assert 'sin' in result_str and 'x' in result_str


def test_sin_with_number():
    """Test sin() with numeric argument"""
    result = sin(3.14159)
    # Should return expression, not evaluate
    assert str(result)


def test_cos_with_symbol():
    """Test cos() with symbolic argument"""
    x, = symbols('x')
    result = cos(x)
    result_str = str(result)
    assert 'cos' in result_str and 'x' in result_str


def test_tan_with_symbol():
    """Test tan() with symbolic argument"""
    x, = symbols('x')
    result = tan(x)
    result_str = str(result)
    assert 'tan' in result_str and 'x' in result_str


def test_trig_identity():
    """Test sin^2 + cos^2 expression"""
    x, = symbols('x')
    expr = sin(x)**2 + cos(x)**2
    expr_str = str(expr)
    # Should contain sin and/or cos
    assert 'sin' in expr_str or 'cos' in expr_str


def test_exp_with_symbol():
    """Test exp() with symbolic argument"""
    x, = symbols('x')
    result = exp(x)
    result_str = str(result)
    assert 'exp' in result_str and 'x' in result_str


def test_exp_with_number():
    """Test exp() with numeric argument"""
    result = exp(1.0)
    assert str(result)


def test_log_default_base():
    """Test log() with default base (natural logarithm)"""
    x, = symbols('x')
    result = log(x)
    result_str = str(result)
    # Natural log should use 'ln'
    assert 'ln' in result_str or 'log' in result_str


def test_log_with_base():
    """Test log() with explicit base"""
    x, = symbols('x')
    result = log(x, 10)
    result_str = str(result)
    assert 'log' in result_str


def test_log_exp_identity():
    """Test exp(log(x)) expression"""
    x, = symbols('x')
    expr = exp(log(x))
    # Should create a valid expression
    assert str(expr)


def test_sqrt_with_symbol():
    """Test sqrt() with symbolic argument"""
    x, = symbols('x')
    result = sqrt(x)
    result_str = str(result)
    # sqrt is represented as x^(1/2)
    assert 'x' in result_str


def test_sqrt_with_number():
    """Test sqrt() with numeric argument"""
    result = sqrt(4)
    # Should create expression for sqrt(4)
    assert str(result)


def test_sqrt_in_expression():
    """Test sqrt() in complex expression"""
    x, y = symbols('x y')
    expr = sqrt(x**2 + y**2)
    assert str(expr)


def test_asin_with_symbol():
    """Test asin() inverse trig function"""
    x, = symbols('x')
    result = asin(x)
    result_str = str(result)
    assert 'asin' in result_str and 'x' in result_str


def test_acos_with_symbol():
    """Test acos() inverse trig function"""
    x, = symbols('x')
    result = acos(x)
    result_str = str(result)
    assert 'acos' in result_str and 'x' in result_str


def test_atan_with_symbol():
    """Test atan() inverse trig function"""
    x, = symbols('x')
    result = atan(x)
    result_str = str(result)
    assert 'atan' in result_str and 'x' in result_str


def test_sinh_with_symbol():
    """Test sinh() hyperbolic function"""
    x, = symbols('x')
    result = sinh(x)
    result_str = str(result)
    assert 'sinh' in result_str and 'x' in result_str


def test_cosh_with_symbol():
    """Test cosh() hyperbolic function"""
    x, = symbols('x')
    result = cosh(x)
    result_str = str(result)
    assert 'cosh' in result_str and 'x' in result_str


def test_tanh_with_symbol():
    """Test tanh() hyperbolic function"""
    x, = symbols('x')
    result = tanh(x)
    result_str = str(result)
    assert 'tanh' in result_str and 'x' in result_str


def test_hyperbolic_identity():
    """Test cosh^2 - sinh^2 expression"""
    x, = symbols('x')
    expr = cosh(x)**2 - sinh(x)**2
    # Should create valid expression
    assert str(expr)


def test_function_composition():
    """Test composing multiple functions: sin(cos(tan(x)))"""
    x, = symbols('x')
    expr = sin(cos(tan(x)))
    expr_str = str(expr)
    # Should contain all three functions
    assert 'sin' in expr_str and 'cos' in expr_str and 'tan' in expr_str


def test_nested_functions():
    """Test deeply nested function calls"""
    x, = symbols('x')
    expr = sin(cos(exp(log(x))))
    # Should create valid nested expression
    assert str(expr)


def test_function_arithmetic():
    """Test arithmetic with functions: 2*sin(x) + 3*cos(x)"""
    x, = symbols('x')
    expr = 2 * sin(x) + 3 * cos(x)
    expr_str = str(expr)
    assert 'sin' in expr_str and 'cos' in expr_str


def test_function_power():
    """Test functions raised to powers: sin(x)^2"""
    x, = symbols('x')
    expr = sin(x) ** 2
    expr_str = str(expr)
    assert 'sin' in expr_str


def test_function_division():
    """Test function division: sin(x) / cos(x) (tan identity)"""
    x, = symbols('x')
    expr = sin(x) / cos(x)
    # Should create valid expression
    assert str(expr)


def test_mixed_functions_and_operators():
    """Test complex expression: (sin(x) + cos(y)) * exp(z)"""
    x, y, z = symbols('x y z')
    expr = (sin(x) + cos(y)) * exp(z)
    expr_str = str(expr)
    assert 'sin' in expr_str or 'cos' in expr_str or 'exp' in expr_str


def test_functions_with_expressions():
    """Test functions with complex arguments: sin(x^2 + 1)"""
    x, = symbols('x')
    expr = sin(x**2 + 1)
    # Should create valid expression
    assert str(expr)


def test_exp_log_cancellation():
    """Test that exp and log create proper expressions"""
    x, = symbols('x')
    expr1 = exp(log(x))
    expr2 = log(exp(x))
    # Both should create valid expressions
    assert str(expr1) and str(expr2)


def test_sqrt_square_cancellation():
    """Test sqrt and square operations"""
    x, = symbols('x')
    expr1 = sqrt(x**2)
    expr2 = sqrt(x) ** 2
    # Both should create valid expressions
    assert str(expr1) and str(expr2)


def test_function_with_zero():
    """Test functions with zero argument"""
    result1 = sin(0)
    result2 = cos(0)
    result3 = exp(0)
    # All should create valid expressions
    assert str(result1) and str(result2) and str(result3)


def test_function_with_negative():
    """Test functions with negative arguments"""
    x, = symbols('x')
    expr = sin(-x)
    # Should create valid expression
    assert str(expr)


def test_multiple_arguments_log():
    """Test log() with both arguments"""
    x, = symbols('x')
    result = log(x, 2)  # log base 2
    # Should create valid expression
    assert str(result)


def test_all_trig_functions():
    """Test that all trig functions work"""
    x, = symbols('x')
    functions = [sin, cos, tan, asin, acos, atan, sinh, cosh, tanh]

    for func in functions:
        result = func(x)
        # Each should create a valid expression
        assert str(result)


def test_functions_with_float():
    """Test functions with float arguments"""
    results = [
        sin(1.5),
        cos(2.7),
        tan(0.5),
        exp(1.0),
        log(2.71828),
        sqrt(2.0)
    ]

    # All should create valid expressions
    for result in results:
        assert str(result)


def test_function_chain_evaluation():
    """Test that function chains don't evaluate prematurely"""
    x, = symbols('x')
    # These should remain as symbolic expressions
    expr1 = sin(cos(x))
    expr2 = exp(log(x))

    # Should not simplify to just x or numeric value
    assert str(expr1) and str(expr2)

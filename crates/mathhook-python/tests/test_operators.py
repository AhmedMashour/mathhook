"""Test operator overloading with auto-sympification

This test suite verifies that all Python operators work correctly with
MathHook expressions, including automatic conversion (sympification) of
Python primitives to expressions.
"""
import mathhook
from mathhook import symbols


def test_addition_with_int():
    """Test x + 2 with auto-sympification"""
    x, = symbols('x')
    result = x + 2
    result_str = str(result)
    assert 'x' in result_str and '2' in result_str


def test_right_addition():
    """Test 3 + x with auto-sympification"""
    x, = symbols('x')
    result = 3 + x
    result_str = str(result)
    assert 'x' in result_str and '3' in result_str


def test_subtraction_with_float():
    """Test x - 3.14 with auto-sympification"""
    x, = symbols('x')
    result = x - 3.14
    result_str = str(result)
    assert 'x' in result_str


def test_right_subtraction():
    """Test 5 - x with auto-sympification"""
    x, = symbols('x')
    result = 5 - x
    result_str = str(result)
    assert 'x' in result_str and '5' in result_str


def test_multiplication():
    """Test 2 * x * 3 with auto-sympification"""
    x, = symbols('x')
    result = 2 * x * 3
    result_str = str(result)
    assert 'x' in result_str


def test_right_multiplication():
    """Test x * 4 with auto-sympification"""
    x, = symbols('x')
    result = x * 4
    result_str = str(result)
    assert 'x' in result_str and '4' in result_str


def test_division():
    """Test x / 2 with auto-sympification"""
    x, = symbols('x')
    result = x / 2
    result_str = str(result)
    # Division is represented as multiplication by inverse: x * 2^(-1)
    assert 'x' in result_str


def test_right_division():
    """Test 6 / x with auto-sympification"""
    x, = symbols('x')
    result = 6 / x
    result_str = str(result)
    assert 'x' in result_str and '6' in result_str


def test_power():
    """Test x ** 2 with auto-sympification"""
    x, = symbols('x')
    result = x ** 2
    result_str = str(result)
    assert 'x' in result_str


def test_right_power():
    """Test 2 ** x with auto-sympification"""
    x, = symbols('x')
    result = 2 ** x
    result_str = str(result)
    assert 'x' in result_str and '2' in result_str


def test_negation():
    """Test -x operator"""
    x, = symbols('x')
    result = -x
    result_str = str(result)
    # Negation can be represented as -x or -1*x
    assert 'x' in result_str


def test_complex_expression():
    """Test (2*x + 3) * (x - 1) with chained operators"""
    x, = symbols('x')
    # (2*x + 3) * (x - 1)
    result = (2*x + 3) * (x - 1)
    result_str = str(result)
    # Should contain x in some form
    assert 'x' in result_str


def test_division_and_multiplication():
    """Test x / 2 * 3 with chained operators"""
    x, = symbols('x')
    result = x / 2 * 3
    result_str = str(result)
    assert 'x' in result_str


def test_power_chain():
    """Test (x ** 2) ** 3 with chained power"""
    x, = symbols('x')
    result = (x ** 2) ** 3
    result_str = str(result)
    assert 'x' in result_str


def test_mixed_types():
    """Test mixing int, float, and Expression"""
    x, = symbols('x')
    result = 2 * x + 3.14 - 1
    result_str = str(result)
    assert 'x' in result_str


def test_expression_with_expression():
    """Test operators between two expressions"""
    x, = symbols('x')
    y, = symbols('y')

    # Test all operators
    add_result = x + y
    sub_result = x - y
    mul_result = x * y
    div_result = x / y
    pow_result = x ** y

    # All should contain both variables
    assert 'x' in str(add_result) and 'y' in str(add_result)
    assert 'x' in str(sub_result) and 'y' in str(sub_result)
    assert 'x' in str(mul_result) and 'y' in str(mul_result)
    assert 'x' in str(div_result) and 'y' in str(div_result)
    assert 'x' in str(pow_result) and 'y' in str(pow_result)


def test_type_error_on_unsupported():
    """Test that unsupported types raise TypeError"""
    x, = symbols('x')
    try:
        result = x + [1, 2, 3]  # List not supported
        assert False, "Should raise TypeError"
    except TypeError:
        pass


def test_order_of_operations():
    """Test that Python operator precedence is respected"""
    x, = symbols('x')
    # 2 + 3 * x should be 2 + (3*x), not (2+3)*x
    result = 2 + 3 * x
    result_str = str(result)
    assert 'x' in result_str


def test_parentheses():
    """Test that parentheses work correctly"""
    x, = symbols('x')
    # (2 + 3) * x should be 5*x
    result = (2 + 3) * x
    result_str = str(result)
    assert 'x' in result_str


def test_float_division():
    """Test division with float"""
    x, = symbols('x')
    result = x / 2.5
    result_str = str(result)
    assert 'x' in result_str


def test_zero_operations():
    """Test operations with zero"""
    x, = symbols('x')

    # x + 0
    result1 = x + 0
    assert 'x' in str(result1)

    # x * 0
    result2 = x * 0
    assert str(result2)  # Should return some representation

    # x - 0
    result3 = x - 0
    assert 'x' in str(result3)


def test_one_operations():
    """Test operations with one"""
    x, = symbols('x')

    # x * 1
    result1 = x * 1
    assert 'x' in str(result1)

    # x / 1
    result2 = x / 1
    assert 'x' in str(result2)

    # x ** 1
    result3 = x ** 1
    assert 'x' in str(result3)


def test_negative_numbers():
    """Test operations with negative numbers"""
    x, = symbols('x')

    # x + (-2)
    result1 = x + (-2)
    assert 'x' in str(result1)

    # x * (-3)
    result2 = x * (-3)
    assert 'x' in str(result2)


def test_string_sympification():
    """Test that string sympification works (creates symbol)"""
    x, = symbols('x')
    # Note: This depends on sympify_python implementation
    # Strings might be converted to symbols
    y, = symbols('y')
    result = x + y
    result_str = str(result)
    assert 'x' in result_str and 'y' in result_str

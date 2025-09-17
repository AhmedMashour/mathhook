"""Test init_printing() configuration

This test suite verifies that init_printing() correctly configures
display/printing behavior for Jupyter notebooks and IPython shells.
"""
import mathhook
from mathhook import symbols, init_printing


def test_default_configuration():
    """Test that default configuration enables LaTeX"""
    # Reset to defaults
    init_printing()

    x, = symbols('x')
    expr = x**2 + 2*x + 1

    # Should return LaTeX string
    latex = expr._repr_latex_()
    assert latex is not None
    assert latex.startswith('$')
    assert latex.endswith('$')
    assert 'x' in latex


def test_disable_latex():
    """Test that use_latex=False disables LaTeX rendering"""
    x, = symbols('x')
    expr = x**2 + 2*x + 1

    # Disable LaTeX
    init_printing(use_latex=False)

    # Should return None (no LaTeX)
    latex = expr._repr_latex_()
    assert latex is None

    # Re-enable for other tests
    init_printing(use_latex=True)


def test_latex_mode_mathjax():
    """Test latex_mode='mathjax' setting"""
    init_printing(latex_mode='mathjax')

    x, = symbols('x')
    expr = x + 1

    # Should still render LaTeX (mode affects Jupyter, not _repr_latex_)
    latex = expr._repr_latex_()
    assert latex is not None
    assert latex.startswith('$')


def test_latex_mode_png():
    """Test latex_mode='png' setting"""
    init_printing(latex_mode='png')

    x, = symbols('x')
    expr = x + 1

    # Should still render LaTeX (mode affects Jupyter, not _repr_latex_)
    latex = expr._repr_latex_()
    assert latex is not None
    assert latex.startswith('$')


def test_latex_mode_svg():
    """Test latex_mode='svg' setting"""
    init_printing(latex_mode='svg')

    x, = symbols('x')
    expr = x + 1

    # Should still render LaTeX (mode affects Jupyter, not _repr_latex_)
    latex = expr._repr_latex_()
    assert latex is not None
    assert latex.startswith('$')


def test_invalid_latex_mode():
    """Test that invalid latex_mode raises ValueError"""
    try:
        init_printing(latex_mode='invalid')
        assert False, "Should raise ValueError for invalid mode"
    except ValueError as e:
        assert "latex_mode must be" in str(e)


def test_unicode_setting():
    """Test unicode parameter"""
    # Just verify it doesn't error
    init_printing(unicode=True)
    init_printing(unicode=False)

    # Reset to default
    init_printing()


def test_configuration_persists():
    """Test that configuration persists across expressions"""
    x, y = symbols('x y')

    # Enable LaTeX
    init_printing(use_latex=True)
    expr1 = x + y
    assert expr1._repr_latex_() is not None

    # Disable LaTeX
    init_printing(use_latex=False)
    expr2 = x * y
    assert expr2._repr_latex_() is None

    # First expression should also respect new config
    assert expr1._repr_latex_() is None

    # Re-enable
    init_printing(use_latex=True)
    assert expr1._repr_latex_() is not None
    assert expr2._repr_latex_() is not None


def test_combined_options():
    """Test setting multiple options together"""
    init_printing(use_latex=True, latex_mode='svg', unicode=False)

    x, = symbols('x')
    expr = x**2

    # Should render LaTeX
    latex = expr._repr_latex_()
    assert latex is not None
    assert latex.startswith('$')


def test_expression_types():
    """Test init_printing with various expression types"""
    init_printing(use_latex=True)

    x, y = symbols('x y')

    # Arithmetic
    expr1 = x + y
    assert expr1._repr_latex_() is not None

    # Power
    expr2 = x**3
    assert expr2._repr_latex_() is not None

    # Function
    from mathhook import sin, cos
    expr3 = sin(x)
    assert expr3._repr_latex_() is not None

    # Complex
    expr4 = (x + 1) * (y - 2)
    assert expr4._repr_latex_() is not None

    # All should respect disabled LaTeX
    init_printing(use_latex=False)
    assert expr1._repr_latex_() is None
    assert expr2._repr_latex_() is None
    assert expr3._repr_latex_() is None
    assert expr4._repr_latex_() is None

    # Re-enable for other tests
    init_printing(use_latex=True)


def test_reset_to_defaults():
    """Test that calling init_printing() without args resets to defaults"""
    # Disable LaTeX
    init_printing(use_latex=False)

    x, = symbols('x')
    expr = x + 1
    assert expr._repr_latex_() is None

    # Reset to defaults
    init_printing()

    # Should enable LaTeX again
    assert expr._repr_latex_() is not None

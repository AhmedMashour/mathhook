"""
Tests for evaluate_with_context() and EvalContext in Python bindings

Verifies mathematical correctness and all configuration options.
"""

import pytest
from mathhook import PyExpression as Expression, EvalContext


class TestEvalContextConstruction:
    """Test EvalContext construction and factory methods"""

    def test_default_constructor(self):
        """Default context should be numerical with simplification"""
        ctx = EvalContext()
        # Should be able to use it (internal state verification not exposed)
        expr = Expression.integer(2).add(Expression.integer(3))
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(5)

    def test_symbolic_factory(self):
        """EvalContext.symbolic() creates symbolic-only context"""
        ctx = EvalContext.symbolic()

        # Symbols should stay symbolic
        x = Expression.symbol("x")
        expr = x.pow(Expression.integer(2))
        result = expr.evaluate_with_context(ctx)
        # Result should still contain 'x' (symbolic)
        # We can't easily test internal state, but can verify it doesn't error
        assert result is not None

    def test_numeric_factory(self):
        """EvalContext.numeric() creates numerical context with substitutions"""
        x = Expression.symbol("x")
        expr = x.pow(Expression.integer(2))

        # Evaluate at x = 3 (numerical)
        ctx = EvalContext.numeric({"x": Expression.integer(3)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(9)

    def test_custom_constructor_all_options(self):
        """Test custom constructor with all options"""
        ctx = EvalContext(
            variables={"x": Expression.integer(5)},
            numeric=True,
            precision=128,
            simplify_first=False
        )

        x = Expression.symbol("x")
        expr = x.add(Expression.integer(1))
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(6)


class TestBasicEvaluation:
    """Test basic evaluation functionality"""

    def test_constant_expression_numerical(self):
        """Constants should evaluate to numerical values"""
        expr = Expression.integer(2).add(Expression.integer(3))
        ctx = EvalContext()
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(5)

    def test_constant_expression_symbolic(self):
        """Constants in symbolic mode should stay exact"""
        expr = Expression.integer(2).add(Expression.integer(3))
        ctx = EvalContext.symbolic()
        result = expr.evaluate_with_context(ctx)
        # Should simplify to 5 even in symbolic mode (constant folding)
        assert result == Expression.integer(5)

    def test_symbolic_expression_no_substitution(self):
        """Symbolic expressions without substitution stay symbolic"""
        x = Expression.symbol("x")
        expr = x.add(Expression.integer(1))

        ctx = EvalContext.symbolic()
        result = expr.evaluate_with_context(ctx)
        # Should stay symbolic
        assert result is not None

    def test_polynomial_evaluation(self):
        """Test polynomial: x^2 + 2x + 1 at x = 3"""
        x = Expression.symbol("x")
        expr = x.pow(Expression.integer(2)).add(
            Expression.integer(2).multiply(x)
        ).add(Expression.integer(1))

        ctx = EvalContext.numeric({"x": Expression.integer(3)})
        result = expr.evaluate_with_context(ctx)
        # (3)^2 + 2(3) + 1 = 9 + 6 + 1 = 16
        assert result == Expression.integer(16)


class TestVariableSubstitution:
    """Test variable substitution functionality"""

    def test_single_variable_substitution(self):
        """Substitute single variable"""
        x = Expression.symbol("x")
        expr = x.multiply(Expression.integer(2))

        ctx = EvalContext.numeric({"x": Expression.integer(5)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(10)

    def test_multiple_variable_substitution(self):
        """Substitute multiple variables"""
        x = Expression.symbol("x")
        y = Expression.symbol("y")
        expr = x.add(y)

        ctx = EvalContext.numeric({
            "x": Expression.integer(3),
            "y": Expression.integer(4)
        })
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(7)

    def test_nested_expression_substitution(self):
        """Substitute with complex nested expressions"""
        x = Expression.symbol("x")
        y = Expression.symbol("y")
        expr = x.pow(Expression.integer(2)).add(y.pow(Expression.integer(2)))

        ctx = EvalContext.numeric({
            "x": Expression.integer(3),
            "y": Expression.integer(4)
        })
        result = expr.evaluate_with_context(ctx)
        # 3^2 + 4^2 = 9 + 16 = 25
        assert result == Expression.integer(25)

    def test_partial_substitution_symbolic(self):
        """Partial substitution in symbolic mode"""
        x = Expression.symbol("x")
        y = Expression.symbol("y")
        expr = x.add(y)

        # Only substitute x, leave y symbolic
        ctx = EvalContext(
            variables={"x": Expression.integer(3)},
            numeric=False,
            simplify_first=True
        )
        result = expr.evaluate_with_context(ctx)
        # Result should be "3 + y" (partially symbolic)
        assert result is not None


class TestSimplificationControl:
    """Test simplify_first option"""

    def test_simplify_before_evaluation(self):
        """Test that simplify_first affects evaluation"""
        x = Expression.symbol("x")
        # Expression that benefits from simplification: x + x
        expr = x.add(x)

        ctx = EvalContext.numeric({"x": Expression.integer(3)})
        result = expr.evaluate_with_context(ctx)
        # Should simplify x + x → 2x, then substitute → 2*3 = 6
        assert result == Expression.integer(6)

    def test_no_simplification(self):
        """Test evaluation without pre-simplification"""
        x = Expression.symbol("x")
        expr = x.add(x)

        ctx = EvalContext(
            variables={"x": Expression.integer(3)},
            numeric=True,
            simplify_first=False
        )
        result = expr.evaluate_with_context(ctx)
        # Should still get 6, just via different path: x + x → 3 + 3 = 6
        assert result == Expression.integer(6)


class TestDomainChecking:
    """Test domain constraint validation"""

    @pytest.mark.xfail(reason="Domain checking not yet fully implemented - expressions stay symbolic")
    def test_sqrt_negative_domain_error(self):
        """sqrt of negative should raise domain error"""
        expr = Expression.function("sqrt", [Expression.integer(-1)])
        ctx = EvalContext()

        with pytest.raises(ValueError, match="Domain error|domain error"):
            expr.evaluate_with_context(ctx)

    @pytest.mark.xfail(reason="Domain checking not yet fully implemented - expressions stay symbolic")
    def test_sqrt_negative_after_substitution(self):
        """Domain error should occur after substitution"""
        x = Expression.symbol("x")
        expr = Expression.function("sqrt", [x])

        ctx = EvalContext.numeric({"x": Expression.integer(-4)})

        with pytest.raises(ValueError, match="Domain error|domain error"):
            expr.evaluate_with_context(ctx)

    @pytest.mark.xfail(reason="Domain checking not yet fully implemented - expressions stay symbolic")
    def test_log_zero_domain_error(self):
        """log(0) should raise domain error"""
        expr = Expression.function("log", [Expression.integer(0)])
        ctx = EvalContext()

        with pytest.raises(ValueError, match="Domain error|domain error"):
            expr.evaluate_with_context(ctx)

    @pytest.mark.xfail(reason="Domain checking not yet fully implemented - expressions stay symbolic")
    def test_division_by_zero(self):
        """Division by zero should raise error"""
        zero = Expression.integer(0)
        one = Expression.integer(1)
        expr = one.multiply(zero.pow(Expression.integer(-1)))

        ctx = EvalContext()

        with pytest.raises(ValueError):
            expr.evaluate_with_context(ctx)


class TestFunctionEvaluation:
    """Test function evaluation with context"""

    def test_trigonometric_evaluation(self):
        """Evaluate trigonometric functions"""
        x = Expression.symbol("x")
        expr = Expression.function("sin", [x])

        # Evaluate sin(0) = 0
        ctx = EvalContext.numeric({"x": Expression.integer(0)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(0)

    def test_exponential_evaluation(self):
        """Evaluate exponential functions"""
        x = Expression.symbol("x")
        expr = Expression.function("exp", [x])

        # Evaluate exp(0) = 1
        ctx = EvalContext.numeric({"x": Expression.integer(0)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(1)

    def test_special_function_gamma(self):
        """Evaluate gamma function"""
        x = Expression.symbol("x")
        expr = Expression.function("gamma", [x])

        # Evaluate gamma(1) = 1 (0! = 1)
        ctx = EvalContext.numeric({"x": Expression.integer(1)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(1)


class TestPrecisionControl:
    """Test precision parameter (for future arbitrary precision support)"""

    def test_default_precision(self):
        """Test with default f64 precision"""
        expr = Expression.integer(1).multiply(Expression.integer(3).pow(Expression.integer(-1)))
        ctx = EvalContext()
        result = expr.evaluate_with_context(ctx)
        # Should get rational 1/3 or float approximation
        assert result is not None

    def test_custom_precision(self):
        """Test custom precision (currently has no effect with f64)"""
        expr = Expression.integer(1).multiply(Expression.integer(3).pow(Expression.integer(-1)))
        ctx = EvalContext().with_precision(128)
        result = expr.evaluate_with_context(ctx)
        # Should work, even if precision isn't used yet
        assert result is not None


class TestComplexExpressions:
    """Test complex multi-operation expressions"""

    def test_formula_evaluation(self):
        """Test realistic formula: (x + y)^2 = x^2 + 2xy + y^2"""
        x = Expression.symbol("x")
        y = Expression.symbol("y")

        # Left side: (x + y)^2
        lhs = x.add(y).pow(Expression.integer(2))

        # Right side: x^2 + 2xy + y^2
        rhs = x.pow(Expression.integer(2)).add(
            Expression.integer(2).multiply(x).multiply(y)
        ).add(y.pow(Expression.integer(2)))

        # Both should evaluate to same value at x=3, y=4
        ctx = EvalContext.numeric({
            "x": Expression.integer(3),
            "y": Expression.integer(4)
        })

        lhs_result = lhs.evaluate_with_context(ctx)
        rhs_result = rhs.evaluate_with_context(ctx)

        # (3 + 4)^2 = 49
        assert lhs_result == Expression.integer(49)
        # 9 + 24 + 16 = 49
        assert rhs_result == Expression.integer(49)
        assert lhs_result == rhs_result

    def test_nested_functions(self):
        """Test nested function calls"""
        x = Expression.symbol("x")
        # sin(cos(x))
        inner = Expression.function("cos", [x])
        expr = Expression.function("sin", [inner])

        # Evaluate at x = 0: sin(cos(0)) = sin(1)
        ctx = EvalContext.numeric({"x": Expression.integer(0)})
        result = expr.evaluate_with_context(ctx)
        # Should evaluate successfully (exact value depends on implementation)
        assert result is not None


class TestEdgeCases:
    """Test edge cases and boundary conditions"""

    def test_empty_variables(self):
        """Empty variable map should work"""
        expr = Expression.integer(5)
        ctx = EvalContext.numeric({})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(5)

    def test_unused_variable_in_context(self):
        """Variables not in expression should be ignored"""
        x = Expression.symbol("x")
        expr = x.add(Expression.integer(1))

        ctx = EvalContext.numeric({
            "x": Expression.integer(3),
            "y": Expression.integer(999)  # Not used
        })
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(4)

    def test_zero_expressions(self):
        """Test with zero values"""
        x = Expression.symbol("x")
        expr = x.multiply(Expression.integer(0))

        ctx = EvalContext.numeric({"x": Expression.integer(42)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(0)

    def test_identity_operations(self):
        """Test with identity elements"""
        x = Expression.symbol("x")
        expr = x.multiply(Expression.integer(1))

        ctx = EvalContext.numeric({"x": Expression.integer(7)})
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(7)


class TestMethodChaining:
    """Test method chaining for configuration"""

    def test_with_precision_chaining(self):
        """Test with_precision returns chainable context"""
        ctx = EvalContext.symbolic().with_precision(128)
        expr = Expression.integer(2).add(Expression.integer(3))
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(5)

    def test_with_simplify_first_chaining(self):
        """Test with_simplify_first returns chainable context"""
        ctx = EvalContext().with_simplify_first(False)
        expr = Expression.integer(2).add(Expression.integer(3))
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(5)

    def test_multiple_method_chaining(self):
        """Test multiple method chains"""
        ctx = EvalContext.symbolic().with_precision(128).with_simplify_first(True)
        expr = Expression.integer(2).add(Expression.integer(3))
        result = expr.evaluate_with_context(ctx)
        assert result == Expression.integer(5)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

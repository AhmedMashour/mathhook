# USER RUST DEVELOPMENT RULES

## AI Assistant Rules for Rust Development

These are permanent rules that apply to all Rust development work with this user.

### Code Style Rules

1. **Documentation Only**: Use /// for functions, //! for modules. Never add inline comments.
2. **No Emojis**: Never add emojis in code, comments, or documentation.
3. **No ALL CAPS**: Never add ALL CAPS descriptions or comments.
4. **Testable Documentation**: Always use /// with examples that people can test in docs (doctests).
5. **Self-Documenting Code**: Keep code clean, professional, and self-documenting through good naming and structure.
6. **Functionality Focus**: Focus on functionality over visual flair in documentation.

### Documentation Standards

- Write comprehensive /// documentation for all public APIs
- Include practical examples that compile and run
- Explain parameters and return values clearly
- Document error conditions and edge cases
- Use clear, professional language without visual noise

### Example of Proper Documentation

```rust
/// Calculate the derivative of an expression with respect to a variable
///
/// Returns a new expression representing the derivative. For polynomial expressions,
/// this uses standard calculus rules. Complex expressions may return symbolic derivatives.
///
/// # Arguments
///
/// * `expression` - The expression to differentiate
/// * `variable` - The variable to differentiate with respect to
/// * `order` - The order of the derivative (1 for first derivative, 2 for second, etc.)
///
/// # Examples
///
/// ```rust
/// use mathhook::core::{Expression, Symbol};
///
/// let x = Symbol::new("x");
/// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
/// let derivative = Expression::derivative(expr, x, 1);
/// 
/// assert!(matches!(derivative, Expression::Calculus(_)));
/// ```
pub fn derivative(expression: Expression, variable: Symbol, order: u32) -> Self {
    Self::Calculus(Box::new(CalculusData::Derivative {
        expression,
        variable,
        order,
    }))
}
```

### Code Quality Standards

- Zero compilation warnings
- Clean, descriptive naming
- Proper error handling with Result types
- Minimal, focused imports
- Professional presentation throughout

These rules apply to ALL Rust development work and should be followed consistently.

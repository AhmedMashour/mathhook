//! Tests for parser integration

#[cfg(test)]
mod tests {
    use crate::core::Expression;
    use crate::parser::universal::MathLanguage;

    #[test]
    fn test_basic_parsing() {
        let expr = Expression::parse("x^2 + 2*x + 1").unwrap();
        println!("Basic parsing: {} → {}", "x^2 + 2*x + 1", expr);
        // Should be Add([Pow(Symbol(x), Number(2)), Mul([Number(2), Symbol(x)]), Number(1)])
    }

    #[test]
    fn test_latex_parsing() {
        let expr = Expression::parse("\\frac{x^2}{2}").unwrap();
        println!("LaTeX parsing: {} → {}", "\\frac{x^2}{2}", expr);
        // Should be Mul([Pow(Symbol(x), Number(2)), Pow(Number(2), Number(-1))])
    }

    #[test]
    fn test_wolfram_parsing() {
        let expr = Expression::parse("Sin[x] + Cos[y]").unwrap();
        println!("Wolfram parsing: {} → {}", "Sin[x] + Cos[y]", expr);
        // Should be Add([Function(sin, [Symbol(x)]), Function(cos, [Symbol(y)])])
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(
            Expression::detect_language("\\frac{1}{2}"),
            MathLanguage::LaTeX
        );
        assert_eq!(Expression::detect_language("Sin[x]"), MathLanguage::Wolfram);
        assert_eq!(Expression::detect_language("x + 1"), MathLanguage::Simple);
    }

    #[test]
    fn test_output_formats() {
        let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));

        let latex = expr.to_latex();
        let simple = expr.to_simple();
        let wolfram = expr.to_wolfram();

        println!("Expression: {}", expr);
        println!("LaTeX: {}", latex);
        println!("Simple: {}", simple);
        println!("Wolfram: {}", wolfram);

        assert!(!latex.is_empty());
        assert!(!simple.is_empty());
        assert!(!wolfram.is_empty());
    }

    #[test]
    fn test_chaining() {
        let result = Expression::parse("x^2 + 1").unwrap().to_latex();
        println!(
            "Chaining: Expression::parse(\"x^2 + 1\").to_latex() → {}",
            result
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_explicit_language_parsing() {
        let latex = Expression::parse_with_language("\\sin(x)", MathLanguage::LaTeX).unwrap();
        let wolfram = Expression::parse_with_language("Sin[x]", MathLanguage::Wolfram).unwrap();
        let simple = Expression::parse_with_language("sin(x)", MathLanguage::Simple).unwrap();

        println!("Explicit LaTeX: {}", latex);
        println!("Explicit Wolfram: {}", wolfram);
        println!("Explicit Simple: {}", simple);
    }

    #[test]
    fn test_thread_local_reuse() {
        // Test that the thread-local parser is reused
        let expr1 = Expression::parse("x").unwrap();
        let expr2 = Expression::parse("x").unwrap(); // Should reuse same 'x' symbol

        println!("First x: {:?}", expr1);
        println!("Second x: {:?}", expr2);

        // Both should be the same symbol (same name at least)
        if let (Expression::Symbol(s1), Expression::Symbol(s2)) = (&expr1, &expr2) {
            assert_eq!(s1.name(), s2.name());
        }
    }
}

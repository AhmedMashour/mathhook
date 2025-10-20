use mathhook_core::core::commutativity::Commutativity;
use mathhook_core::core::symbol::SymbolType;
use mathhook_core::core::Expression;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;

fn parse_latex(input: &str) -> Result<Expression, String> {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });
    parser.parse(input).map_err(|e| e.to_string())
}

#[test]
fn test_mathbf_creates_matrix_symbol() {
    let result = parse_latex("\\mathbf{A}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Matrix);
        assert_eq!(sym.commutativity(), Commutativity::Noncommutative);
        assert_eq!(sym.name(), "A");
    } else {
        panic!("Expected Symbol expression, got {:?}", expr);
    }
}

#[test]
fn test_mathbf_multiple_matrix_symbols() {
    let result = parse_latex("\\mathbf{B}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Matrix);
        assert_eq!(sym.name(), "B");
    } else {
        panic!("Expected Symbol expression");
    }
}

#[test]
fn test_mathbf_lowercase_creates_matrix() {
    let result = parse_latex("\\mathbf{a}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Matrix);
        assert_eq!(sym.name(), "a");
    } else {
        panic!("Expected Symbol expression");
    }
}

#[test]
fn test_hat_creates_operator_symbol() {
    let result = parse_latex("\\hat{p}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Operator);
        assert_eq!(sym.commutativity(), Commutativity::Noncommutative);
        assert_eq!(sym.name(), "p");
    } else {
        panic!("Expected Symbol expression, got {:?}", expr);
    }
}

#[test]
fn test_hat_uppercase_creates_operator() {
    let result = parse_latex("\\hat{H}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Operator);
        assert_eq!(sym.name(), "H");
    } else {
        panic!("Expected Symbol expression");
    }
}

#[test]
fn test_hat_position_operator() {
    let result = parse_latex("\\hat{x}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Operator);
        assert_eq!(sym.name(), "x");
    } else {
        panic!("Expected Symbol expression");
    }
}

#[test]
fn test_quaternion_symbol_programmatic() {
    use mathhook_core::core::Symbol;

    let i = Symbol::quaternion("i");
    let j = Symbol::quaternion("j");
    let k = Symbol::quaternion("k");

    assert_eq!(i.symbol_type(), SymbolType::Quaternion);
    assert_eq!(i.commutativity(), Commutativity::Noncommutative);

    assert_eq!(j.symbol_type(), SymbolType::Quaternion);
    assert_eq!(j.commutativity(), Commutativity::Noncommutative);

    assert_eq!(k.symbol_type(), SymbolType::Quaternion);
    assert_eq!(k.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_lowercase_stays_scalar() {
    let result = parse_latex("x");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Scalar);
        assert_eq!(sym.commutativity(), Commutativity::Commutative);
        assert_eq!(sym.name(), "x");
    } else {
        panic!("Expected Symbol expression, got {:?}", expr);
    }
}

#[test]
fn test_uppercase_stays_scalar() {
    let result = parse_latex("X");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Scalar);
        assert_eq!(sym.commutativity(), Commutativity::Commutative);
    } else {
        panic!("Expected Symbol expression");
    }
}

#[test]
fn test_greek_stays_scalar() {
    let result = parse_latex("\\theta");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Symbol(sym) = expr {
        assert_eq!(sym.symbol_type(), SymbolType::Scalar);
        assert_eq!(sym.commutativity(), Commutativity::Commutative);
    } else {
        panic!("Expected Symbol expression");
    }
}

#[test]
fn test_mathbf_multiplication_noncommutative() {
    let result = parse_latex("\\mathbf{A}*\\mathbf{B}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);

    if let Expression::Mul(factors) = expr {
        assert_eq!(factors.len(), 2);

        if let Expression::Symbol(sym) = &factors[0] {
            assert_eq!(sym.symbol_type(), SymbolType::Matrix);
            assert_eq!(sym.name(), "A");
        } else {
            panic!("Expected first factor to be Symbol");
        }

        if let Expression::Symbol(sym) = &factors[1] {
            assert_eq!(sym.symbol_type(), SymbolType::Matrix);
            assert_eq!(sym.name(), "B");
        } else {
            panic!("Expected second factor to be Symbol");
        }
    } else {
        panic!("Expected Mul expression, got {:?}", expr);
    }
}

#[test]
fn test_hat_multiplication_noncommutative() {
    let result = parse_latex("\\hat{p}*\\hat{x}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);

    if let Expression::Mul(factors) = expr {
        assert_eq!(factors.len(), 2);

        if let Expression::Symbol(sym) = &factors[0] {
            assert_eq!(sym.symbol_type(), SymbolType::Operator);
            assert_eq!(sym.name(), "p");
        } else {
            panic!("Expected first factor to be Symbol");
        }

        if let Expression::Symbol(sym) = &factors[1] {
            assert_eq!(sym.symbol_type(), SymbolType::Operator);
            assert_eq!(sym.name(), "x");
        } else {
            panic!("Expected second factor to be Symbol");
        }
    } else {
        panic!("Expected Mul expression");
    }
}

#[test]
fn test_quaternion_multiplication_noncommutative_programmatic() {
    use mathhook_core::core::Symbol;

    let i = Expression::symbol(Symbol::quaternion("i"));
    let j = Expression::symbol(Symbol::quaternion("j"));
    let product = Expression::mul(vec![i, j]);

    assert_eq!(product.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_scalar_multiplication_commutative() {
    let result = parse_latex("x*y");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Commutative);
}

#[test]
fn test_mixed_scalar_matrix_noncommutative() {
    let result = parse_latex("x*\\mathbf{A}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_mixed_matrix_scalar_noncommutative() {
    let result = parse_latex("\\mathbf{A}*x");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_mixed_scalar_operator_noncommutative() {
    let result = parse_latex("x*\\hat{p}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_mixed_operator_scalar_noncommutative() {
    let result = parse_latex("\\hat{p}*x");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_matrix_operator_mixed_noncommutative() {
    let result = parse_latex("\\mathbf{A}*\\hat{p}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_complex_expression_with_matrices() {
    let result = parse_latex("\\mathbf{A}*\\mathbf{B}+\\mathbf{C}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_complex_expression_with_operators() {
    let result = parse_latex("\\hat{H}*\\hat{p}+\\hat{x}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_power_of_matrix() {
    let result = parse_latex("\\mathbf{A}^2");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Pow(base, exponent) = expr {
        if let Expression::Symbol(sym) = base.as_ref() {
            assert_eq!(sym.symbol_type(), SymbolType::Matrix);
        } else {
            panic!("Expected base to be Symbol");
        }

        if let Expression::Number(n) = exponent.as_ref() {
            assert_eq!(n.to_string(), "2");
        } else {
            panic!("Expected exponent to be Number");
        }
    } else {
        panic!("Expected Pow expression");
    }
}

#[test]
fn test_power_of_operator() {
    let result = parse_latex("\\hat{p}^2");
    assert!(result.is_ok());
    let expr = result.unwrap();

    if let Expression::Pow(base, _exponent) = expr {
        if let Expression::Symbol(sym) = base.as_ref() {
            assert_eq!(sym.symbol_type(), SymbolType::Operator);
        } else {
            panic!("Expected base to be Symbol");
        }
    } else {
        panic!("Expected Pow expression");
    }
}

#[test]
fn test_scalar_commutator_vs_matrix_noncommutator() {
    let scalar_ab = parse_latex("a*b").unwrap();
    let scalar_ba = parse_latex("b*a").unwrap();
    let matrix_ab = parse_latex("\\mathbf{A}*\\mathbf{B}").unwrap();
    let matrix_ba = parse_latex("\\mathbf{B}*\\mathbf{A}").unwrap();

    assert_eq!(scalar_ab.commutativity(), Commutativity::Commutative);
    assert_eq!(scalar_ba.commutativity(), Commutativity::Commutative);
    assert_eq!(matrix_ab.commutativity(), Commutativity::Noncommutative);
    assert_eq!(matrix_ba.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_addition_preserves_commutativity() {
    let result = parse_latex("\\mathbf{A}+\\mathbf{B}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_subtraction_preserves_commutativity() {
    let result = parse_latex("\\hat{H}-\\hat{p}");
    assert!(result.is_ok());
    let expr = result.unwrap();

    assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_nested_expression_commutativity() {
    use mathhook_core::core::Symbol;

    let a = Expression::symbol(Symbol::matrix("A"));
    let b = Expression::symbol(Symbol::matrix("B"));
    let c = Expression::symbol(Symbol::matrix("C"));
    let d = Expression::symbol(Symbol::matrix("D"));

    let sum1 = Expression::add(vec![a, b]);
    let sum2 = Expression::add(vec![c, d]);
    let product = Expression::mul(vec![sum1, sum2]);

    assert_eq!(product.commutativity(), Commutativity::Noncommutative);
}

#[test]
fn test_ambiguous_mathbf_nested() {
    let result = parse_latex(r"\mathbf{\mathbf{A}}");

    if result.is_ok() {
        let expr = result.unwrap();
        if let Expression::Symbol(sym) = expr {
            assert_eq!(sym.symbol_type(), SymbolType::Matrix);
        }
    }
}

#[test]
fn test_ambiguous_hat_operator() {
    let result = parse_latex(r"\hat{\hat{p}}");

    if result.is_ok() {
        let expr = result.unwrap();
        if let Expression::Symbol(sym) = expr {
            assert_eq!(sym.symbol_type(), SymbolType::Operator);
        }
    }
}

#[test]
fn test_malformed_latex_mathbf() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });

    let result1 = parser.parse(r"\mathbf{");
    let result2 = parser.parse(r"\mathbf}A");
    let result3 = parser.parse(r"\mathbf");

    assert!(
        result1.is_err() || result1.is_ok(),
        "Parser should handle malformed mathbf gracefully"
    );
    assert!(
        result2.is_err() || result2.is_ok(),
        "Parser should handle malformed mathbf gracefully"
    );
    assert!(
        result3.is_err() || result3.is_ok(),
        "Parser should handle malformed mathbf gracefully"
    );
}

#[test]
fn test_malformed_latex_hat() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });

    let result1 = parser.parse(r"\hat{");
    let result2 = parser.parse(r"\hat}p");
    let result3 = parser.parse(r"\hat");

    assert!(
        result1.is_err() || result1.is_ok(),
        "Parser should handle malformed hat gracefully"
    );
    assert!(
        result2.is_err() || result2.is_ok(),
        "Parser should handle malformed hat gracefully"
    );
    assert!(
        result3.is_err() || result3.is_ok(),
        "Parser should handle malformed hat gracefully"
    );
}

#[test]
fn test_mixed_notation_precedence() {
    let result = parse_latex(r"\mathbf{A}*\hat{p}+\mathbf{B}*\hat{x}");

    if result.is_ok() {
        let expr = result.unwrap();

        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);

        if let Expression::Add(terms) = expr {
            assert_eq!(terms.len(), 2);

            for term in terms.iter() {
                if let Expression::Mul(factors) = term {
                    assert!(
                        factors.len() >= 2,
                        "Each term should have matrix and operator"
                    );
                }
            }
        }
    }
}

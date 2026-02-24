//! Functions module for MathHook Node.js bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.
use crate::generated::symbol::JsSymbol;
use crate::JsExpression;
use mathhook_core::algebra::groebner::{GroebnerBasis, MonomialOrder};
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use mathhook_core::{Expression, Symbol};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// A wrapper type that accepts Expression, number, or string from JavaScript.
/// This follows a Pattern for automatic type conversion.
///
/// Accepts:
/// - `Expression` object → use directly
/// - `number` (integer or float) → convert to Expression::integer/float
/// - `string` → convert to Expression::symbol (creates symbol on the fly)
///
/// # Examples (JavaScript)
/// ```javascript
/// sin(x)      // Expression object
/// sin(1)      // integer → Expression::integer(1)
/// sin(3.14)   // float → Expression::float(3.14)
/// sin('y')    // string → Expression::symbol('y')
/// ```
pub struct ExpressionOrNumber(pub Expression);

impl FromNapiValue for ExpressionOrNumber {
    unsafe fn from_napi_value(
        env: napi::sys::napi_env,
        value: napi::sys::napi_value,
    ) -> Result<Self> {
        // Get the type of the value
        let mut value_type = 0;
        napi::sys::napi_typeof(env, value, &mut value_type);

        if value_type == napi::sys::ValueType::napi_number {
            // Number: convert to integer or float
            let num: f64 = f64::from_napi_value(env, value)?;
            let expr = if num.fract() == 0.0 && num.is_finite() && num.abs() < i64::MAX as f64 {
                Expression::integer(num as i64)
            } else {
                Expression::float(num)
            };
            Ok(ExpressionOrNumber(expr))
        } else if value_type == napi::sys::ValueType::napi_string {
            // String: convert to symbol (like SymPy's sympify)
            let s: String = String::from_napi_value(env, value)?;
            Ok(ExpressionOrNumber(Expression::symbol(Symbol::new(&s))))
        } else if value_type == napi::sys::ValueType::napi_object {
            // Object: try to unwrap as JsExpression
            match ClassInstance::<JsExpression>::from_napi_value(env, value) {
                Ok(instance) => Ok(ExpressionOrNumber(instance.inner.clone())),
                Err(_) => Err(Error::new(
                    Status::InvalidArg,
                    "Expected Expression object, number, or string".to_string(),
                )),
            }
        } else {
            Err(Error::new(
                Status::InvalidArg,
                "Expected Expression, number, or string".to_string(),
            ))
        }
    }
}

/// A wrapper type that accepts Symbol or Expression (containing a symbol) from JavaScript.
/// This enables seamless use of both `symbol("x")` (returns Expression) and `new Symbol("x")`
/// in functions that need a Symbol.
///
/// Accepts:
/// - `Symbol` object → use directly
/// - `Expression` that is a symbol → extract the inner Symbol
/// - `string` → create a Symbol from the string
///
/// # Examples (JavaScript)
/// ```javascript
/// const x = symbol("x");           // Returns Expression
/// const y = new Symbol("y");       // Returns Symbol
///
/// // Both work with derivative:
/// expr.derivative(x);  // Works - extracts Symbol from Expression
/// expr.derivative(y);  // Works - uses Symbol directly
/// ```
pub struct SymbolOrExpression(pub Symbol);

impl FromNapiValue for SymbolOrExpression {
    unsafe fn from_napi_value(
        env: napi::sys::napi_env,
        value: napi::sys::napi_value,
    ) -> Result<Self> {
        // Get the type of the value
        let mut value_type = 0;
        napi::sys::napi_typeof(env, value, &mut value_type);

        if value_type == napi::sys::ValueType::napi_string {
            // String: create a Symbol from it
            let s: String = String::from_napi_value(env, value)?;
            Ok(SymbolOrExpression(Symbol::new(&s)))
        } else if value_type == napi::sys::ValueType::napi_object {
            // Object: try JsSymbol first, then JsExpression
            if let Ok(sym_instance) = ClassInstance::<JsSymbol>::from_napi_value(env, value) {
                return Ok(SymbolOrExpression(sym_instance.inner.clone()));
            }

            if let Ok(expr_instance) = ClassInstance::<JsExpression>::from_napi_value(env, value) {
                // Try to extract Symbol from Expression
                if let Some(sym) = expr_instance.inner.as_symbol() {
                    return Ok(SymbolOrExpression(sym.clone()));
                } else {
                    return Err(Error::new(
                        Status::InvalidArg,
                        "Expression is not a symbol. Use symbol('x') or new Symbol('x')"
                            .to_string(),
                    ));
                }
            }

            Err(Error::new(
                Status::InvalidArg,
                "Expected Symbol or Expression containing a symbol".to_string(),
            ))
        } else {
            Err(Error::new(
                Status::InvalidArg,
                "Expected Symbol, Expression (containing symbol), or string".to_string(),
            ))
        }
    }
}

/// Compute Gröbner basis for a system of polynomials
///
/// A Gröbner basis is a special generating set for a polynomial ideal that
/// has useful computational properties, analogous to row echelon form for matrices.
///
/// # Arguments
///
/// * `polynomials` - Array of polynomial expressions
/// * `variables` - Array of variable names
/// * `order` - Monomial ordering: "lex" (lexicographic), "grlex" (graded lex), or "grevlex" (graded reverse lex)
///
/// # Examples
///
/// ```javascript
/// const x = JsExpression.symbol("x");
/// const y = JsExpression.symbol("y");
/// const p1 = x.pow(JsExpression.integer(2)).add(y.pow(JsExpression.integer(2))).subtract(JsExpression.integer(1));
/// const p2 = x.subtract(y);
/// const basis = groebnerBasis([p1, p2], ["x", "y"], "lex");
/// // Returns Gröbner basis for the ideal generated by p1 and p2
/// ```
#[napi]
pub fn groebner_basis(
    polynomials: Vec<&JsExpression>,
    variables: Vec<String>,
    order: String,
) -> Result<Vec<JsExpression>> {
    let polys: Vec<Expression> = polynomials.iter().map(|p| p.inner.clone()).collect();
    let vars: Vec<Symbol> = variables.iter().map(|v| Symbol::new(v.clone())).collect();
    let monomial_order = match order.as_str() {
        "lex" => MonomialOrder::Lex,
        "grlex" => MonomialOrder::Grlex,
        "grevlex" => MonomialOrder::Grevlex,
        _ => {
            return Err(Error::new(
                Status::InvalidArg,
                format!(
                    "Invalid monomial order: {}. Use 'lex', 'grlex', or 'grevlex'",
                    order
                ),
            ));
        }
    };
    let mut gb = GroebnerBasis::new(polys, vars, monomial_order);
    gb.compute();
    Ok(gb
        .basis
        .into_iter()
        .map(|expr| JsExpression { inner: expr })
        .collect())
}
/// Create a single symbol
///
/// Creates a symbolic variable for use in mathematical expressions.
/// This is the primary way to create variables in MathHook.
///
/// # Arguments
///
/// * `name` - Name of the symbol (e.g., "x", "y", "theta")
///
/// # Returns
///
/// A JsExpression representing the symbol
///
/// # Example
///
/// ```javascript
/// const { symbol } = require('mathhook-node');
///
/// // Create a single symbol
/// const x = symbol('x');
/// const y = symbol('y');
///
/// // Use in expressions
/// const expr = x.pow(2).add(y);
/// console.log(expr.toSimple());  // "x^2 + y"
///
/// // Greek letters
/// const theta = symbol('θ');
/// const alpha = symbol('alpha');
/// ```
#[napi]
pub fn symbol(name: String) -> JsExpression {
    JsExpression {
        inner: Expression::symbol(Symbol::new(&name)),
    }
}

/// Create multiple symbols at once from a string specification
///
/// Supports three input formats:
/// - Space-separated: `"x y z"` → [x, y, z]
/// - Comma-separated: `"a, b, c"` or `"a,b,c"` → [a, b, c]
/// - Range syntax: `"x0:3"` → [x0, x1, x2]
///
/// # Arguments
///
/// * `names` - String containing symbol names in one of the supported formats
///
/// # Returns
///
/// Array of JsExpression symbols
///
/// # Examples
///
/// ```javascript
/// // Space-separated
/// const [x, y, z] = symbols('x y z');
///
/// // Comma-separated
/// const [a, b, c] = symbols('a, b, c');
///
/// // Range syntax
/// const [x0, x1, x2] = symbols('x0:3');
///
/// // Use in expressions
/// const expr = x.add(y).multiply(z);
/// ```
#[napi]
pub fn symbols(names: String) -> Result<Vec<JsExpression>> {
    if names.contains(':') {
        let parts: Vec<&str> = names.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::new(
                Status::InvalidArg,
                "Range syntax must be 'prefix:count' (e.g., 'x0:3')",
            ));
        }
        let prefix = parts[0];
        let end: usize = parts[1].parse().map_err(|_| {
            Error::new(
                Status::InvalidArg,
                format!("Range end '{}' must be a number", parts[1]),
            )
        })?;
        let num_start = prefix
            .chars()
            .position(|c| c.is_numeric())
            .unwrap_or(prefix.len());
        let base = &prefix[..num_start];
        let start: usize = if num_start < prefix.len() {
            prefix[num_start..].parse().unwrap_or(0)
        } else {
            0
        };
        let mut result = Vec::new();
        for i in start..end {
            let name = format!("{}{}", base, i);
            result.push(JsExpression {
                inner: Expression::symbol(Symbol::new(&name)),
            });
        }
        return Ok(result);
    }
    let separator = if names.contains(',') { ',' } else { ' ' };
    let symbol_names: Vec<&str> = names
        .split(separator)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if symbol_names.is_empty() {
        return Err(Error::new(
            Status::InvalidArg,
            "No symbol names provided - input string is empty or contains only whitespace",
        ));
    }
    Ok(symbol_names
        .iter()
        .map(|name| JsExpression {
            inner: Expression::symbol(Symbol::new(name)),
        })
        .collect())
}
mathhook_macros::generate_nodejs_binding!(sin);
mathhook_macros::generate_nodejs_binding!(cos);
mathhook_macros::generate_nodejs_binding!(tan);
mathhook_macros::generate_nodejs_binding!(asin);
mathhook_macros::generate_nodejs_binding!(acos);
mathhook_macros::generate_nodejs_binding!(atan);
mathhook_macros::generate_nodejs_binding!(sinh);
mathhook_macros::generate_nodejs_binding!(cosh);
mathhook_macros::generate_nodejs_binding!(tanh);
mathhook_macros::generate_nodejs_binding!(exp);
mathhook_macros::generate_nodejs_binding!(ln);
mathhook_macros::generate_nodejs_binding!(log10);
/// Square root function
///
/// Uses Expression::pow(expr, 1/2) internally to match SymPy's representation.
///
/// # Arguments
/// * `x` - Expression or number to evaluate
///
/// # Examples
/// ```javascript
/// const { sqrt, symbol } = require('mathhook');
/// const x = symbol('x');
/// const expr = sqrt(x);  // √x (represented as x^(1/2))
/// const num = sqrt(4);   // √4
/// ```
#[napi]
pub fn sqrt(x: ExpressionOrNumber) -> JsExpression {
    JsExpression {
        inner: Expression::function("sqrt", vec![x.0]),
    }
}
mathhook_macros::generate_nodejs_binding!(abs);
mathhook_macros::generate_nodejs_binding!(sign);
mathhook_macros::generate_nodejs_binding!(floor);
mathhook_macros::generate_nodejs_binding!(ceil);
mathhook_macros::generate_nodejs_binding!(round);
mathhook_macros::generate_nodejs_binding!(gamma);
mathhook_macros::generate_nodejs_binding!(factorial);
mathhook_macros::generate_nodejs_binding!(digamma);
mathhook_macros::generate_nodejs_binding!(zeta);
mathhook_macros::generate_nodejs_binding!(erf);
mathhook_macros::generate_nodejs_binding!(erfc);
mathhook_macros::generate_nodejs_binding!(isprime);
mathhook_macros::generate_nodejs_binary_binding!(gcd);
mathhook_macros::generate_nodejs_binary_binding!(lcm);
mathhook_macros::generate_nodejs_binary_binding!(modulo);
mathhook_macros::generate_nodejs_binary_binding!(polygamma);
mathhook_macros::generate_nodejs_binary_binding!(bessel_j);
mathhook_macros::generate_nodejs_binary_binding!(bessel_y);
mathhook_macros::generate_nodejs_binary_binding!(beta);
/// Get polynomial degree with respect to a variable
///
/// # Arguments
///
/// * `poly` - Polynomial expression
/// * `variable` - Variable name to check degree for
///
/// # Returns
///
/// Degree as integer expression or symbolic
///
/// # Examples
///
/// ```javascript
/// const { degree, symbols, parse } = require('mathhook');
///
/// const [x] = symbols('x');
/// const poly = parse('x^3 + 2*x^2 + x + 1');
/// const deg = degree(poly, 'x');  // Returns 3
/// ```
#[napi]
pub fn degree(poly: &JsExpression, variable: String) -> JsExpression {
    use mathhook_core::functions::polynomials::polynomial_eval;
    use mathhook_core::Symbol;
    let var_symbol = Symbol::new(&variable);
    JsExpression {
        inner: polynomial_eval::degree(&poly.inner, &var_symbol),
    }
}
/// Find polynomial roots with respect to a variable
///
/// # Arguments
///
/// * `poly` - Polynomial expression
/// * `variable` - Variable name to solve for
///
/// # Returns
///
/// Set of roots or symbolic expression
///
/// # Examples
///
/// ```javascript
/// const { roots, symbols, parse } = require('mathhook');
///
/// const [x] = symbols('x');
/// const poly = parse('x^2 - 1');
/// const r = roots(poly, 'x');  // Returns roots of quadratic
/// ```
#[napi]
pub fn roots(poly: &JsExpression, variable: String) -> JsExpression {
    use mathhook_core::functions::polynomials::polynomial_eval;
    use mathhook_core::Symbol;
    let var_symbol = Symbol::new(&variable);
    JsExpression {
        inner: polynomial_eval::roots(&poly.inner, &var_symbol),
    }
}
/// Parse a mathematical expression from a string
///
/// Supports multiple input formats with auto-detection:
/// - **Standard notation**: `x^2 + 2*x + 1`
/// - **LaTeX notation**: `\frac{x^2}{2} + \sin(x)`
/// - **Wolfram notation**: `Sin[x] + Cos[y]`
/// - **Implicit multiplication**: `2x`, `(a)(b)`, `2(x+1)`
/// - **Functions**: sin, cos, tan, exp, log, sqrt, and all special functions
/// - **Greek letters**: alpha, beta, gamma, theta, pi, etc.
/// - **Constants**: pi, e, i (imaginary unit)
///
/// # Arguments
/// * `expression` - Mathematical expression string in any supported format
///
/// # Returns
/// Parsed Expression object ready for manipulation
///
/// # Errors
/// Returns error if the expression cannot be parsed
///
/// # Examples
/// ```javascript
/// const { parse } = require('mathhook');
///
/// // Basic arithmetic
/// const expr1 = parse('x^2 + 2*x + 1');
///
/// // Implicit multiplication
/// const expr2 = parse('2x + 3y');  // Same as '2*x + 3*y'
///
/// // Functions
/// const expr3 = parse('sin(x) + cos(y)');
///
/// // LaTeX (auto-detected)
/// const expr4 = parse('\\frac{x^2}{2}');
///
/// // Wolfram notation (auto-detected)
/// const expr5 = parse('Sin[x] + Cos[y]');
///
/// // Greek letters
/// const expr6 = parse('alpha + beta');
///
/// // Complex expressions
/// const expr7 = parse('sin(2*pi*x) + exp(-x^2/2)');
/// ```
#[napi]
pub fn parse(expression: String) -> Result<JsExpression> {
    let parser = Parser::new(&ParserConfig::default());
    match parser.parse(&expression) {
        Ok(expr) => Ok(JsExpression { inner: expr }),
        Err(e) => Err(Error::new(
            Status::InvalidArg,
            format!("Parse error: {}", e),
        )),
    }
}

//! Node.js bindings for MathHook
//!
//! This crate provides Node.js bindings using NAPI-RS, exposing the hybrid API
//! for JavaScript/TypeScript users with both Expression-centric and object-oriented interfaces.

use mathhook_core::{parser::universal::MathLanguage, Expression, MathSolver, Simplify, Symbol};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// JavaScript wrapper for Expression
#[napi]
pub struct JsExpression {
    inner: Expression,
}

#[napi]
impl JsExpression {
    /// Create a new expression from an integer
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.integer(42);
    /// ```
    #[napi(factory)]
    pub fn integer(value: i64) -> Self {
        Self {
            inner: Expression::integer(value),
        }
    }

    /// Create a new expression from a symbol
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const alpha = JsExpression.symbol("α");
    /// ```
    #[napi(factory)]
    pub fn symbol(name: String) -> Self {
        Self {
            inner: Expression::symbol(name),
        }
    }

    /// Add two expressions
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const two = JsExpression.integer(2);
    /// const result = x.add(two);
    /// ```
    #[napi]
    pub fn add(&self, other: &JsExpression) -> JsExpression {
        JsExpression {
            inner: Expression::add(vec![self.inner.clone(), other.inner.clone()]),
        }
    }

    /// Multiply two expressions
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const three = JsExpression.integer(3);
    /// const result = x.multiply(three);
    /// ```
    #[napi]
    pub fn multiply(&self, other: &JsExpression) -> JsExpression {
        JsExpression {
            inner: Expression::mul(vec![self.inner.clone(), other.inner.clone()]),
        }
    }

    /// Raise expression to a power
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const two = JsExpression.integer(2);
    /// const xSquared = x.pow(two);
    /// ```
    #[napi]
    pub fn pow(&self, exponent: &JsExpression) -> JsExpression {
        JsExpression {
            inner: Expression::pow(self.inner.clone(), exponent.inner.clone()),
        }
    }

    /// Simplify the expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.integer(2).add(JsExpression.integer(3));
    /// const simplified = expr.simplify();
    /// ```
    #[napi]
    pub fn simplify(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.clone().simplify(),
        }
    }

    /// Parse a mathematical expression from string with automatic language detection
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr1 = JsExpression.parse("2*x + sin(y)");
    /// const expr2 = JsExpression.parse("\\frac{x^2}{2}");    // LaTeX auto-detected
    /// const expr3 = JsExpression.parse("Sin[x] + Cos[y]");   // Wolfram auto-detected
    /// ```
    #[napi(factory)]
    pub fn parse(input: String) -> Result<JsExpression> {
        match Expression::parse(&input) {
            Ok(expr) => Ok(JsExpression { inner: expr }),
            Err(e) => Err(Error::new(
                Status::InvalidArg,
                format!("Parse error: {}", e),
            )),
        }
    }

    /// Parse with explicit language specification
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const latex = JsExpression.parseWithLanguage("\\sin(x)", "latex");
    /// const wolfram = JsExpression.parseWithLanguage("Sin[x]", "wolfram");
    /// const simple = JsExpression.parseWithLanguage("sin(x)", "simple");
    /// ```
    #[napi(factory)]
    pub fn parse_with_language(input: String, language: String) -> Result<JsExpression> {
        let lang = match language.as_str() {
            "latex" => MathLanguage::LaTeX,
            "wolfram" => MathLanguage::Wolfram,
            "simple" => MathLanguage::Simple,
            _ => MathLanguage::Simple,
        };

        match Expression::parse_with_language(&input, lang) {
            Ok(expr) => Ok(JsExpression { inner: expr }),
            Err(e) => Err(Error::new(
                Status::InvalidArg,
                format!("Parse error: {}", e),
            )),
        }
    }

    /// Convert expression to LaTeX format
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.symbol("x").pow(JsExpression.integer(2));
    /// const latex = expr.toLatex();  // Returns "x^{2}"
    /// ```
    #[napi]
    pub fn to_latex(&self) -> String {
        self.inner.to_latex()
    }

    /// Convert expression to simple mathematical notation
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.symbol("x").pow(JsExpression.integer(2));
    /// const simple = expr.toSimple();  // Returns "x^2"
    /// ```
    #[napi]
    pub fn to_simple(&self) -> String {
        self.inner.to_simple()
    }

    /// Convert expression to Wolfram Language format
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.function("sin", [JsExpression.symbol("x")]);
    /// const wolfram = expr.toWolfram();  // Returns "Sin[x]"
    /// ```
    #[napi]
    pub fn to_wolfram(&self) -> String {
        self.inner.to_wolfram()
    }

    /// Create an equation (equality relation)
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const five = JsExpression.integer(5);
    /// const equation = JsExpression.equation(x, five);
    /// ```
    #[napi(factory)]
    pub fn equation(left: &JsExpression, right: &JsExpression) -> JsExpression {
        JsExpression {
            inner: Expression::equation(left.inner.clone(), right.inner.clone()),
        }
    }

    /// String representation
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// console.log(x.toString());
    /// ```
    #[napi]
    pub fn to_string(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// JavaScript wrapper for MathSolver
#[napi]
pub struct JsMathSolver {
    inner: MathSolver,
}

#[napi]
impl JsMathSolver {
    /// Create a new solver
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const solver = new JsMathSolver();
    /// ```
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: MathSolver::new(),
        }
    }

    /// Solve an equation
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const solver = new JsMathSolver();
    /// const x = JsExpression.symbol("x");
    /// const five = JsExpression.integer(5);
    /// const equation = JsExpression.equation(x, five);
    /// const result = solver.solve(equation, "x");
    /// ```
    #[napi]
    pub fn solve(&mut self, equation: &JsExpression, variable: String) -> String {
        let symbol = Symbol::new(variable);
        let result = self.inner.solve(&equation.inner, &symbol);
        format!("{:?}", result)
    }
}

//! Code generation macros for MathHook language bindings and expression construction
//!
//! This crate provides procedural macros to generate Python (PyO3) and Node.js (NAPI)
//! bindings from a single function definition, eliminating code duplication while
//! maintaining zero-cost abstractions.
//!
//! Also provides procedural macros for mathematical expression and symbol construction.
//!
//! # Supported Function Arities
//!
//! - **Unary**: sin(x), cos(x), exp(x)
//! - **Binary**: pow(x, y), atan2(y, x)
//! - **Variadic**: add(args...), mul(args...)
//! - **Constants**: pi(), e()

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

mod expr;
mod function;
mod symbol;

/// Procedural macro for creating mathematical expressions with full syntax support
///
/// # Supported Operators
///
/// - **Basic arithmetic**: `+`, `-`, `*`, `/`
/// - **Unary negation**: `-x`
/// - **Power operations**: Both `**` and `.pow()` syntax
/// - **Comparison operators**: `==`, `<`, `>`, `<=`, `>=`
///
/// # Supported Literals
///
/// - **Integers**: `42`, `-5`, `0`
/// - **Floats**: `3.14`, `-2.5`, `0.0`
/// - **Identifiers/symbols**: `x`, `theta`, `alpha_1`
/// - **Mathematical constants**: `pi`, `e`, `i`
///
/// # Supported Constructs
///
/// - **Function calls**: `sin(x)`, `log(x, y)`, `f(a, b, c)`
/// - **Parenthesized expressions**: `(2*x + 3)`, `((x+y))`
/// - **Method calls**: `x.pow(2)`, `x.abs()`, `x.sqrt()`, `x.simplify()`
///
/// # Power Operator Syntax
///
/// The macro supports **two syntaxes** for exponentiation:
///
/// ## Method Syntax: `.pow()`
///
/// The `.pow()` method works everywhere without restrictions:
///
/// ```rust,ignore
/// expr!(x.pow(2))              // x squared
/// expr!(x.pow(y))              // x to the y
/// expr!((x + 1).pow(3))        // (x+1) cubed
/// expr!(2 * x.pow(3) + 5)      // Complex expression
/// ```
///
/// **Advantages:**
/// - Mirrors Rust's standard library API
/// - Works seamlessly in any context
/// - Clear, unambiguous precedence
/// - No parentheses required
///
/// ## Infix Syntax: `**`
///
/// The `**` operator provides mathematical notation:
///
/// ```rust,ignore
/// expr!(x ** 2)                // x squared
/// expr!(x ** y)                // x to the y
/// expr!((x + 1) ** 3)          // (x+1) cubed
/// ```
///
/// **IMPORTANT - When to use parentheses with `**`:**
///
/// Due to token-level preprocessing, complex expressions with `**` should use
/// parentheses for clarity when mixing with other operators:
///
/// ```rust,ignore
/// // ✅ RECOMMENDED (clear precedence):
/// expr!(2 * (x ** 3) + 5)      // Parentheses make precedence explicit
/// expr!((x ** 2) + (y ** 2))   // Clear grouping
///
/// // ⚠️  WORKS BUT LESS CLEAR:
/// expr!(x ** 2 + y ** 2)       // Relies on implicit precedence
/// ```
///
/// **Right-associativity:**
///
/// Power operations are right-associative (like standard mathematical notation):
///
/// ```rust,ignore
/// expr!(x ** 2 ** 3)           // Parsed as: x ** (2 ** 3) = x ** 8
/// expr!(2 ** 3 ** 2)           // Parsed as: 2 ** (3 ** 2) = 2 ** 9 = 512
/// ```
///
/// **Which syntax to use?**
///
/// - Use `.pow()` for guaranteed clarity in complex expressions
/// - Use `**` when writing simple mathematical formulas that match notation
/// - When in doubt, add parentheses: `expr!(2 * (x ** 3) + 5)`
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_macros::expr;
/// use mathhook_core::{Expression, symbol};
///
/// let x = symbol!(x);
/// let y = symbol!(y);
///
/// // Basic operations
/// let sum = expr!(x + 2);
/// let product = expr!(2 * x);
///
/// // Power operations (both syntaxes work)
/// let power1 = expr!(x ** 2);
/// let power2 = expr!(x.pow(2));
///
/// // Complex expressions (use parentheses with ** for clarity)
/// let quadratic = expr!((x ** 2) + 2*x + 1);
/// let nested = expr!((x + 1) * (x - 1));
/// let functions = expr!(sin(x ** 2));
///
/// // Comparison operators
/// let equation = expr!(x ** 2 == 4);
/// let inequality = expr!(x + 1 > y);
///
/// // Method calls
/// let abs_val = expr!(x.abs());
/// let sqrt_expr = expr!(x.sqrt());
/// let simplified = expr!((x + x).simplify());
/// ```
///
/// # Precedence Rules
///
/// Operator precedence from highest to lowest:
///
/// 1. **Method calls** (highest): `.pow()`, `.abs()`, `.sqrt()`, `.simplify()`
/// 2. **Power** (right-associative): `**`
/// 3. **Unary negation**: `-x`
/// 4. **Multiplication/division**: `*`, `/`
/// 5. **Addition/subtraction**: `+`, `-`
/// 6. **Comparison operators** (lowest): `==`, `<`, `>`, `<=`, `>=`
///
/// Use parentheses to override precedence:
///
/// ```rust,ignore
/// expr!(2 * x + 3)             // Parsed as: (2*x) + 3
/// expr!((2 + 3) * x)           // Parentheses override: 5*x
/// expr!(2 * (x ** 3))          // Recommended for clarity with **
/// expr!((x + 1) ** 2)          // Power of sum
/// ```
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    expr::expr_impl(input)
}

/// Procedural macro for creating symbols with optional type specification
///
/// # Syntax
///
/// ```rust,ignore
/// symbol!(x)                  // Scalar (default, commutative)
/// symbol!(A; matrix)          // Matrix (noncommutative)
/// symbol!(p; operator)        // Operator (noncommutative)
/// symbol!(i; quaternion)      // Quaternion (noncommutative)
/// symbol!("name")             // String literal for symbol name
/// ```
///
/// # Symbol Types
///
/// - **scalar**: Commutative symbols (default) - variables like x, y, z
/// - **matrix**: Noncommutative matrix symbols - A*B ≠ B*A
/// - **operator**: Noncommutative operator symbols - for quantum mechanics [x,p] ≠ 0
/// - **quaternion**: Noncommutative quaternion symbols - i*j = k, j*i = -k
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_macros::symbol;
///
/// // Scalar symbols (commutative)
/// let x = symbol!(x);
/// let theta = symbol!(theta);
///
/// // Matrix symbols (noncommutative)
/// let A = symbol!(A; matrix);
/// let B = symbol!(B; matrix);
///
/// // Operator symbols (noncommutative)
/// let p = symbol!(p; operator);
/// let x_op = symbol!(x; operator);
///
/// // Quaternion symbols (noncommutative)
/// let i = symbol!(i; quaternion);
/// let j = symbol!(j; quaternion);
/// ```
#[proc_macro]
pub fn symbol(input: TokenStream) -> TokenStream {
    symbol::symbol_impl(input)
}

/// Procedural macro for creating multiple symbols at once
///
/// # Syntax
///
/// ```rust,ignore
/// symbols![x, y, z]              // All scalars (default)
/// symbols![A, B, C => matrix]    // All matrices
/// symbols![p, x, H => operator]  // All operators
/// symbols![i, j, k => quaternion] // All quaternions
/// ```
///
/// # Returns
///
/// Returns `Vec<Symbol>` containing all created symbols.
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_macros::symbols;
///
/// // Scalar symbols (default, commutative)
/// let syms = symbols![x, y, z];
/// assert_eq!(syms.len(), 3);
///
/// // Matrix symbols (noncommutative)
/// let mats = symbols![A, B, C => matrix];
/// assert_eq!(mats.len(), 3);
///
/// // Operator symbols (noncommutative)
/// let ops = symbols![p, x, H => operator];
/// assert_eq!(ops.len(), 3);
///
/// // Quaternion symbols (noncommutative)
/// let quats = symbols![i, j, k => quaternion];
/// assert_eq!(quats.len(), 3);
/// ```
#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
    symbol::symbols_impl(input)
}

/// Procedural macro for creating function expressions
///
/// # Syntax
///
/// ```rust,ignore
/// function!(sin)              // Zero args
/// function!(sin, x)           // One arg (x is an Expression)
/// function!(log, x, y)        // Two args
/// function!(f, a, b, c)       // N args
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_macros::function;
/// use mathhook_core::expr;
///
/// // Zero-argument function
/// let gamma_call = function!(gamma);
///
/// // Single argument
/// let x = expr!(x);
/// let sin_x = function!(sin, x);
///
/// // Multiple arguments
/// let log_xy = function!(log, x, expr!(2));
/// ```
#[proc_macro]
pub fn function(input: TokenStream) -> TokenStream {
    function::function_impl(input)
}

/// Generate Python (PyO3) binding for a unary mathematical function
///
/// # Arguments
///
/// * `name` - The function name (e.g., `sin`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_python_binding!(sin);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[pyfunction]
/// pub fn sin(x: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
///     let expr = sympify_python(x)?;
///     Ok(PyExpression {
///         inner: Expression::function("sin", vec![expr]),
///     })
/// }
/// ```
#[proc_macro]
pub fn generate_python_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();

    let expanded = quote! {
        #[pyo3::pyfunction]
        pub fn #name(x: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<crate::PyExpression> {
            use mathhook_core::Expression;
            let expr = crate::helpers::sympify_python(x)?;
            Ok(crate::PyExpression {
                inner: Expression::function(#name_str, vec![expr]),
            })
        }
    };

    TokenStream::from(expanded)
}

/// Generate Python (PyO3) binding for a binary mathematical function
///
/// # Arguments
///
/// * `name` - The function name (e.g., `pow`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_python_binary_binding!(pow);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[pyfunction]
/// pub fn pow(x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
///     let expr1 = sympify_python(x)?;
///     let expr2 = sympify_python(y)?;
///     Ok(PyExpression {
///         inner: Expression::function("pow", vec![expr1, expr2]),
///     })
/// }
/// ```
#[proc_macro]
pub fn generate_python_binary_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();

    let expanded = quote! {
        #[pyo3::pyfunction]
        pub fn #name(
            x: &pyo3::Bound<'_, pyo3::PyAny>,
            y: &pyo3::Bound<'_, pyo3::PyAny>,
        ) -> pyo3::PyResult<crate::PyExpression> {
            use mathhook_core::Expression;
            let expr1 = crate::helpers::sympify_python(x)?;
            let expr2 = crate::helpers::sympify_python(y)?;
            Ok(crate::PyExpression {
                inner: Expression::function(#name_str, vec![expr1, expr2]),
            })
        }
    };

    TokenStream::from(expanded)
}

/// Generate Python (PyO3) binding for a variadic mathematical function
///
/// # Arguments
///
/// * `name` - The function name (e.g., `add`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_python_variadic_binding!(add);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[pyfunction]
/// #[pyo3(signature = (*args))]
/// pub fn add(args: &Bound<'_, PyTuple>) -> PyResult<PyExpression> {
///     let mut exprs = Vec::new();
///     for arg in args {
///         exprs.push(sympify_python(&arg)?);
///     }
///     Ok(PyExpression {
///         inner: Expression::function("add", exprs),
///     })
/// }
/// ```
#[proc_macro]
pub fn generate_python_variadic_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();

    let expanded = quote! {
        #[pyo3::pyfunction]
        #[pyo3(signature = (*args))]
        pub fn #name(args: &pyo3::Bound<'_, pyo3::types::PyTuple>) -> pyo3::PyResult<crate::PyExpression> {
            use mathhook_core::Expression;
            let mut exprs = Vec::new();
            for arg in args {
                exprs.push(crate::helpers::sympify_python(&arg)?);
            }
            Ok(crate::PyExpression {
                inner: Expression::function(#name_str, exprs),
            })
        }
    };

    TokenStream::from(expanded)
}

/// Generate Python (PyO3) binding for a zero-argument constant function
///
/// # Arguments
///
/// * `name` - The constant name (e.g., `pi`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_python_constant_binding!(pi);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[pyfunction]
/// pub fn pi() -> PyExpression {
///     PyExpression {
///         inner: Expression::pi(),
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_python_constant_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        #[pyo3::pyfunction]
        pub fn #name() -> crate::PyExpression {
            use mathhook_core::Expression;
            crate::PyExpression {
                inner: Expression::#name(),
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate Node.js (NAPI) binding for a unary mathematical function
///
/// # Arguments
///
/// * `name` - The function name (e.g., `sin`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_nodejs_binding!(sin);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[napi]
/// pub fn sin(x: napi::Either<&JsExpression, f64>) -> JsExpression {
///     let expr = match x {
///         napi::Either::A(e) => e.inner.clone(),
///         napi::Either::B(num) => {
///             if num.fract() == 0.0 && num.is_finite() {
///                 Expression::integer(num as i64)
///             } else {
///                 Expression::float(num)
///             }
///         }
///     };
///     JsExpression {
///         inner: Expression::function("sin", vec![expr]),
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_nodejs_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();

    let expanded = quote! {
        #[napi_derive::napi]
        pub fn #name(x: napi::bindgen_prelude::Either<&crate::JsExpression, f64>) -> crate::JsExpression {
            use mathhook_core::Expression;

            let expr = match x {
                napi::bindgen_prelude::Either::A(e) => e.inner.clone(),
                napi::bindgen_prelude::Either::B(num) => {
                    if num.fract() == 0.0 && num.is_finite() {
                        Expression::integer(num as i64)
                    } else {
                        Expression::float(num)
                    }
                }
            };

            crate::JsExpression {
                inner: Expression::function(#name_str, vec![expr]),
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate Node.js (NAPI) binding for a binary mathematical function
///
/// # Arguments
///
/// * `name` - The function name (e.g., `pow`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_nodejs_binary_binding!(pow);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[napi]
/// pub fn pow(
///     x: napi::Either<&JsExpression, f64>,
///     y: napi::Either<&JsExpression, f64>
/// ) -> JsExpression {
///     let expr1 = match x { /* ... */ };
///     let expr2 = match y { /* ... */ };
///     JsExpression {
///         inner: Expression::function("pow", vec![expr1, expr2]),
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_nodejs_binary_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();

    let expanded = quote! {
        #[napi_derive::napi]
        pub fn #name(
            x: napi::bindgen_prelude::Either<&crate::JsExpression, f64>,
            y: napi::bindgen_prelude::Either<&crate::JsExpression, f64>,
        ) -> crate::JsExpression {
            use mathhook_core::Expression;

            let expr1 = match x {
                napi::bindgen_prelude::Either::A(e) => e.inner.clone(),
                napi::bindgen_prelude::Either::B(num) => {
                    if num.fract() == 0.0 && num.is_finite() {
                        Expression::integer(num as i64)
                    } else {
                        Expression::float(num)
                    }
                }
            };

            let expr2 = match y {
                napi::bindgen_prelude::Either::A(e) => e.inner.clone(),
                napi::bindgen_prelude::Either::B(num) => {
                    if num.fract() == 0.0 && num.is_finite() {
                        Expression::integer(num as i64)
                    } else {
                        Expression::float(num)
                    }
                }
            };

            crate::JsExpression {
                inner: Expression::function(#name_str, vec![expr1, expr2]),
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate Node.js (NAPI) binding for a variadic mathematical function
///
/// # Arguments
///
/// * `name` - The function name (e.g., `add`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_nodejs_variadic_binding!(add);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[napi]
/// pub fn add(args: Vec<napi::Either<&JsExpression, f64>>) -> JsExpression {
///     let exprs: Vec<Expression> = args
///         .into_iter()
///         .map(|x| match x { /* ... */ })
///         .collect();
///     JsExpression {
///         inner: Expression::function("add", exprs),
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_nodejs_variadic_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();

    let expanded = quote! {
        #[napi_derive::napi]
        pub fn #name(args: Vec<napi::bindgen_prelude::Either<&crate::JsExpression, f64>>) -> crate::JsExpression {
            use mathhook_core::Expression;

            let exprs: Vec<Expression> = args
                .into_iter()
                .map(|x| match x {
                    napi::bindgen_prelude::Either::A(e) => e.inner.clone(),
                    napi::bindgen_prelude::Either::B(num) => {
                        if num.fract() == 0.0 && num.is_finite() {
                            Expression::integer(num as i64)
                        } else {
                            Expression::float(num)
                        }
                    }
                })
                .collect();

            crate::JsExpression {
                inner: Expression::function(#name_str, exprs),
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate Node.js (NAPI) binding for a zero-argument constant function
///
/// # Arguments
///
/// * `name` - The constant name (e.g., `pi`)
///
/// # Examples
///
/// ```rust,ignore
/// generate_nodejs_constant_binding!(pi);
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// #[napi]
/// pub fn pi() -> JsExpression {
///     JsExpression {
///         inner: Expression::pi(),
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_nodejs_constant_binding(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        #[napi_derive::napi]
        pub fn #name() -> crate::JsExpression {
            use mathhook_core::Expression;
            crate::JsExpression {
                inner: Expression::#name(),
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros_module_compiles() {
        println!("All macro definitions compiled successfully");
    }
}

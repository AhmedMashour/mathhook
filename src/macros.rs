//! Mathematical expression macros
//!
//! This module provides ergonomic macros for creating, parsing, and manipulating mathematical expressions.
//! Choose the right tool for your complexity level:
//!
//! ## Quick Start Guide
//!
//! ### Simple Expressions - Use `expr!`
//! ```rust
//! use mathhook::expr;
//!
//! let x = expr!(x);           // Symbols
//! let sum = expr!(x + 1);     // Simple operations  
//! let func = expr!(sin(x));   // Functions
//! ```
//!
//! ### Just Need Symbols - Use `symbol!`
//! ```rust
//! use mathhook::symbol;
//!
//! let x = symbol!(x);                    // Single symbol
//! let (a, b, c) = symbol!(a, b, c);      // Multiple symbols
//! let alpha = symbol!("α");              // Unicode names
//! ```
//!
//! ### Mathematical Constants - Use `const_expr!`
//! ```rust
//! use mathhook::const_expr;
//!
//! let pi = const_expr!(pi);           // π
//! let e = const_expr!(e);             // Euler's number
//! let i = const_expr!(i);             // Imaginary unit
//! ```
//!
//! ### Complex Expressions - Use `parse!`
//! ```rust
//! use mathhook::parse;
//!
//! let complex = parse!("x^2 + 2*x + 1").unwrap();           // Polynomials
//! let nested = parse!("sin(x^2) / (x + 1)").unwrap();       // Nested functions
//! let multi = parse!("a*cos(x) + b*sin(x) + c").unwrap();   // Multiple terms
//! ```
//!
//! ### Advanced Calculus - Use `calculus!`
//! ```rust
//! use mathhook::{expr, calculus};
//!
//! let f = expr!(x^2);
//! let derivative = calculus!(derivative: f, x);      // df/dx
//! let integral = calculus!(integral: f, x);          // ∫f dx
//! ```
//!
//! ## Macro Categories
//!
//! - [`simple`]: Basic expression creation (`expr!`, `const_expr!`, `symbol!`)
//! - [`parsing`]: Format parsing and conversion (`parse!`, `to_format!`)  
//! - [`calculus`]: Calculus operations (`calculus!`)
//!
//! ## Complexity Decision Tree
//!
//! ```text
//! Your Need
//! │
//! ├─ Just creating symbols (x, y, z)?
//! │  └─ Use symbol!(x) or symbol!(x, y, z)
//! │
//! ├─ Single operation (x + 1, sin(x), x^2)?
//! │  └─ Use expr!(x + 1)
//! │
//! ├─ Mathematical constant (π, e, i)?
//! │  └─ Use const_expr!(pi)
//! │
//! ├─ Complex string expression?
//! │  └─ Use parse!("x^2 + 2*x + 1")
//! │
//! ├─ Calculus operation?
//! │  └─ Use calculus!(derivative: f, x)
//! │
//! └─ Multiple operations but want type safety?
//!    └─ Compose: let result = expr!(x) + expr!(y) * expr!(z);
//! ```

#[path = "macros/calculus.rs"]
pub mod calculus;

#[path = "macros/parsing.rs"]
pub mod parsing;

#[path = "macros/simple.rs"]
pub mod simple;

// Re-export all macros for convenient access
pub use calculus::*;
pub use parsing::*;
pub use simple::*;

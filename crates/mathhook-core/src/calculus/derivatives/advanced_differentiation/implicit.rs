//! Implicit differentiation for equations and relations
//!
//! Handles differentiation of implicitly defined functions where y is defined
//! implicitly as a function of x through an equation F(x,y) = 0.
//!
//! # Module Organization
//!
//! This module is split into focused sub-modules to maintain the 500-line file size limit:
//!
//! - `differentiation` - Core implicit differentiation operations
//! - `curve_analysis` - Critical point analysis for implicit curves
//!
//! # Examples
//!
//! ## Computing dy/dx for Implicit Curves
//!
//! ```rust
//! use mathhook_core::{expr, symbol};
//! use mathhook_core::calculus::derivatives::ImplicitDifferentiation;
//!
//! let x = symbol!(x);
//! let y = symbol!(y);
//! // For x² + y² = 1 (circle)
//! let equation = expr!((x^2) + (y^2));
//! let dy_dx = ImplicitDifferentiation::compute(&equation, x, y);
//! ```
//!
//! ## Finding Critical Points
//!
//! ```rust
//! use mathhook_core::{expr, symbol, Expression};
//! use mathhook_core::calculus::derivatives::ImplicitCurveAnalysis;
//!
//! let x = symbol!(x);
//! let y = symbol!(y);
//! let curve = Expression::add(vec![expr!(x^1), expr!(y^2), expr!(-1)]);
//! let critical_points = ImplicitCurveAnalysis::critical_points(&curve, x, y);
//! ```

mod curve_analysis;
mod differentiation;

// Re-export public API
pub use curve_analysis::ImplicitCurveAnalysis;
pub use differentiation::ImplicitDifferentiation;

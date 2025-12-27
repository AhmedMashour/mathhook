//! Core mathematical engine for MathHook
//!
//! This crate provides the foundational types and operations for symbolic mathematics,
//! implementing a hybrid API architecture optimized for both performance and usability.

extern crate self as mathhook_core;

pub mod algebra;
pub mod calculus;
pub mod core;
pub mod educational;
pub mod error;
pub mod formatter;
pub mod functions;
pub mod macros;
pub mod matrices;
pub mod parser;
pub mod pattern;
pub mod serialize;
pub mod simplify;
pub mod solvers;

// Curated re-exports (no wildcards for maintainability)

// Macro re-exports (at crate root for backward compatibility)
pub use mathhook_macros::{expr, function, symbol, symbols};

// Core types (most commonly used)
pub use core::expression::eval_numeric::{EvalContext, EvalNumeric};
pub use core::{
    Commutativity, Expression, MathConstant, Number, NumericMatrix, Symbol, SymbolType,
};

// Algebra traits and key types
pub use algebra::{multivariate_gcd, polynomial_div, polynomial_quo, polynomial_rem};
pub use algebra::{
    AdvancedPolynomial, AdvancedSimplify, Collect, ComplexOperations, EquationAnalyzer,
    EquationType, Expand, Factor, PolynomialGcd, RationalSimplify, SmartEquationSolver,
    ZeroDetection,
};

// Calculus operations
pub use calculus::derivatives::Derivative;
pub use calculus::{
    BasicDerivatives, BasicIntegrals, ChainRule, ComplexAnalysis, FunctionIntegrals,
    HigherOrderDerivatives, Integration, IntegrationMethods, LimitDirection, Limits, PowerRule,
    ProductRule, ResidueCalculus, SeriesExpansion, SeriesType, Summation, SummationMethods,
};

// Solvers
pub use solvers::{MathSolver, SolverConfig, SolverResult};

// Pattern matching and substitution
pub use pattern::{Matchable, Pattern, PatternMatches, Substitutable};

// Parser types (commonly needed)
pub use parser::config::ParserConfig;
pub use parser::error::ParseError;
pub use parser::Parser;

// Error types
pub use error::MathError;

// Formatter types
pub use formatter::{LaTeXFormatter, MathLanguage, SimpleFormatter, WolframFormatter};

// Functions (re-export function modules for users who want specific functions)
pub use functions::elementary;
pub use functions::polynomials;
pub use functions::special;

// Simplify operations
pub use simplify::Simplify;

/// Convenience prelude for common imports
///
/// This prelude provides everything needed for typical symbolic mathematics operations.
/// Import with `use mathhook_core::prelude::*;` to access all common types and traits.
pub mod prelude {
    pub use crate::macros::*;
    pub use crate::{expr, function, symbol, symbols};

    // Core types
    pub use crate::{
        Commutativity, Expression, MathConstant, Number, NumericMatrix, Symbol, SymbolType,
    };

    // Algebra traits
    pub use crate::{
        AdvancedPolynomial, AdvancedSimplify, Collect, ComplexOperations, EquationAnalyzer,
        EquationType, Expand, Factor, PolynomialGcd, RationalSimplify, Simplify,
        SmartEquationSolver, ZeroDetection,
    };

    // Algebra functions
    pub use crate::{multivariate_gcd, polynomial_div, polynomial_quo, polynomial_rem};

    // Calculus traits (comprehensive)
    pub use crate::Derivative;
    pub use crate::{
        BasicDerivatives, BasicIntegrals, ChainRule, ComplexAnalysis, FunctionIntegrals,
        HigherOrderDerivatives, Integration, IntegrationMethods, LimitDirection, Limits, PowerRule,
        ProductRule, ResidueCalculus, SeriesExpansion, SeriesType, Summation, SummationMethods,
    };

    // Pattern matching and substitution
    pub use crate::{Matchable, Pattern, PatternMatches, Substitutable};

    // Educational features
    pub use crate::educational::{
        DifficultyLevel, EducationalExt, EducationalOperation, EducationalResult, EnhancedStep,
        EnhancedStepExplanation, OperationContext, Step, StepByStep, StepByStepExplanation,
    };

    // Matrix operations
    pub use crate::matrices::{
        CoreMatrixOps, EigenOperations, Matrix, MatrixDecomposition, MatrixOperations,
    };

    // Solver
    pub use crate::{MathSolver, SolverConfig, SolverResult};

    // Parser
    pub use crate::{Parser, ParserConfig};

    // Error types
    pub use crate::{MathError, ParseError};

    // Formatters
    pub use crate::{LaTeXFormatter, MathLanguage, SimpleFormatter, WolframFormatter};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_memory_optimization() {
        let expression_size = std::mem::size_of::<Expression>();
        println!("Clean Expression size: {} bytes", expression_size);

        assert!(
            expression_size <= 32,
            "Expression should be ≤ 32 bytes, got {} bytes",
            expression_size
        );
    }

    #[test]
    fn test_hybrid_api_basic_operations() {
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Number(num) => {
                if let Number::Integer(i) = num {
                    assert_eq!(i, 5);
                } else {
                    panic!("Expected integer result");
                }
            }
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_solver_object_api() {
        let solver = MathSolver::new();

        let equation = Expression::equation(expr!(x), expr!(42));

        let result = solver.solve(&equation, &symbol!(x));

        match result {
            SolverResult::Single(solution) => match solution {
                Expression::Number(num) => {
                    if let Number::Integer(i) = num {
                        assert_eq!(i, 42);
                    } else {
                        panic!("Expected integer solution");
                    }
                }
                _ => panic!("Expected number solution"),
            },
            _ => panic!("Expected single solution"),
        }
    }

    #[test]
    fn test_hot_path_performance() {
        let start = std::time::Instant::now();

        for _ in 0..10000 {
            let expr = expr!(1 + x + 2);
            let _simplified = expr.simplify();
        }

        let duration = start.elapsed();
        println!("10K hot-path operations took: {:?}", duration);

        assert!(
            duration.as_millis() < 100,
            "Hot path too slow: {:?}",
            duration
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_procedural_symbol_macro() {
        let x = symbol!(x);
        assert_eq!(x.name(), "x");
        assert_eq!(x.commutativity(), Commutativity::Commutative);

        let A = symbol!(A; matrix);
        assert_eq!(A.name(), "A");
        assert_eq!(A.commutativity(), Commutativity::Noncommutative);

        let p = symbol!(p; operator);
        assert_eq!(p.name(), "p");
        assert_eq!(p.commutativity(), Commutativity::Noncommutative);

        let i = symbol!(i; quaternion);
        assert_eq!(i.name(), "i");
        assert_eq!(i.commutativity(), Commutativity::Noncommutative);

        let syms = symbols![x, y, z];
        assert_eq!(syms.len(), 3);
        assert_eq!(syms[0].name(), "x");
        assert_eq!(syms[1].name(), "y");
        assert_eq!(syms[2].name(), "z");
        assert_eq!(syms[0].commutativity(), Commutativity::Commutative);

        let mats = symbols![A, B => matrix];
        assert_eq!(mats.len(), 2);
        assert_eq!(mats[0].name(), "A");
        assert_eq!(mats[1].name(), "B");
        assert_eq!(mats[0].commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_procedural_function_macro() {
        let gamma_call = function!(gamma);
        if let Expression::Function { name, args } = &gamma_call {
            assert_eq!(name, "gamma");
            assert_eq!(args.len(), 0);
        } else {
            panic!("Expected Function expression");
        }

        let x = expr!(x);
        let sin_x = function!(sin, x.clone());
        if let Expression::Function { name, args } = &sin_x {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected Function expression");
        }

        let y = expr!(y);
        let log_xy = function!(log, x, y);
        if let Expression::Function { name, args } = &log_xy {
            assert_eq!(name, "log");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected Function expression");
        }
    }
}

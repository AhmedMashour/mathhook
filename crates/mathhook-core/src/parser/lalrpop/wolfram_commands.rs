use once_cell::sync::Lazy;
/// Dynamic Wolfram Language Command Parser
///
/// Efficiently parses Wolfram Language functions at runtime instead of compile-time,
/// dramatically reducing LALRPOP grammar compilation time while preserving all functionality.
use std::collections::HashMap;

/// Wolfram function categories for efficient parsing
#[derive(Debug, Clone, PartialEq)]
pub enum WolframFunctionType {
    // Basic mathematical functions
    Arithmetic,
    Trigonometric,
    Logarithmic,
    SpecialFunction,

    // Calculus and analysis
    Calculus,
    Limit,

    // Algebra and number theory
    Polynomial,
    NumberTheory,

    // Linear algebra and matrices
    Matrix,
    LinearAlgebra,

    // Complex numbers
    Complex,

    // Logic and control
    Logic,
    Control,

    // Unknown function
    Unknown,
}

/// Efficient Wolfram function lookup table
static WOLFRAM_FUNCTIONS: Lazy<HashMap<&'static str, WolframFunctionType>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Basic arithmetic and mathematical functions
    map.insert("Plus", WolframFunctionType::Arithmetic);
    map.insert("Subtract", WolframFunctionType::Arithmetic);
    map.insert("Times", WolframFunctionType::Arithmetic);
    map.insert("Divide", WolframFunctionType::Arithmetic);
    map.insert("Mod", WolframFunctionType::Arithmetic);
    map.insert("Abs", WolframFunctionType::Arithmetic);
    map.insert("Max", WolframFunctionType::Arithmetic);
    map.insert("Min", WolframFunctionType::Arithmetic);
    map.insert("Floor", WolframFunctionType::Arithmetic);
    map.insert("Ceiling", WolframFunctionType::Arithmetic);
    map.insert("Round", WolframFunctionType::Arithmetic);
    map.insert("Sign", WolframFunctionType::Arithmetic);
    map.insert("Factorial", WolframFunctionType::Arithmetic);
    map.insert("Binomial", WolframFunctionType::Arithmetic);

    // Trigonometric functions
    map.insert("Sin", WolframFunctionType::Trigonometric);
    map.insert("Cos", WolframFunctionType::Trigonometric);
    map.insert("Tan", WolframFunctionType::Trigonometric);

    // Logarithmic and exponential functions
    map.insert("Log", WolframFunctionType::Logarithmic);
    map.insert("Exp", WolframFunctionType::Logarithmic);
    map.insert("Sqrt", WolframFunctionType::Logarithmic);

    // Special functions
    map.insert("Gamma", WolframFunctionType::SpecialFunction);

    // Calculus functions
    map.insert("D", WolframFunctionType::Calculus);
    map.insert("Integrate", WolframFunctionType::Calculus);
    map.insert("Sum", WolframFunctionType::Calculus);
    map.insert("Limit", WolframFunctionType::Limit);

    // Polynomial theory (Phase 2)
    map.insert("CyclotomicPolynomial", WolframFunctionType::Polynomial);
    map.insert("Discriminant", WolframFunctionType::Polynomial);
    map.insert("GroebnerBasis", WolframFunctionType::Polynomial);
    map.insert("MinimalPolynomial", WolframFunctionType::Polynomial);
    map.insert("PolynomialGCD", WolframFunctionType::Polynomial);
    map.insert("Resultant", WolframFunctionType::Polynomial);

    // Number theory (Phase 2)
    map.insert("EulerPhi", WolframFunctionType::NumberTheory);
    map.insert("MoebiusMu", WolframFunctionType::NumberTheory);
    map.insert("PrimePi", WolframFunctionType::NumberTheory);
    map.insert("RiemannSiegelTheta", WolframFunctionType::NumberTheory);
    map.insert("GCD", WolframFunctionType::NumberTheory);
    map.insert("LCM", WolframFunctionType::NumberTheory);

    // Matrix and linear algebra functions
    map.insert("Det", WolframFunctionType::Matrix);
    map.insert("Tr", WolframFunctionType::Matrix);
    map.insert("Inverse", WolframFunctionType::Matrix);
    map.insert("Transpose", WolframFunctionType::Matrix);
    map.insert("Eigenvalues", WolframFunctionType::LinearAlgebra);
    map.insert("Eigenvectors", WolframFunctionType::LinearAlgebra);
    map.insert("MatrixPower", WolframFunctionType::Matrix);
    map.insert("MatrixExp", WolframFunctionType::Matrix);
    map.insert("Norm", WolframFunctionType::LinearAlgebra);
    map.insert("Cross", WolframFunctionType::LinearAlgebra);
    map.insert("Dot", WolframFunctionType::LinearAlgebra);
    map.insert("Inner", WolframFunctionType::LinearAlgebra);
    map.insert("Outer", WolframFunctionType::LinearAlgebra);
    map.insert("KroneckerProduct", WolframFunctionType::LinearAlgebra);
    map.insert("LinearSolve", WolframFunctionType::LinearAlgebra);
    map.insert("LeastSquares", WolframFunctionType::LinearAlgebra);
    map.insert("QRDecomposition", WolframFunctionType::LinearAlgebra);
    map.insert(
        "SingularValueDecomposition",
        WolframFunctionType::LinearAlgebra,
    );
    map.insert("LUDecomposition", WolframFunctionType::LinearAlgebra);
    map.insert("CholeskyDecomposition", WolframFunctionType::LinearAlgebra);

    // Complex number functions
    map.insert("Re", WolframFunctionType::Complex);
    map.insert("Im", WolframFunctionType::Complex);
    map.insert("Conjugate", WolframFunctionType::Complex);
    map.insert("Arg", WolframFunctionType::Complex);

    // Control structures
    map.insert("Piecewise", WolframFunctionType::Control);

    map
});

/// Parse a Wolfram function and return its type and semantic information
pub fn parse_wolfram_function(function: &str) -> (WolframFunctionType, &str) {
    let function_type = WOLFRAM_FUNCTIONS
        .get(function)
        .cloned()
        .unwrap_or(WolframFunctionType::Unknown);

    (function_type, function)
}

/// Check if a function name is a known Wolfram function
pub fn is_wolfram_function(function: &str) -> bool {
    WOLFRAM_FUNCTIONS.contains_key(function)
}

/// Check if a Wolfram function requires special handling
pub fn requires_special_handling(function: &str) -> bool {
    let (func_type, _) = parse_wolfram_function(function);
    matches!(
        func_type,
        WolframFunctionType::Calculus | WolframFunctionType::Control | WolframFunctionType::Limit
    )
}

/// Get all supported Wolfram functions
pub fn get_all_wolfram_functions() -> Vec<&'static str> {
    WOLFRAM_FUNCTIONS.keys().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_functions() {
        assert_eq!(
            parse_wolfram_function("Plus").0,
            WolframFunctionType::Arithmetic
        );
        assert_eq!(
            parse_wolfram_function("Times").0,
            WolframFunctionType::Arithmetic
        );
        assert!(is_wolfram_function("Abs"));
    }

    #[test]
    fn test_trigonometric_functions() {
        assert_eq!(
            parse_wolfram_function("Sin").0,
            WolframFunctionType::Trigonometric
        );
        assert_eq!(
            parse_wolfram_function("Cos").0,
            WolframFunctionType::Trigonometric
        );
    }

    #[test]
    fn test_matrix_functions() {
        assert_eq!(parse_wolfram_function("Det").0, WolframFunctionType::Matrix);
        assert_eq!(
            parse_wolfram_function("Eigenvalues").0,
            WolframFunctionType::LinearAlgebra
        );
    }

    #[test]
    fn test_unknown_functions() {
        assert_eq!(
            parse_wolfram_function("UnknownFunction").0,
            WolframFunctionType::Unknown
        );
        assert!(!is_wolfram_function("NotAFunction"));
    }

    #[test]
    fn test_special_handling() {
        assert!(requires_special_handling("Integrate"));
        assert!(requires_special_handling("Piecewise"));
        assert!(!requires_special_handling("Sin"));
    }

    #[test]
    fn test_function_count() {
        let functions = get_all_wolfram_functions();
        // We should have all the major Wolfram functions
        assert!(functions.len() >= 50);
        assert!(functions.contains(&"Sin"));
        assert!(functions.contains(&"Integrate"));
        assert!(functions.contains(&"Det"));
    }
}

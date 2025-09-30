/// Mathematical constants and patterns for lexical analysis
///
/// Provides efficient pattern matching for mathematical functions,
/// operators, and command recognition.

/// LaTeX command patterns for direct tokenization
pub const LATEX_COMMANDS: &[&str] = &[
    // Fractions and roots
    "\\frac{",
    "\\sqrt{",
    "\\cbrt{",
    // Trigonometric functions
    "\\sin(",
    "\\cos(",
    "\\tan(",
    "\\sec(",
    "\\csc(",
    "\\cot(",
    // Inverse trigonometric functions
    "\\arcsin(",
    "\\arccos(",
    "\\arctan(",
    // Hyperbolic functions
    "\\sinh(",
    "\\cosh(",
    "\\tanh(",
    // Logarithmic functions
    "\\ln(",
    "\\log(",
    "\\log_",
    // Special functions
    "\\Gamma(",
    "\\exp(",
    // Function powers (common patterns)
    "\\sin^{",
    "\\cos^{",
    "\\tan^{",
    "\\ln^{",
    "\\log^{",
    // Calculus
    "\\int",
    "\\int_",
    "\\sum_{",
    "\\prod_{",
    "\\lim_{",
    "\\frac{d}{d",
    "\\partial",
    // Delimiters
    "\\left(",
    "\\right)",
    "\\left[",
    "\\right]",
    "\\left\\{",
    "\\right\\}",
    "\\left|",
    "\\right|",
    // Environments
    "\\begin{cases}",
    "\\end{cases}",
    "\\begin{pmatrix}",
    "\\end{pmatrix}",
    "\\begin{bmatrix}",
    "\\end{bmatrix}",
    // Operators
    "\\cdot",
    "\\times",
    "\\div",
    "\\pm",
    "\\mp",
    // Constants and symbols
    "\\pi",
    "\\infty",
    "\\emptyset",
    // Relations
    "\\leq",
    "\\geq",
    "\\neq",
    "\\approx",
    "\\equiv",
    // Set operations
    "\\cup",
    "\\cap",
    "\\subset",
    "\\supset",
    "\\in",
    "\\notin",
    // Arrows
    "\\to",
    "\\rightarrow",
    "\\leftarrow",
    "\\leftrightarrow",
    // Text
    "\\text{if}",
    "\\text{otherwise}",
];

/// Wolfram Language function patterns
pub const WOLFRAM_FUNCTIONS: &[&str] = &[
    // Basic arithmetic
    "Plus[",
    "Times[",
    "Power[",
    "Subtract[",
    "Divide[",
    // Trigonometric functions
    "Sin[",
    "Cos[",
    "Tan[",
    "Sec[",
    "Csc[",
    "Cot[",
    // Inverse trigonometric
    "ArcSin[",
    "ArcCos[",
    "ArcTan[",
    // Hyperbolic functions
    "Sinh[",
    "Cosh[",
    "Tanh[",
    // Logarithmic functions
    "Log[",
    "Exp[",
    // Special functions
    "Gamma[",
    "Beta[",
    "Factorial[",
    "Sqrt[",
    // Calculus
    "D[",
    "Integrate[",
    "Limit[",
    "Sum[",
    "Product[",
    // Complex numbers
    "Complex[",
    "Re[",
    "Im[",
    "Abs[",
    "Arg[",
    // Lists and sets
    "List[",
    "Set[",
    // Piecewise
    "Piecewise[{",
    // Matrix operations
    "MatrixForm[",
    "Transpose[",
    "Det[",
    "Inverse[",
    // Logic
    "And[",
    "Or[",
    "Not[",
    "Implies[",
    // Comparison
    "Equal[",
    "Unequal[",
    "Less[",
    "Greater[",
    "LessEqual[",
    "GreaterEqual[",
];

/// Mathematical constants in different notations
pub const MATHEMATICAL_CONSTANTS: &[(&str, &str)] = &[
    // LaTeX notation
    ("\\pi", "pi"),
    ("\\infty", "infinity"),
    ("\\emptyset", "emptyset"),
    // Unicode symbols
    ("π", "pi"),
    ("∞", "infinity"),
    ("∅", "emptyset"),
    ("φ", "phi"),
    ("γ", "gamma"),
    // Standard notation
    ("pi", "pi"),
    ("e", "e"),
    ("i", "i"),
    ("infinity", "infinity"),
    // Wolfram notation
    ("Pi", "pi"),
    ("E", "e"),
    ("I", "i"),
    ("Infinity", "infinity"),
];

/// Check if string starts with any LaTeX command
pub fn matches_latex_command(input: &str) -> Option<&'static str> {
    LATEX_COMMANDS
        .iter()
        .find(|&&cmd| input.starts_with(cmd))
        .copied()
}

/// Check if string starts with any Wolfram function
pub fn matches_wolfram_function(input: &str) -> Option<&'static str> {
    WOLFRAM_FUNCTIONS
        .iter()
        .find(|&&func| input.starts_with(func))
        .copied()
}

/// Get mathematical constant name
pub fn get_constant_name(input: &str) -> Option<&'static str> {
    MATHEMATICAL_CONSTANTS
        .iter()
        .find(|(pattern, _)| input.starts_with(pattern))
        .map(|(_, name)| *name)
}

/// Check if character is a mathematical operator
pub fn is_math_operator(ch: char) -> bool {
    matches!(
        ch,
        '+' | '-'
            | '*'
            | '/'
            | '^'
            | '='
            | '<'
            | '>'
            | '!'
            | '&'
            | '|'
            | '±'
            | '∓'
            | '·'
            | '×'
            | '÷'
    )
}

/// Check if character is a delimiter
pub fn is_delimiter(ch: char) -> bool {
    matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_command_matching() {
        assert_eq!(matches_latex_command("\\frac{1}{2}"), Some("\\frac{"));
        assert_eq!(matches_latex_command("\\sin(x)"), Some("\\sin("));
        assert_eq!(matches_latex_command("\\sin^{2}(x)"), Some("\\sin^{"));
        assert_eq!(matches_latex_command("\\unknown"), None);
    }

    #[test]
    fn test_wolfram_function_matching() {
        assert_eq!(matches_wolfram_function("Sin[x]"), Some("Sin["));
        assert_eq!(
            matches_wolfram_function("Integrate[x, x]"),
            Some("Integrate[")
        );
        assert_eq!(matches_wolfram_function("Unknown[x]"), None);
    }

    #[test]
    fn test_constant_recognition() {
        assert_eq!(get_constant_name("π"), Some("pi"));
        assert_eq!(get_constant_name("\\pi"), Some("pi"));
        assert_eq!(get_constant_name("Pi"), Some("pi"));
        assert_eq!(get_constant_name("unknown"), None);
    }
}

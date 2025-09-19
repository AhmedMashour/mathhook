//! Parser constants and patterns

/// LaTeX function patterns and their internal names
pub const LATEX_SIMPLE_FUNCTIONS: &[(&str, &str)] = &[
    ("\\sin(", "sin"),
    ("\\cos(", "cos"),
    ("\\tan(", "tan"),
    ("\\ln(", "ln"),
    ("\\log(", "log"),
    ("\\exp(", "exp"),
];

/// LaTeX space-separated function patterns (for \sin x format)
pub const LATEX_SPACE_FUNCTIONS: &[(&str, &str)] = &[
    ("\\sin ", "sin"),
    ("\\cos ", "cos"),
    ("\\tan ", "tan"),
    ("\\ln ", "ln"),
    ("\\log ", "log"),
    ("\\exp ", "exp"),
];

/// Wolfram function patterns and their internal names
pub const WOLFRAM_SIMPLE_FUNCTIONS: &[(&str, &str)] = &[
    ("Sin[", "sin"),
    ("Cos[", "cos"),
    ("Tan[", "tan"),
    ("Log[", "ln"),
];

/// Wolfram operators that become Expression operations
pub const WOLFRAM_OPERATORS: &[(&str, &str)] =
    &[("Times[", "mul"), ("Plus[", "add"), ("Power[", "pow")];

/// Wolfram special functions requiring custom handling
pub const WOLFRAM_SPECIAL_FUNCTIONS: &[(&str, &str)] = &[
    ("Exp[", "exp_to_power"),
    ("Sqrt[", "sqrt_to_power"),
    ("D[", "derivative"),
    ("Integrate[", "integral"),
    ("Limit[", "limit"),
    ("Sum[", "sum"),
];

/// LaTeX symbol replacements for preprocessing
pub const LATEX_SIMPLE_REPLACEMENTS: &[(&str, &str)] = &[
    ("\\cdot", "*"),
    ("\\times", "*"),
    ("\\pi", "π"),
    ("\\infty", "∞"),
];

/// Wolfram constant replacements for preprocessing (whole words only)
pub const WOLFRAM_SIMPLE_REPLACEMENTS: &[(&str, &str)] = &[
    (" Pi ", " π "),       // Whole word Pi
    (" E ", " e "),        // Whole word E
    (" Infinity ", " ∞ "), // Whole word Infinity
    (" I ", " i "),        // Whole word I
];

/// Patterns for detecting LaTeX input
pub const LATEX_DETECTION_PATTERNS: &[&str] = &[
    "\\frac",
    "\\sqrt",
    "\\sin",
    "\\cos",
    "\\ln",
    "\\int",
    "\\sum",
    "\\cdot",
    "\\times",
    "\\div",
    "\\pm",
    "\\mp",
    "\\log",
    "\\pi",
    "\\infty",
    "\\lim",
    "\\partial",
    "\\begin{",
    "\\{",
];

/// Patterns for detecting Wolfram Language input
pub const WOLFRAM_DETECTION_PATTERNS: &[&str] = &[
    "Sin[",
    "Cos[",
    "Times[",
    "Plus[",
    "Power[",
    "Log[",
    "Integrate[",
    "Sqrt[",
    "Exp[",
    "D[",
    "Limit[",
    "Sum[",
    "{{", // Wolfram matrix syntax
];

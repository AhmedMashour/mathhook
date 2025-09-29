pub mod formatter;
pub mod parser;

/// Wolfram Language mathematical expression parser
///
/// Handles Wolfram Language (Mathematica) syntax including functions and operators.
///
/// Wolfram Language specific parser
pub struct WolframParser {}

impl WolframParser {
    pub fn new() -> Self {
        Self {}
    }
}

/// Wolfram formatting context
#[derive(Debug, Default)]
pub struct WolframContext {
    function_style: WolframFunctionStyle,
    precedence_level: u8,
}

/// Wolfram function call style
#[derive(Debug, Clone, Copy, PartialEq)]
enum WolframFunctionStyle {
    Functional, // Sin[x]
    Infix,      // x + y
    Prefix,     // -x
}

impl Default for WolframFunctionStyle {
    fn default() -> Self {
        WolframFunctionStyle::Functional
    }
}

/// Comprehensive token definitions for mathematical expression parsing
///
/// Defines all tokens used in mathematical expression parsing with
/// efficient string slice references for zero-copy parsing performance.

/// Mathematical expression token
///
/// Uses string slices for zero-copy parsing performance and structured
/// tokens for complex mathematical constructs.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'input> {
    // Basic operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    // Comparison operators
    Equals,
    NotEquals,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // Punctuation
    Comma,
    Semicolon,
    Exclamation,
    Ampersand,
    Arrow,       // ->
    DoubleArrow, // =>

    // Unicode mathematical symbols
    Pi,
    E,
    ImaginaryUnit,
    Infinity,
    NegativeInfinity,
    PlusMinus,
    MinusPlus,
    Cdot,
    Times,
    Div,
    Phi,
    Gamma,

    // Greek letters (commonly used)
    Alpha,
    Beta,
    Delta,
    Epsilon,
    Theta,
    Lambda,
    Mu,
    Sigma,
    Omega,

    // Literals
    Number(&'input str),
    Identifier(&'input str),

    // LaTeX commands (structured tokens)
    FracStart, // \frac{
    SqrtStart, // \sqrt{
    SinFunc,   // \sin(
    CosFunc,   // \cos(
    TanFunc,   // \tan(
    LnFunc,    // \ln(
    LogFunc,   // \log(
    GammaFunc, // \Gamma(
    ExpFunc,   // \exp(

    // LaTeX function powers
    SinPower, // \sin^{
    CosPower, // \cos^{
    TanPower, // \tan^{

    // LaTeX calculus
    IntegralStart,  // \int
    IntegralBounds, // \int_
    SumStart,       // \sum_{
    ProdStart,      // \prod_{
    LimitStart,     // \lim_{

    // LaTeX environments
    CasesStart,   // \begin{cases}
    CasesEnd,     // \end{cases}
    PMatrixStart, // \begin{pmatrix}
    PMatrixEnd,   // \end{pmatrix}

    // LaTeX delimiters
    LeftParen,    // \left(
    RightParen,   // \right)
    LeftBracket,  // \left[
    RightBracket, // \right]
    LeftBrace,    // \left\{
    RightBrace,   // \right\}

    // LaTeX text
    TextIf,        // \text{if}
    TextOtherwise, // \text{otherwise}

    // LaTeX arrows and relations
    To,  // \to
    Leq, // \leq
    Geq, // \geq
    Neq, // \neq

    // Wolfram functions (structured tokens)
    WolframPlus,  // Plus[
    WolframTimes, // Times[
    WolframPower, // Power[
    WolframSin,   // Sin[
    WolframCos,   // Cos[
    WolframTan,   // Tan[
    WolframLog,   // Log[
    WolframExp,   // Exp[
    WolframSqrt,  // Sqrt[
    WolframGamma, // Gamma[

    // Wolfram calculus
    WolframD,         // D[
    WolframIntegrate, // Integrate[
    WolframLimit,     // Limit[
    WolframSum,       // Sum[
    WolframProduct,   // Product[

    // Wolfram special
    WolframPiecewise, // Piecewise[{

    // Wolfram constants
    WolframPi,       // Pi
    WolframE,        // E
    WolframI,        // I
    WolframInfinity, // Infinity

    // Special patterns
    DifferentialD, // d (in dx)
    LineBreak,     // \\ (in LaTeX)

    // Keywords and special tokens
    If,
    Then,
    Else,
    True,
    False,
}

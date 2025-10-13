# Mathematical Notation

This appendix documents the mathematical notation used throughout MathHook.

## LaTeX Support

MathHook can parse standard LaTeX mathematical notation:

- `\frac{a}{b}` - Fractions
- `\sqrt{x}` - Square root
- `x^{2}` - Exponentiation
- `\sin(x)`, `\cos(x)`, `\tan(x)` - Trigonometric functions
- `\log(x)`, `\ln(x)` - Logarithms
- `\sum`, `\prod`, `\int` - Summation, product, integral
- Greek letters: `\alpha`, `\beta`, `\gamma`, etc.

## Standard Notation

- `2*x` or `2x` - Multiplication (implicit multiplication supported)
- `x^2` - Exponentiation
- `x/y` - Division
- `sin(x)` - Functions
- `|x|` - Absolute value

## Wolfram Language

MathHook also supports Wolfram Language syntax:

- `Power[x, 2]` - Exponentiation
- `Sin[x]` - Functions
- `D[expr, x]` - Derivatives
- `Integrate[expr, x]` - Integration

## Operator Precedence

1. Function application: `sin(x)`, `log(y)`
2. Exponentiation: `^` (right-associative)
3. Multiplication/Division: `*`, `/` (left-associative)
4. Addition/Subtraction: `+`, `-` (left-associative)

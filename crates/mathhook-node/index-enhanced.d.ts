/**
 * Enhanced TypeScript definitions for mathhook-node
 *
 * This file provides advanced TypeScript types and utilities
 * that complement the auto-generated index.d.ts file.
 *
 * Import this file for enhanced type safety and IDE support:
 * ```typescript
 * import type { MathFunction, SymbolicVariable } from 'mathhook/index-enhanced';
 * ```
 */

import { JsExpression, MathSolver } from './index';

// ============================================================
// Branded Types (Type Safety)
// ============================================================

/**
 * Branded type for symbolic expressions (created from symbols)
 * Prevents mixing incompatible types at compile time
 */
export type SymbolicVariable = JsExpression & { readonly __brand: 'symbolic' };

/**
 * Branded type for numeric expressions (created from numbers)
 * Prevents accidental mixing of numeric and symbolic contexts
 */
export type NumericExpression = JsExpression & { readonly __brand: 'numeric' };

/**
 * Branded type for constant expressions (pi, e, i, etc.)
 */
export type MathConstantExpression = JsExpression & { readonly __brand: 'constant' };

// ============================================================
// Utility Types
// ============================================================

/**
 * Type for unary mathematical functions (single argument)
 * Examples: sin, cos, exp, ln, sqrt
 */
export type MathFunction = (x: JsExpression | number) => JsExpression;

/**
 * Type for binary mathematical operations
 * Examples: add, subtract, multiply, divide, pow
 */
export type BinaryOperation = (a: JsExpression | number, b: JsExpression | number) => JsExpression;

/**
 * Type for mathematical comparison operators
 */
export type ComparisonOperator = 'eq' | 'ne' | 'lt' | 'le' | 'gt' | 'ge';

/**
 * Type for expression simplification strategies
 */
export type SimplificationStrategy = 'automatic' | 'algebraic' | 'trigonometric' | 'logarithmic';

// ============================================================
// Function Groups
// ============================================================

/**
 * Namespace for trigonometric functions
 * Provides grouped access to trig functions with consistent types
 */
export namespace Trigonometric {
    export const sin: MathFunction;
    export const cos: MathFunction;
    export const tan: MathFunction;
    export const asin: MathFunction;
    export const acos: MathFunction;
    export const atan: MathFunction;
}

/**
 * Namespace for hyperbolic functions
 */
export namespace Hyperbolic {
    export const sinh: MathFunction;
    export const cosh: MathFunction;
    export const tanh: MathFunction;
}

/**
 * Namespace for elementary functions
 */
export namespace Elementary {
    export const exp: MathFunction;
    export const ln: MathFunction;
    export const log10: MathFunction;
    export const sqrt: MathFunction;
    export const abs: MathFunction;
}

/**
 * Namespace for rounding functions
 */
export namespace Rounding {
    export const sign: MathFunction;
    export const floor: MathFunction;
    export const ceil: MathFunction;
    export const round: MathFunction;
}

/**
 * Namespace for special functions
 */
export namespace Special {
    export const gamma: MathFunction;
    export const factorial: MathFunction;
}

// ============================================================
// Solver Options
// ============================================================

/**
 * Options for equation solving
 */
export interface SolverOptions {
    /**
     * Maximum number of iterations for iterative methods
     * @default 1000
     */
    maxIterations?: number;

    /**
     * Numerical precision threshold
     * @default 1e-10
     */
    precision?: number;

    /**
     * Solution domain
     * - 'real': Only find real solutions
     * - 'complex': Include complex solutions
     * @default 'real'
     */
    domain?: 'real' | 'complex';

    /**
     * Whether to simplify solutions
     * @default true
     */
    simplify?: boolean;
}

/**
 * Result of equation solving
 */
export interface SolutionResult {
    /**
     * Array of solution expressions
     */
    solutions: JsExpression[];

    /**
     * Whether the solution is exact or approximate
     */
    exact: boolean;

    /**
     * Number of iterations used (for iterative methods)
     */
    iterations?: number;

    /**
     * Error message if solving failed
     */
    error?: string;
}

// ============================================================
// Parser Options
// ============================================================

/**
 * Options for expression parsing
 */
export interface ParseOptions {
    /**
     * Expected input format
     * - 'auto': Automatically detect format
     * - 'standard': Standard mathematical notation
     * - 'latex': LaTeX notation
     * - 'wolfram': Wolfram notation
     * @default 'auto'
     */
    format?: 'auto' | 'standard' | 'latex' | 'wolfram';

    /**
     * Whether to simplify after parsing
     * @default false
     */
    simplify?: boolean;

    /**
     * Implicit multiplication handling
     * @default true
     */
    implicitMultiplication?: boolean;
}

// ============================================================
// Expression Builder Pattern
// ============================================================

/**
 * Fluent builder for complex expressions
 * Enables readable construction of complex mathematical expressions
 *
 * @example
 * ```typescript
 * const expr = new ExpressionBuilder()
 *   .symbol('x')
 *   .pow(2)
 *   .add(3)
 *   .multiply(2)
 *   .build();
 * ```
 */
export class ExpressionBuilder {
    /**
     * Start with a symbol
     */
    symbol(name: string): this;

    /**
     * Start with an integer
     */
    integer(value: number): this;

    /**
     * Start with a float
     */
    float(value: number): this;

    /**
     * Add to current expression
     */
    add(other: JsExpression | number): this;

    /**
     * Subtract from current expression
     */
    subtract(other: JsExpression | number): this;

    /**
     * Multiply current expression
     */
    multiply(other: JsExpression | number): this;

    /**
     * Divide current expression
     */
    divide(other: JsExpression | number): this;

    /**
     * Raise current expression to power
     */
    pow(exponent: JsExpression | number): this;

    /**
     * Negate current expression
     */
    negate(): this;

    /**
     * Apply a function to current expression
     */
    apply(func: MathFunction): this;

    /**
     * Build and return the expression
     */
    build(): JsExpression;
}

// ============================================================
// Type Guards
// ============================================================

/**
 * Type guard to check if expression is symbolic
 */
export function isSymbolic(expr: JsExpression): expr is SymbolicVariable;

/**
 * Type guard to check if expression is numeric
 */
export function isNumeric(expr: JsExpression): expr is NumericExpression;

/**
 * Type guard to check if expression is a constant
 */
export function isConstant(expr: JsExpression): expr is MathConstantExpression;

// ============================================================
// Helper Types for Advanced Use Cases
// ============================================================

/**
 * Type for polynomial coefficients
 */
export type PolynomialCoefficients = {
    [degree: number]: number | JsExpression;
};

/**
 * Type for matrix dimensions
 */
export interface MatrixDimensions {
    rows: number;
    cols: number;
}

/**
 * Type for derivative order
 */
export type DerivativeOrder = number;

/**
 * Type for integration bounds
 */
export interface IntegrationBounds {
    lower: JsExpression | number;
    upper: JsExpression | number;
}

// ============================================================
// Re-export core types with enhancements
// ============================================================

export {
    JsExpression as Expression,
    MathSolver as Solver,
} from './index';

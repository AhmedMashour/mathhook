#!/usr/bin/env tsx

/**
 * Advanced MathHook Node.js/TypeScript Usage Examples
 * 
 * This example demonstrates advanced mathematical operations,
 * complex expressions, and real-world use cases.
 */

import { JsExpression, JsMathSolver, JsMathParser } from '../mathhook-node.node';

console.log('🚀 MathHook TypeScript Advanced Usage Examples\n');

// ===== Complex Mathematical Expressions =====
console.log('🔬 Complex Mathematical Expressions:');

// Variables for advanced examples
const x = JsExpression.symbol('x');
const y = JsExpression.symbol('y');
const z = JsExpression.symbol('z');
const a = JsExpression.symbol('a');
const b = JsExpression.symbol('b');
const c = JsExpression.symbol('c');

// Multi-variable polynomial: ax² + bxy + cy²
const multiPoly = a.multiply(x.pow(JsExpression.integer(2)))
    .add(b.multiply(x).multiply(y))
    .add(c.multiply(y.pow(JsExpression.integer(2))));

console.log(`Multi-variable polynomial: ${multiPoly.toString()}`);
console.log(`Simplified: ${multiPoly.simplify().toString()}\n`);

// ===== Nested Operations =====
console.log('🎯 Nested Mathematical Operations:');

// Nested expression: (x + y)³
const cubed = x.add(y).pow(JsExpression.integer(3));
console.log(`(x + y)³: ${cubed.toString()}`);
console.log(`Simplified: ${cubed.simplify().toString()}`);

// Complex fraction-like expression: (x² + 2x + 1) / (x + 1)
const numerator = x.pow(JsExpression.integer(2))
    .add(JsExpression.integer(2).multiply(x))
    .add(JsExpression.integer(1));
const denominator = x.add(JsExpression.integer(1));
const fraction = numerator.multiply(denominator.pow(JsExpression.integer(-1)));

console.log(`Complex fraction: ${fraction.toString()}`);
console.log(`Simplified: ${fraction.simplify().toString()}\n`);

// ===== System of Equations (Conceptual) =====
console.log('📊 System of Equations Example:');

try {
    const solver = new JsMathSolver();
    
    // First equation: 2x + 3y = 12
    const eq1Left = JsExpression.integer(2).multiply(x)
        .add(JsExpression.integer(3).multiply(y));
    const eq1 = JsExpression.equation(eq1Left, JsExpression.integer(12));
    
    // Second equation: x - y = 1  
    const eq2Left = x.add(y.multiply(JsExpression.integer(-1)));
    const eq2 = JsExpression.equation(eq2Left, JsExpression.integer(1));
    
    console.log(`Equation 1: ${eq1.toString()}`);
    console.log(`Equation 2: ${eq2.toString()}`);
    
    // Note: Current solver handles single equations
    // System solving would require additional implementation
    console.log('📝 Note: System solving requires additional implementation\n');
    
} catch (error: any) {
    console.error(`❌ System solver error: ${error.message}\n`);
}

// ===== Advanced Parsing Examples =====
console.log('📚 Advanced Parsing Examples:');

try {
    const parser = new JsMathParser();
    
    // Complex LaTeX expressions
    const latexExpressions = [
        '\\sqrt{x^2 + y^2}',
        '\\frac{a^2 + b^2}{c^2}',
        'x^{2n+1} + y^{n-1}',
        '\\sum_{i=1}^{n} x_i^2'
    ];
    
    console.log('LaTeX Expressions:');
    latexExpressions.forEach((expr, index) => {
        try {
            const parsed = parser.parse(expr, 'latex');
            console.log(`  ${index + 1}. "${expr}" → ${parsed.toString()}`);
        } catch (e: any) {
            console.log(`  ${index + 1}. "${expr}" → Parse Error: ${e.message}`);
        }
    });
    
    // Complex Wolfram expressions
    const wolframExpressions = [
        'Expand[(x + y)^3]',
        'Simplify[x^2 - 2*x + 1]',
        'Factor[x^2 - 1]',
        'D[x^3 + 2*x^2 + x, x]'
    ];
    
    console.log('\nWolfram Expressions:');
    wolframExpressions.forEach((expr, index) => {
        try {
            const parsed = parser.parse(expr, 'wolfram');
            console.log(`  ${index + 1}. "${expr}" → ${parsed.toString()}`);
        } catch (e: any) {
            console.log(`  ${index + 1}. "${expr}" → Parse Error: ${e.message}`);
        }
    });
    
} catch (error: any) {
    console.error(`❌ Advanced parser error: ${error.message}`);
}

console.log('\n');

// ===== Performance Testing =====
console.log('⚡ Performance Testing:');

const performanceTest = () => {
    const startTime = process.hrtime.bigint();
    
    // Create and simplify 1000 expressions
    for (let i = 0; i < 1000; i++) {
        const expr = x.multiply(JsExpression.integer(i))
            .add(y.pow(JsExpression.integer(2)))
            .add(JsExpression.integer(i * 2));
        expr.simplify();
    }
    
    const endTime = process.hrtime.bigint();
    const duration = Number(endTime - startTime) / 1_000_000; // Convert to milliseconds
    
    console.log(`Created and simplified 1000 expressions in ${duration.toFixed(2)}ms`);
    console.log(`Average: ${(duration / 1000).toFixed(4)}ms per expression`);
};

performanceTest();

// ===== Memory Usage Example =====
console.log('\n💾 Memory Usage Example:');

const memoryTest = () => {
    const expressions: JsExpression[] = [];
    
    // Create a large number of expressions
    for (let i = 0; i < 10000; i++) {
        const expr = x.pow(JsExpression.integer(i % 5))
            .add(y.multiply(JsExpression.integer(i)))
            .add(z.pow(JsExpression.integer(2)));
        expressions.push(expr);
    }
    
    console.log(`Created ${expressions.length} expressions in memory`);
    
    // Simplify all expressions
    const simplified = expressions.map(expr => expr.simplify());
    console.log(`Simplified ${simplified.length} expressions`);
    
    // Clear references (JavaScript GC will handle cleanup)
    expressions.length = 0;
    simplified.length = 0;
    
    console.log('Memory test completed');
};

memoryTest();

// ===== Real-world Use Case: Quadratic Formula =====
console.log('\n🎓 Real-world Use Case: Quadratic Formula');

const quadraticFormula = () => {
    try {
        const solver = new JsMathSolver();
        
        // Quadratic equation: ax² + bx + c = 0
        // For example: x² - 5x + 6 = 0
        const a_val = JsExpression.integer(1);
        const b_val = JsExpression.integer(-5);
        const c_val = JsExpression.integer(6);
        
        const quadratic = a_val.multiply(x.pow(JsExpression.integer(2)))
            .add(b_val.multiply(x))
            .add(c_val);
        
        const equation = JsExpression.equation(quadratic, JsExpression.integer(0));
        
        console.log(`Quadratic equation: ${equation.toString()}`);
        
        const solution = solver.solve(equation, 'x');
        console.log(`Solution: ${solution}`);
        
        // The solutions should be x = 2 and x = 3 (since (x-2)(x-3) = x² - 5x + 6)
        
    } catch (error: any) {
        console.error(`❌ Quadratic formula error: ${error.message}`);
    }
};

quadraticFormula();

console.log('\n✅ All advanced examples completed successfully!');
console.log('🎉 You now know how to use MathHook with TypeScript for complex mathematical operations!');

# MathHook Node.js/TypeScript Examples

This directory contains comprehensive examples demonstrating how to use the MathHook Node.js bindings with TypeScript.

> **Note**: These examples are part of the `mathhook-node` crate and use the binding directly from the parent directory.

## 🚀 Quick Start

### Prerequisites
- Node.js 16+ 
- npm or yarn

### Setup

1. **Complete setup (recommended):**
   ```bash
   npm run setup
   ```
   This will install dependencies, build the binding, and create the necessary links.

2. **Or step by step:**
   ```bash
   # Install dependencies
   npm install
   
   # Build the MathHook binding
   npm run build-binding
   
   # Link the binding to this directory
   npm run link-binding
   ```

3. **Run the examples:**
   ```bash
   # Basic usage examples
   npm run basic
   
   # Advanced usage examples  
   npm run advanced
   ```

## 📁 Files

- `basic-usage.ts` - Fundamental operations and concepts
- `advanced-usage.ts` - Complex expressions and real-world use cases
- `package.json` - Dependencies and scripts
- `tsconfig.json` - TypeScript configuration
- `README.md` - This file
- `../mathhook-node.node` - Native binding (built by parent crate)
- `../index.d.ts` - TypeScript definitions (from parent crate)

## 🧮 What You'll Learn

### Basic Usage
- Creating mathematical expressions
- Basic arithmetic operations (add, multiply, power)
- Expression simplification
- Equation solving
- Parsing mathematical notation (Standard, LaTeX, Wolfram)

### Advanced Usage
- Complex multi-variable expressions
- Nested mathematical operations
- Performance testing
- Memory usage patterns
- Real-world use cases (quadratic formula)

## 🔧 Available Scripts

- `npm run setup` - Complete setup (install deps + build binding)
- `npm run basic` - Run basic usage examples
- `npm run advanced` - Run advanced usage examples
- `npm run build-binding` - Build the MathHook native binding
- `npm run clean` - Clean build artifacts

## 📚 API Reference

The examples use these main classes:

- `JsExpression` - Core mathematical expression type
- `JsMathSolver` - Equation solving functionality  
- `JsMathParser` - Parse mathematical notation from strings

## 🎯 Example Output

When you run the examples, you'll see output like:

```
🧮 MathHook TypeScript Basic Usage Examples

📝 Creating Mathematical Expressions:
Variable x: Symbol("x")
Variable y: Symbol("y")
Constant 2: Number(Integer(2))
Constant 3: Number(Integer(3))

🔢 Basic Arithmetic Operations:
x + 2 = Add([Symbol("x"), Number(Integer(2))])
x × 3 = Mul([Symbol("x"), Number(Integer(3))])
x² = Pow(Symbol("x"), Number(Integer(2)))
2x + 3 = Add([Mul([Number(Integer(2)), Symbol("x")]), Number(Integer(3))])
```

## 🐛 Troubleshooting

If you encounter issues:

1. **Binding not found**: Run `npm run build-binding`
2. **TypeScript errors**: 
   - Check that `tsx` is installed with `npm install`
   - Ensure `mathhook-node.d.ts` exists for type definitions
3. **Import errors**: Use `from './mathhook-node'` (not `.node` extension)
4. **Node version**: Ensure you're using Node.js 16+

## 🤝 Contributing

These examples are part of the MathHook project. See the main repository for contribution guidelines.

# MathHook Examples

This directory contains comprehensive examples demonstrating how to use MathHook's language bindings.

## 📁 Available Examples

### 🟢 Node.js/TypeScript Examples
**Status: ✅ Fully Working**
**Location: `crates/mathhook-node/examples/`**

Complete TypeScript examples with proper binding integration:
- Basic mathematical operations
- Expression simplification
- Equation solving
- Mathematical notation parsing
- Performance testing

**Quick Start:**
```bash
cd crates/mathhook-node/examples/
npm run setup
npm run basic
```

### 🟡 Python Examples
**Status: 🚧 Placeholder (Binding Not Yet Implemented)**
**Location: `crates/mathhook-python/examples/`**

Planned Python examples showing the intended API structure:
- Expression creation and manipulation
- Solver integration
- Parser functionality
- Advanced mathematical operations
- Performance testing

**To Implement:**
```bash
cd crates/mathhook-python/examples/
pip install maturin
cd .. && maturin develop
python basic_usage.py
```

## 🚀 Getting Started

### For Node.js/TypeScript Users

1. **Navigate to the Node examples:**
   ```bash
   cd crates/mathhook-node/examples/
   ```

2. **Run the complete setup:**
   ```bash
   npm run setup
   ```

3. **Try the examples:**
   ```bash
   npm run basic      # Basic usage
   npm run advanced   # Advanced features
   ```

### For Python Users

The Python binding is not yet implemented. The examples in `crates/mathhook-python/examples/` contain placeholder code showing the intended API structure.

**To Implement:**
```bash
cd crates/mathhook-python/examples/
pip install maturin
cd .. && maturin develop
python basic_usage.py
```

## 🧮 What You'll Learn

- **Expression Creation**: Build mathematical expressions programmatically
- **Symbolic Operations**: Add, multiply, exponentiate symbolic expressions
- **Simplification**: Automatically simplify complex mathematical expressions
- **Equation Solving**: Solve algebraic equations for unknown variables
- **Parsing**: Convert mathematical notation (Standard, LaTeX, Wolfram) to expressions
- **Performance**: Understand performance characteristics and optimization

## 📚 Example Output

When you run the Node.js examples, you'll see output like:

```
🧮 MathHook TypeScript Basic Usage Examples

📝 Creating Mathematical Expressions:
Variable x: Symbol(Symbol { name: "x" })
Variable y: Symbol(Symbol { name: "y" })
Constant 2: Number(Integer(2))

🔢 Basic Arithmetic Operations:
x + 2 = Add([Symbol(Symbol { name: "x" }), Number(Integer(2))])
x × 3 = Mul([Symbol(Symbol { name: "x" }), Number(Integer(3))])

⚡ Expression Simplification:
Before: 2 + 3 = Add([Number(Integer(2)), Number(Integer(3))])
After:  Number(Integer(5))
```

## 🔧 Directory Structure

```
examples/
└── README.md                 # This file

crates/mathhook-node/examples/  # Node.js/TypeScript examples
├── README.md                   # Node-specific documentation  
├── package.json               # Dependencies and scripts
├── tsconfig.json              # TypeScript configuration
├── basic-usage.ts             # Basic examples
├── advanced-usage.ts          # Advanced examples
└── ../mathhook-node.node      # Native binding (built by parent crate)

crates/mathhook-python/examples/  # Python examples (placeholder)
├── README.md                     # Python-specific documentation
├── requirements.txt             # Python dependencies
├── basic_usage.py              # Basic examples (placeholder)
└── advanced_usage.py           # Advanced examples (placeholder)
```

## 🤝 Contributing

These examples are part of the MathHook project. To contribute:

1. **For Node.js examples**: Add new `.ts` files in `crates/mathhook-node/examples/` and update the scripts in `package.json`
2. **For Python examples**: Add new `.py` files in `crates/mathhook-python/examples/` and help implement the Python binding
3. **For documentation**: Improve README files and inline documentation

## 🐛 Troubleshooting

### Node.js Issues

- **"Cannot find module"**: Run `npm run build-binding` from `crates/mathhook-node/examples/`
- **TypeScript errors**: Ensure `tsx` is installed with `npm install`
- **Binding not found**: Run `npm run build-binding`

### Python Issues

- **Import errors**: Python binding not yet implemented
- **Maturin not found**: Install with `pip install maturin`
- **Build errors**: Run `cd crates/mathhook-python && maturin develop`

## 📖 Further Reading

- [MathHook Core Documentation](../crates/mathhook-core/)
- [Node.js Binding Source](../crates/mathhook-node/)
- [Python Binding Source](../crates/mathhook-python/)
- [Main Project README](../README.md)

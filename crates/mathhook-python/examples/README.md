# MathHook Python Examples

This directory contains comprehensive examples demonstrating how to use the MathHook Python bindings.

> **Note**: These examples are part of the `mathhook-python` crate. The Python binding is currently **not implemented** - these are placeholder examples showing the intended API structure.

## 🚧 Current Status

**Status: 🔴 Not Yet Implemented**

The Python binding for MathHook is planned but not yet developed. These examples serve as:
- API design documentation
- Implementation roadmap
- Test cases for future development

## 🚀 Quick Start (When Implemented)

### Prerequisites
- Python 3.8+
- pip or conda

### Setup

1. **Install maturin (build tool):**
   ```bash
   pip install maturin
   ```

2. **Build the Python binding:**
   ```bash
   cd ..  # Go to mathhook-python crate root
   maturin develop
   ```

3. **Install example dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

4. **Run the examples:**
   ```bash
   # Basic usage examples
   python basic_usage.py
   
   # Advanced usage examples  
   python advanced_usage.py
   ```

## 📁 Files

- `basic_usage.py` - Fundamental operations and concepts (placeholder)
- `advanced_usage.py` - Complex expressions and real-world use cases (placeholder)
- `requirements.txt` - Python dependencies
- `README.md` - This file
- `../src/lib.rs` - Rust binding source (to be implemented)

## 🧮 Planned API

The examples demonstrate the intended Python API:

### Basic Operations
```python
from mathhook_python import Expression, MathSolver, MathParser

# Create expressions
x = Expression.symbol('x')
y = Expression.symbol('y')
two = Expression.integer(2)

# Basic operations
sum_expr = x + two              # Addition
product = x * 3                 # Multiplication  
power = x ** two                # Exponentiation

# Simplification
result = sum_expr.simplify()

# Equation solving
solver = MathSolver()
equation = Expression.equation(x, Expression.integer(5))
solution = solver.solve(equation, 'x')

# Parsing
parser = MathParser()
parsed = parser.parse('x + 2*y', 'standard')
latex_parsed = parser.parse(r'\frac{x}{2} + y^2', 'latex')
```

### Advanced Features
```python
# Multi-variable expressions
multi_poly = a * x**2 + b * x * y + c * y**2

# Complex operations
cubed = (x + y)**3
expanded = cubed.expand()

# System solving (planned)
eq1 = Expression.equation(2*x + 3*y, 12)
eq2 = Expression.equation(x - y, 1)
solutions = solver.solve_system([eq1, eq2], ['x', 'y'])

# Calculus (planned)
derivative = expr.differentiate('x')
integral = expr.integrate('x')
limit = expr.limit('x', 0)
```

## 🔧 Implementation Roadmap

To implement the Python binding:

1. **Core Binding Structure**:
   - [ ] Basic Expression wrapper
   - [ ] Symbol and Number types
   - [ ] Arithmetic operations (+, -, *, /, **)

2. **Advanced Operations**:
   - [ ] Simplification engine integration
   - [ ] Equation solving
   - [ ] Expression parsing

3. **Python-Specific Features**:
   - [ ] Pythonic operator overloading
   - [ ] Iterator support
   - [ ] String representations (__str__, __repr__)
   - [ ] Jupyter notebook integration

4. **Performance Optimizations**:
   - [ ] Efficient memory management
   - [ ] Parallel processing support
   - [ ] Caching mechanisms

## 🎯 Example Output (Planned)

When implemented, running the examples should produce output like:

```
🧮 MathHook Python Basic Usage Examples

📝 Creating Mathematical Expressions:
Variable x: Symbol('x')
Variable y: Symbol('y')
Constant 2: Integer(2)

🔢 Basic Arithmetic Operations:
x + 2 = Add(Symbol('x'), Integer(2))
x * 3 = Mul(Symbol('x'), Integer(3))
x ** 2 = Pow(Symbol('x'), Integer(2))

⚡ Expression Simplification:
Before: 2 + 3 = Add(Integer(2), Integer(3))
After:  Integer(5)

🎯 Equation Solving:
Equation: Eq(Symbol('x'), Integer(5))
Solution: [Integer(5)]
```

## 🤝 Contributing

To contribute to the Python binding implementation:

1. **Study the Node.js binding**: See `../mathhook-node/src/lib.rs` for reference
2. **Implement core types**: Start with Expression, Symbol, Number
3. **Add operations**: Implement arithmetic and simplification
4. **Test thoroughly**: Use these examples as test cases
5. **Document**: Update examples as features are implemented

## 🐛 Troubleshooting

### Current Issues

- **Import errors**: Python binding not yet implemented
- **Maturin not found**: Install with `pip install maturin`
- **Build errors**: Binding source code needs to be written first

### Future Issues (When Implemented)

- **Performance**: Large expressions may be slow initially
- **Memory**: Complex expressions might use significant memory
- **Compatibility**: Ensure Python 3.8+ compatibility

## 📖 Further Reading

- [Maturin Documentation](https://maturin.rs/) - Python-Rust binding tool
- [PyO3 Guide](https://pyo3.rs/) - Rust-Python integration
- [MathHook Core](../mathhook-core/) - Core mathematical engine
- [Node.js Examples](../mathhook-node/examples/) - Working binding examples

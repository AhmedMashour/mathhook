# Plan 3: Python API to Production

**Priority**: 🐍 CRITICAL
**Timeline**: 8-10 weeks
**Waves**: 5
**Orchestrator**: `/sc:spawn`

## Executive Summary

**Current State**: 20% complete (15 basic methods, package not built)
- ✅ PyO3 infrastructure setup
- ❌ Package gives `ModuleNotFoundError` (never built with maturin)
- ❌ Missing: Operator overloading, Jupyter integration, calculus, 80% of features

**Goal**: Production-ready Python package installable via `pip install mathhook`

**Market Opportunity**: Python is THE language for scientific computing, neuro-symbolic AI, education

---

## Bootstrap Command

```bash
/sc:spawn python-expert "Execute Wave-Based Python API Production Plan for MathHook"
```

**Orchestrator Prompt**:

```markdown
You are the Orchestrator for **MathHook Python API Production**.

**Context**: You are the `python-expert` agent - Expert Python developer specializing in creating production-ready Python packages with Rust bindings via PyO3.

**Your Mission**: Execute a 5-wave plan to bring MathHook Python API from 20% complete to production-ready PyPI package.

**Mandatory Reading** (in this order):
1. `/Users/ahmedmashhour/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven wave-based methodology
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` - Project constraints (Rust backend)
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_3_PYTHON_API_PRODUCTION.md` - This plan

**5 Mandatory Rules**:
1. **You Are Always The Orchestrator** - Delegate to python-expert agents
2. **Sequential Waves, Parallel Agents** - Complete waves in order
3. **Mandatory Verification** - Each wave ends with verification
4. **Strict CLAUDE.md Enforcement** - Follow documentation standards for Rust bindings
5. **Maintain Momentum** - Report after each wave
```

---

## Wave Breakdown

### Wave 1: Build Infrastructure & Package (4-6 hours)

**Goal**: Make `pip install mathhook` work

**Tasks**:
1. Build with maturin: `cd crates/mathhook-python && maturin develop`
2. Test import: `python -c "import mathhook; print(mathhook.Expression.integer(42))"`
3. Fix any build errors
4. Create setup for PyPI publication
5. Test installation in clean virtualenv

**Verification**:
```bash
# verify_wave_1_python_build.sh
python3 -m venv test_env
source test_env/bin/activate
pip install -e crates/mathhook-python
python -c "import mathhook; assert mathhook.Expression.integer(42).to_simple() == '42'"
deactivate
rm -rf test_env
```

**Deliverables**:
- Working `pip install -e .`
- Import test passing
- maturin build configuration
- PyPI-ready structure

---

### Wave 2: Operator Overloading & Python Ergonomics (10-14 hours)

**Goal**: Make Python API Pythonic (not just Rust wrapper)

**Current** (verbose):
```python
x = Expression.symbol("x")
result = Expression.add(x, Expression.integer(2))
```

**Target** (Pythonic):
```python
x = symbol("x")
result = x + 2  # Operator overloading!
```

**Tasks**:
1. **Operator Overloading**:
   ```rust
   #[pymethods]
   impl PyExpression {
       fn __add__(&self, other: &PyExpression) -> PyExpression { ... }
       fn __sub__(&self, other: &PyExpression) -> PyExpression { ... }
       fn __mul__(&self, other: &PyExpression) -> PyExpression { ... }
       fn __truediv__(&self, other: &PyExpression) -> PyExpression { ... }
       fn __pow__(&self, other: &PyExpression) -> PyExpression { ... }
       fn __neg__(&self) -> PyExpression { ... }
   }
   ```

2. **Module-Level Functions** (Python ergonomics):
   ```python
   # Instead of Expression.symbol("x")
   def symbol(name: str) -> Expression: ...
   def symbols(*names: str) -> list[Expression]: ...
   def solve(equation, variable, domain="real"): ...
   def simplify(expr): ...
   ```

3. **Type Hints**: Add stub file `.pyi` for IDE autocomplete

4. **Conversion Helpers**:
   ```python
   def parse(expr: str | Expression) -> Expression: ...
   def to_int(expr: Expression) -> int | None: ...
   def to_float(expr: Expression) -> float: ...
   ```

**Verification**: Test operator overloading, module functions, type hints

**Deliverables**:
- Full operator overloading
- Pythonic module API
- Type stubs for IDEs
- Conversion helpers

---

### Wave 3: Jupyter Integration (8-12 hours)

**Goal**: Beautiful math rendering in Jupyter notebooks

**Target Experience**:
```python
from mathhook import symbol, solve

x = symbol("x")
equation = x**2 - 5*x + 6

# In Jupyter, this renders as LaTeX automatically:
equation  # Displays: x² - 5x + 6

solutions = solve(equation, x)
# Displays: x = 2, x = 3 (in beautiful LaTeX)
```

**Tasks**:
1. **IPython Display Hooks**:
   ```rust
   #[pymethods]
   impl PyExpression {
       fn _repr_latex_(&self) -> String {
           format!("$${}$$", self.inner.to_latex())
       }

       fn _repr_html_(&self) -> String {
           // MathJax/KaTeX rendering
       }
   }
   ```

2. **init_mathhook()**: Optional explicit initialization (SymPy has `init_printing()`)

3. **Interactive Features**:
   - Step-by-step display in notebooks
   - Interactive solver (show work checkbox)
   - Plot integration (if expression is plottable)

4. **Examples**: Create example notebooks:
   - `notebooks/quickstart.ipynb`
   - `notebooks/calculus.ipynb`
   - `notebooks/linear_algebra.ipynb`

**Verification**: Run notebooks, check LaTeX rendering works

**Deliverables**:
- Jupyter display hooks
- Example notebooks (3+)
- Interactive features
- Documentation for Jupyter usage

---

### Wave 4: Calculus & Mathematical Functions (12-16 hours)

**Goal**: Expose ALL mathhook-core functionality to Python

**Current**: Missing ~80% of core features

**Tasks**:
1. **Calculus Operations**:
   ```python
   def derivative(expr, variable, order=1): ...
   def integrate(expr, variable, lower=None, upper=None): ...
   def limit(expr, variable, point, direction="both"): ...
   def series(expr, variable, point, order): ...
   ```

2. **Mathematical Functions**:
   ```python
   def sin(x): ...
   def cos(x): ...
   def exp(x): ...
   def log(x, base=None): ...
   def sqrt(x): ...
   # ... all elementary and special functions
   ```

3. **Matrix Operations**:
   ```python
   class Matrix:
       def det(self): ...
       def inv(self): ...
       def eigenvalues(self): ...
       def __matmul__(self, other): ...  # @ operator
   ```

4. **Educational Features**:
   ```python
   def explain(expr, operation="simplify"):
       """Returns step-by-step explanation"""
       return expr.explain(operation)
   ```

**Verification**: Test all operations match Rust behavior

**Deliverables**:
- Complete calculus API
- All math functions exposed
- Matrix operations
- Educational explain() function

---

### Wave 5: Testing & PyPI Publication (8-10 hours)

**Goal**: Production-ready, published to PyPI

**Tasks**:
1. **Comprehensive Tests**:
   ```python
   # tests/test_operators.py
   def test_addition():
       x = symbol("x")
       assert (x + 2).to_simple() == "x + 2"

   # tests/test_calculus.py
   def test_derivative():
       x = symbol("x")
       assert derivative(x**2, x) == 2*x
   ```
   - Target: >90% coverage

2. **Performance Benchmarks**:
   - Compare against SymPy for common operations
   - Document speedup in README

3. **Documentation**:
   - README.md with quickstart
   - API reference (Sphinx)
   - Installation guide
   - Migration guide (from SymPy)

4. **PyPI Publication**:
   - Configure pyproject.toml for PyPI
   - Build wheels: `maturin build --release`
   - Publish: `maturin publish`
   - Test install: `pip install mathhook`

5. **CI/CD**:
   - GitHub Actions for testing
   - Automated PyPI releases
   - Documentation deployment

**Verification**: Install from PyPI, run tests, check docs

**Deliverables**:
- 90%+ test coverage
- PyPI package published
- Complete documentation
- CI/CD workflows

---

## Final Success Criteria

- [ ] `pip install mathhook` works
- [ ] Pythonic API (operators, module functions)
- [ ] Jupyter integration with LaTeX rendering
- [ ] Complete feature parity with core
- [ ] Published to PyPI
- [ ] >90% test coverage
- [ ] Documentation complete

**Exit Criteria**: Python users can switch from SymPy to MathHook with minimal code changes and get 10-100x speedup + educational features.

**Timeline**: 8-10 weeks to production PyPI package

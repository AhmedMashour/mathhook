# Plan 3: Python API to Production

**Priority**: 🐍 CRITICAL
**Timeline**: 9-11 weeks
**Waves**: 6
**Orchestrator**: `/sc:spawn`
**Version**: V2 (Added Wave 0: Platform Audit & Multi-Platform Build Strategy)

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

**Your Mission**: Execute a 6-wave plan to bring MathHook Python API from 20% complete to production-ready PyPI package (Wave 0 focuses on platform audit).

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

### Wave 0: Platform Audit & Multi-Platform Build Strategy (1 week)

**Objective**: Audit current platform support, identify gaps, and establish multi-platform wheel building strategy BEFORE Wave 1 build.

**Critical Success Criteria**:
- ✅ Platform audit completed (Linux, macOS, Windows - x86_64 and ARM64)
- ✅ Multi-platform build strategy documented
- ✅ Type stub (.pyi) generation strategy established
- ✅ GitHub Actions workflow for multi-platform builds created

**Tasks**:

1. **Platform Support Audit** (1 day):
   - Current maturin configuration analysis
   - Identify supported vs unsupported platforms
   - Document platform-specific dependencies (e.g., BLAS/LAPACK for linear algebra)
   - Create platform compatibility matrix:
     ```
     | Platform      | x86_64 | ARM64 | Status      | Blockers         |
     |---------------|--------|-------|-------------|------------------|
     | Linux         | ?      | ?     | To be tested | Unknown          |
     | macOS         | ?      | ?     | To be tested | Unknown          |
     | Windows       | ?      | ?     | To be tested | Unknown          |
     ```

2. **Multi-Platform Build Strategy** (2 days):
   - **maturin build configuration**:
     ```toml
     # pyproject.toml
     [build-system]
     requires = ["maturin>=1.0,<2.0"]
     build-backend = "maturin"

     [tool.maturin]
     python-source = "python"
     module-name = "mathhook._mathhook"
     ```

   - **Platform-specific build commands**:
     ```bash
     # Linux x86_64
     maturin build --release --target x86_64-unknown-linux-gnu

     # Linux ARM64
     maturin build --release --target aarch64-unknown-linux-gnu

     # macOS x86_64 (Intel)
     maturin build --release --target x86_64-apple-darwin

     # macOS ARM64 (Apple Silicon)
     maturin build --release --target aarch64-apple-darwin

     # Windows x86_64
     maturin build --release --target x86_64-pc-windows-msvc

     # Windows ARM64
     maturin build --release --target aarch64-pc-windows-msvc
     ```

   - **Cross-compilation strategy**:
     - Use GitHub Actions runners for native builds (fastest, most reliable)
     - Docker for Linux cross-compilation if needed
     - Document minimum Rust toolchain version
     - Identify platform-specific dependencies

3. **Type Stub (.pyi) Generation Strategy** (2 days):
   - **Problem**: IDE autocomplete doesn't work without type stubs
   - **Solution**: Generate .pyi files from PyO3 bindings

   - **Option 1: Manual type stubs** (fallback):
     ```python
     # mathhook.pyi
     class Expression:
         @staticmethod
         def symbol(name: str) -> Expression: ...
         @staticmethod
         def integer(value: int) -> Expression: ...
         def __add__(self, other: Expression | int | float) -> Expression: ...
         def __sub__(self, other: Expression | int | float) -> Expression: ...
         # ... all methods
     ```

   - **Option 2: pyo3-stub-gen** (preferred if available):
     ```bash
     pip install pyo3-stub-gen
     pyo3-stub-gen mathhook -o mathhook-stubs/
     ```

   - **Validation**: Test with mypy strict mode
     ```bash
     mypy --strict examples/test_types.py
     ```

   - **Distribution**: Include .pyi files in wheel
     ```toml
     # pyproject.toml
     [tool.maturin]
     include = ["mathhook/*.pyi"]
     ```

4. **GitHub Actions Multi-Platform CI** (2 days):
   - Create `.github/workflows/python_build_multiplatform.yml`:
     ```yaml
     name: Python Multi-Platform Wheels

     on:
       push:
         branches: [main, master]
       pull_request:
         types: [opened, synchronize]

     jobs:
       build-wheels:
         name: Build wheels on ${{ matrix.os }}
         runs-on: ${{ matrix.os }}
         strategy:
           matrix:
             os: [ubuntu-latest, macos-latest, windows-latest]
             python-version: ['3.8', '3.9', '3.10', '3.11', '3.12']

         steps:
           - uses: actions/checkout@v4

           - name: Set up Python
             uses: actions/setup-python@v4
             with:
               python-version: ${{ matrix.python-version }}

           - name: Install maturin
             run: pip install maturin

           - name: Build wheels
             run: maturin build --release --out dist/

           - name: Upload wheels
             uses: actions/upload-artifact@v3
             with:
               name: wheels
               path: dist/*.whl

       test-install:
         needs: build-wheels
         runs-on: ${{ matrix.os }}
         strategy:
           matrix:
             os: [ubuntu-latest, macos-latest, windows-latest]

         steps:
           - name: Download wheels
             uses: actions/download-artifact@v3
             with:
               name: wheels
               path: dist/

           - name: Test install
             run: |
               pip install dist/*.whl
               python -c "import mathhook; print(mathhook.Expression.integer(42))"
     ```

   - **ARM64 builds** (if needed):
     - Use `qemu` for ARM64 emulation on x86_64 runners
     - Or use self-hosted ARM64 runners (AWS Graviton, GitHub-hosted runners)

**Agent Delegation**:
```bash
/sc:spawn python-expert "Implement Wave 0: Platform Audit & Multi-Platform Build Strategy"
```

**Agent Prompt**:
```markdown
**Context**: You are the `python-expert` agent for MathHook CAS project.

Audit platform support and establish multi-platform wheel building strategy.

**Goal**: Ensure MathHook Python package can be installed on all major platforms before Wave 1 build.

**Tasks**:

1. **Platform audit**:
   - Test current maturin build on Linux, macOS, Windows
   - Identify supported architectures (x86_64, ARM64)
   - Document platform-specific blockers
   - Create compatibility matrix

2. **Multi-platform build strategy**:
   - Document maturin build commands for all platforms
   - Identify cross-compilation requirements
   - Plan GitHub Actions multi-platform CI
   - Document minimum Rust version

3. **Type stub generation**:
   - Research pyo3-stub-gen vs manual stubs
   - Generate .pyi files for all public APIs
   - Validate with mypy --strict
   - Include in wheel distribution

4. **GitHub Actions CI**:
   - Create multi-platform wheel build workflow
   - Test install on all platforms
   - Upload wheels as artifacts
   - Document deployment process

**Deliverables**:
- Platform compatibility matrix
- Multi-platform build documentation
- Type stub files (.pyi)
- GitHub Actions workflow (.github/workflows/python_build_multiplatform.yml)

**Quality Target**: 9+/10 - Comprehensive platform coverage, automated builds
```

**Verification Script** (`verify_wave_0_python_platform_audit.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 0 Verification: Platform Audit ==="

# 1. Check platform compatibility matrix
if [ ! -f ".mathhook_sessions/python_platform_compatibility.md" ]; then
    echo "❌ FAIL: Platform compatibility matrix not found"
    exit 1
fi
echo "✅ Platform compatibility documented"

# 2. Check multi-platform build documentation
if [ ! -f ".mathhook_sessions/python_multiplatform_build_strategy.md" ]; then
    echo "❌ FAIL: Build strategy not documented"
    exit 1
fi
echo "✅ Build strategy documented"

# 3. Check type stub files
if [ ! -f "crates/mathhook-python/mathhook/__init__.pyi" ]; then
    echo "❌ FAIL: Type stub files not found"
    exit 1
fi
echo "✅ Type stubs generated"

# 4. Validate type stubs with mypy
echo "Validating type stubs..."
python3 -m pip install --quiet mypy
python3 -m mypy --strict crates/mathhook-python/examples/test_types.py 2>&1 | tee mypy_output.txt
if grep -q "error:" mypy_output.txt; then
    echo "❌ FAIL: Type stubs have errors"
    exit 1
fi
echo "✅ Type stubs validated with mypy"

# 5. Check GitHub Actions workflow
if [ ! -f ".github/workflows/python_build_multiplatform.yml" ]; then
    echo "❌ FAIL: Multi-platform CI workflow not found"
    exit 1
fi
echo "✅ GitHub Actions workflow created"

# 6. Test build on current platform
echo "Testing maturin build on current platform..."
cd crates/mathhook-python
maturin build --release --out ../../dist/ 2>&1 | tee build_output.txt
if [ $? -ne 0 ]; then
    echo "❌ FAIL: maturin build failed"
    exit 1
fi
echo "✅ Build succeeds on current platform"

# 7. Test wheel install
echo "Testing wheel installation..."
python3 -m venv test_venv
source test_venv/bin/activate
pip install --quiet ../../dist/*.whl
python -c "import mathhook; assert mathhook.Expression.integer(42).to_simple() == '42'"
deactivate
rm -rf test_venv
echo "✅ Wheel install test passed"

echo ""
echo "=== Wave 0 Verification: PASSED ==="
echo "✅ Proceed to Wave 1: Build Infrastructure & Package"
```

**Deliverables**:
- `.mathhook_sessions/python_platform_compatibility.md`: Platform audit results
- `.mathhook_sessions/python_multiplatform_build_strategy.md`: Build strategy documentation
- `crates/mathhook-python/mathhook/*.pyi`: Type stub files
- `.github/workflows/python_build_multiplatform.yml`: Multi-platform CI

**Exit Criteria**:
- [ ] Platform audit completed for Linux, macOS, Windows (x86_64 and ARM64)
- [ ] Build strategy documented with platform-specific commands
- [ ] Type stubs generated and validated with mypy
- [ ] GitHub Actions multi-platform workflow created

**Risks**:
- ARM64 builds may require additional toolchain setup (mitigation: use GitHub-hosted ARM64 runners or QEMU)
- Platform-specific dependencies may block some platforms (mitigation: document and provide fallbacks)
- Type stub generation may be incomplete (mitigation: manual stubs for critical APIs)

**Dependencies**: None (Wave 0 is foundation)

**Unblocks**: Wave 1 (build infrastructure now aware of all target platforms)

**Critical Insight**: Multi-platform support is non-negotiable for PyPI packages. Auditing FIRST prevents discovering platform issues after Wave 5 when rushing to publish.

---

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

### Wave Completion Checklist
- [ ] Wave 0: Platform audit complete, multi-platform build strategy established, type stubs generated
- [ ] Wave 1: `pip install mathhook` works
- [ ] Wave 2: Pythonic API (operators, module functions)
- [ ] Wave 3: Jupyter integration with LaTeX rendering
- [ ] Wave 4: Complete feature parity with core
- [ ] Wave 5: Published to PyPI, >90% test coverage, documentation complete

### Quality Metrics
- All waves score ≥ 8/10
- Multi-platform wheels build successfully (Linux, macOS, Windows - x86_64 and ARM64)
- Type stubs validated with mypy --strict
- >90% test coverage
- Documentation complete

### Deliverables Checklist
- [ ] Wave 0: Platform compatibility matrix, build strategy, type stubs, multi-platform CI
- [ ] Working pip install
- [ ] Pythonic API
- [ ] Jupyter integration
- [ ] Complete feature parity
- [ ] PyPI publication
- [ ] Comprehensive tests
- [ ] Complete documentation

**Exit Criteria**: Python users can switch from SymPy to MathHook with minimal code changes and get 10-100x speedup + educational features on ANY major platform.

**Timeline**: 9-11 weeks to production PyPI package (added Wave 0 for platform audit)

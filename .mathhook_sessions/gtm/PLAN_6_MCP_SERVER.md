# Plan 6: MCP Server for AI Integration

**Priority**: 🤖 MEDIUM
**Timeline**: 3-4 weeks
**Waves**: 4
**Orchestrator**: `/sc:spawn`

## Executive Summary

**Research Finding**: FastMCP (Python) is the recommended approach for MathHook MCP server

**Recommendation**: Python MCP server using FastMCP framework
- **Why Python**: Existing PyO3 bindings ready, fastest time to market (2-3 weeks)
- **Why FastMCP**: Enterprise features, decorator-based, official Anthropic SDK
- **Future**: Rust MCP server for performance-critical deployments (optional)

**Market Opportunity**:
- Model Context Protocol (MCP) launched Nov 2024 by Anthropic
- Growing ecosystem of MCP-enabled tools (Claude Desktop, Cursor, Cline, etc.)
- LLMs are terrible at symbolic math (hallucinate variables) - MCP offloads to CAS

**Goal**: MathHook available as MCP tool for all AI assistants

**Reference**: [SymPy MCP server](https://github.com/sdiehl/sympy-mcp) by Stephen Diehl (31+ tools)

---

## Bootstrap Command

```bash
/sc:spawn rust-engineer "Execute Wave-Based MCP Server Implementation for MathHook"
```

**Orchestrator Prompt**:

```markdown
You are the Orchestrator for **MathHook MCP Server Implementation**.

**Context**: You are the `rust-engineer` agent from `.claude/agents/rust-engineer.md` - Expert Rust developer with Python expertise (PyO3) for creating MCP servers using FastMCP framework.

**Your Mission**: Execute a 4-wave plan to implement MCP server for MathHook using Python/FastMCP (recommended approach based on research).

**Mandatory Reading** (in this order):
1. `/Users/ahmedmashhour/.claude/agents/rust-engineer.md` - Your agent specification
2. `/Users/ahmedmashhour/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven wave-based methodology
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` - Project constraints
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_6_MCP_SERVER.md` - This plan

**Research Findings**: FastMCP (Python) is recommended for 2-3 week timeline. Rust MCP server (rmcp) is future optimization.

**5 Mandatory Rules**:
1. **You Are Always The Orchestrator** - Delegate to rust-engineer agents
2. **Sequential Waves, Parallel Agents** - Complete waves in order
3. **Mandatory Verification** - Each wave ends with verification
4. **Strict CLAUDE.md Enforcement** - Follow documentation standards
5. **Maintain Momentum** - Report after each wave
```

---

## Tool Namespacing Strategy

**Problem**: MCP tool names are global across all servers. Multiple servers with `solve` or `simplify` tools cause collisions.

**Solution**: Prefix all tools with `mathhook_` namespace

**Implementation**:

```python
# server.py
from fastmcp import FastMCP

# Configure namespace prefix
mcp = FastMCP(
    "mathhook",
    description="Educational CAS with symbolic mathematics",
    tool_prefix="mathhook_"  # All tools auto-prefixed
)

# Define tool WITHOUT prefix (FastMCP adds it automatically)
@mcp.tool()
async def solve_equation(equation: str, variable: str) -> str:
    """Solve algebraic equations symbolically."""
    # Implementation
```

**Result**:
- Tool name in code: `solve_equation`
- Tool name in MCP: `mathhook_solve_equation`
- AI invokes: `mathhook_solve_equation("x^2 - 4 = 0", "x")`

**Namespace Design**:
```yaml
mathhook_solve_equation       # vs sympy_solve
mathhook_compute_derivative   # vs sympy_diff
mathhook_explain_steps        # Unique to MathHook (no collision)
mathhook_simplify_expression  # vs sympy_simplify
```

**Benefits**:
1. **No Collisions**: Works alongside SymPy MCP, Mathematica MCP, etc.
2. **Clear Origin**: AI knows which CAS is being used
3. **Version Coexistence**: mathhook_v1 vs mathhook_v2 if needed

**Alternative Considered**: Custom namespace via `@mcp.tool(name="custom_name")` but auto-prefix is cleaner

---

## Wave Breakdown

### Wave 1: Python MCP Server Foundation (6-8 hours)

**Goal**: Basic MCP server with 5-10 core tools working

**Architecture**:
```
mathhook-mcp/              # New Python package
├── server.py              # Main MCP server
├── tools/
│   ├── algebra.py         # solve, simplify, expand, factor
│   ├── calculus.py        # derivative, integral, limit
│   ├── educational.py     # explain, show_steps
│   └── parsing.py         # parse_latex, format_latex
├── tests/
│   └── test_tools.py      # In-memory MCP testing
├── pyproject.toml
└── README.md
```

**Tasks**:

1. **Setup Package**:
   ```bash
   mkdir mathhook-mcp
   cd mathhook-mcp
   uv init --python 3.12
   uv add fastmcp mathhook
   ```

2. **Core Server Structure**:
   ```python
   # server.py
   from fastmcp import FastMCP
   from mathhook import Expression, symbol, solve, simplify

   mcp = FastMCP("mathhook", description="Educational CAS with symbolic mathematics")

   @mcp.tool()
   async def solve_equation(equation: str, variable: str, domain: str = "real") -> str:
       """
       Solve algebraic equations symbolically.

       Supports: linear, quadratic, polynomial, rational, transcendental.
       Returns exact symbolic solutions.

       Args:
           equation: Equation in LaTeX or string format (e.g., "x^2 - 5*x + 6 = 0")
           variable: Variable to solve for (e.g., "x")
           domain: "real" for real solutions, "complex" for complex

       Returns:
           JSON with solutions, LaTeX formatting, and optional steps
       """
       try:
           expr = Expression.parse(equation)
           solutions = solve(expr, variable, domain=domain)
           return {
               "success": True,
               "solutions": [str(sol) for sol in solutions],
               "latex": [sol.to_latex() for sol in solutions],
               "count": len(solutions)
           }
       except Exception as e:
           return {
               "success": False,
               "error": str(e),
               "suggestion": "Check equation syntax: use * for multiplication, ^ for exponents"
           }

   @mcp.tool()
   async def simplify_expression(expression: str, show_steps: bool = False) -> str:
       """Simplify mathematical expressions using algebraic rules."""
       expr = Expression.parse(expression)
       result = simplify(expr)
       return {
           "original": str(expr),
           "simplified": str(result),
           "latex": result.to_latex()
       }

   if __name__ == "__main__":
       mcp.run()
   ```

3. **Core Tools** (5-10 to start):
   - `solve_equation`: Solve algebraic equations
   - `simplify_expression`: Simplify expressions
   - `expand_expression`: Expand products
   - `factor_expression`: Factor polynomials
   - `compute_derivative`: Symbolic differentiation
   - `compute_integral`: Symbolic integration
   - `parse_latex`: LaTeX → Expression
   - `format_latex`: Expression → LaTeX

4. **Testing**:
   ```python
   # tests/test_tools.py
   from fastmcp.testing import FastMCPTransport
   import pytest

   @pytest.mark.asyncio
   async def test_solve_quadratic():
       async with FastMCPTransport(mcp) as client:
           result = await client.call_tool(
               "solve_equation",
               equation="x^2 - 5*x + 6 = 0",
               variable="x"
           )
           assert result["success"] == True
           assert len(result["solutions"]) == 2
   ```

5. **Claude Desktop Integration**:
   ```json
   // ~/Library/Application Support/Claude/claude_desktop_config.json
   {
     "mcpServers": {
       "mathhook": {
         "command": "python",
         "args": ["-m", "mathhook_mcp"],
         "env": {}
       }
     }
   }
   ```

**Deliverables**:
- Working MCP server with 5-10 tools
- Claude Desktop integration
- Test suite (in-memory transport)
- Basic documentation

---

### Wave 2: Comprehensive Tool Coverage (8-10 hours)

**Goal**: 30-40 mathematical tools across all categories

**Tool Categories** (inspired by SymPy MCP):

1. **Expression Management** (5 tools):
   - `parse_expression`: Multiple formats (LaTeX, Wolfram, standard)
   - `format_latex`: Expression → LaTeX
   - `format_wolfram`: Expression → Wolfram notation
   - `simplify_expression`: Algebraic simplification
   - `evaluate_numeric`: Numerical evaluation

2. **Equation Solving** (6 tools):
   - `solve_linear`: Linear equations
   - `solve_quadratic`: Quadratic equations
   - `solve_polynomial`: General polynomials
   - `solve_rational`: Rational equations
   - `solve_system`: System of equations
   - `solve_matrix_equation`: Matrix equations (MathHook specialty!)

3. **Calculus** (8 tools):
   - `compute_derivative`: Symbolic differentiation
   - `compute_integral`: Symbolic integration
   - `compute_definite_integral`: Definite integrals
   - `compute_limit`: Limit evaluation
   - `compute_series`: Taylor/power series
   - `check_continuity`: Continuity analysis
   - `find_critical_points`: Optimization
   - `compute_gradient`: Vector calculus

4. **Linear Algebra** (6 tools):
   - `matrix_determinant`: Determinant
   - `matrix_inverse`: Inverse
   - `matrix_eigenvalues`: Eigenvalues
   - `matrix_eigenvectors`: Eigenvectors
   - `solve_linear_system`: Matrix equation solving
   - `matrix_rank`: Rank computation

5. **Educational** (5 tools) - **MathHook's Unique Value**:
   - `explain_steps`: Step-by-step solution
   - `show_work`: Detailed explanation
   - `validate_solution`: Check student answer
   - `generate_similar`: Generate practice problems
   - `educational_context`: Get learning resources

6. **Algebra** (5 tools):
   - `expand_expression`: Expand products
   - `factor_expression`: Factor polynomials
   - `collect_terms`: Collect like terms
   - `substitute`: Variable substitution
   - `polynomial_division`: Divide polynomials

**Implementation Pattern**:
```python
# tools/calculus.py
@mcp.tool()
async def compute_derivative(
    expression: str,
    variable: str,
    order: int = 1,
    simplify_result: bool = True
) -> str:
    """
    Compute symbolic derivative.

    Uses chain rule, product rule, quotient rule automatically.
    Supports multiple orders (nth derivative).

    Examples:
        - First derivative: compute_derivative("x^2 + 3*x", "x", 1)
        - Second derivative: compute_derivative("sin(x)", "x", 2)
    """
    expr = Expression.parse(expression)
    result = derivative(expr, variable, order)
    if simplify_result:
        result = simplify(result)

    return {
        "derivative": str(result),
        "latex": result.to_latex(),
        "order": order,
        "variable": variable
    }
```

**Deliverables**:
- 30-40 comprehensive tools
- All categories covered
- Educational tools (unique to MathHook)
- Test coverage for all tools

---

### Wave 3: Error Handling & Educational Enhancement (6-8 hours)

**Goal**: Helpful errors and rich educational output

**Key Principle**: Help AI decide next steps (not just "failed")

**Error Handling Strategy**:

```python
@mcp.tool()
async def solve_equation(equation: str, variable: str, domain: str = "real") -> str:
    try:
        expr = Expression.parse(equation)
        solutions = solve(expr, variable, domain=domain)
        return {
            "success": True,
            "solutions": [str(sol) for sol in solutions],
            "explanation": "Found {count} solution(s) in {domain} domain"
        }
    except ParseError as e:
        return {
            "success": False,
            "error": "Equation parsing failed",
            "details": str(e),
            "suggestion": "Expected format: 'x^2 + 2*x + 1 = 0'. Use * for multiplication, ^ for exponents.",
            "example": "Try: 'x^2 - 5*x + 6 = 0'"
        }
    except DomainError as e:
        return {
            "success": False,
            "error": "Solution outside specified domain",
            "details": str(e),
            "suggestion": "Try domain='complex' for complex solutions",
            "alternative": "solve_equation(equation, variable, domain='complex')"
        }
    except NoSolutionsError:
        return {
            "success": True,
            "solutions": [],
            "explanation": "No solutions exist for this equation",
            "suggestion": "Check if equation is correct. Some equations have no solutions."
        }
```

**Educational Enhancement**:

```python
@mcp.tool()
async def explain_solution(
    problem: str,
    problem_type: str = "auto",
    detail_level: str = "standard"
) -> str:
    """
    Generate step-by-step explanation (MathHook's specialty).

    Args:
        problem: Mathematical problem
        problem_type: "equation", "derivative", "integral", "auto"
        detail_level: "brief", "standard", "detailed"

    Returns:
        Step-by-step solution with educational explanations
    """
    # Use MathHook's educational system
    if problem_type == "auto":
        problem_type = detect_problem_type(problem)

    explanation = generate_educational_explanation(problem, problem_type, detail_level)

    return {
        "steps": explanation.steps,
        "final_answer": explanation.result,
        "key_concepts": explanation.concepts,
        "common_mistakes": explanation.warnings,
        "practice_problems": explanation.similar_problems
    }
```

**Special Cases**:
- Division by zero → educational explanation
- No solutions → explain why
- Infinite solutions → explain condition
- Domain restrictions → explain limitations

**Response Size Optimization**:

**Problem**: Large symbolic expressions can exceed MCP message limits (typical limit: 100KB JSON)

**Examples of Large Responses**:
- Polynomial expansion: `(x + 1)^100` → 101 terms
- Matrix operations: 100x100 determinant → massive expression
- Infinite series: Taylor series with many terms

**Solution: Adaptive Response Strategy**

```python
MAX_RESPONSE_SIZE = 50_000  # 50KB safe limit for MCP messages

@mcp.tool()
async def solve_equation(equation: str, variable: str, max_solution_length: int = 1000) -> str:
    """Solve equation with automatic response truncation."""
    try:
        solutions = solve(expr, variable)

        # Check total response size
        response = format_solutions(solutions)
        if len(json.dumps(response)) > MAX_RESPONSE_SIZE:
            # Strategy 1: Truncate individual solutions
            response = truncate_large_solutions(solutions, max_solution_length)

        if len(json.dumps(response)) > MAX_RESPONSE_SIZE:
            # Strategy 2: Paginate solutions
            response = {
                "success": True,
                "solutions_count": len(solutions),
                "solutions_preview": solutions[:5],  # First 5
                "truncated": True,
                "suggestion": "Use mathhook_get_solution(index) for specific solutions",
                "pagination": {
                    "total": len(solutions),
                    "showing": 5,
                    "fetch_command": "mathhook_get_solution_batch(start=0, count=10)"
                }
            }

        return response
    except Exception as e:
        # Error responses are always small
        return format_error(e)
```

**Response Size Strategies**:

1. **Truncation with Context** (for individual expressions):
   ```python
   def truncate_expression(expr: Expression, max_length: int = 1000) -> dict:
       """Truncate large expression with useful metadata."""
       expr_str = str(expr)
       if len(expr_str) <= max_length:
           return {"value": expr_str, "truncated": False}

       return {
           "value": expr_str[:max_length] + "...",
           "truncated": True,
           "full_length": len(expr_str),
           "term_count": count_terms(expr),
           "suggestion": "Simplify result or use mathhook_export_to_file(result_id)"
       }
   ```

2. **Pagination** (for multiple solutions):
   ```python
   # Store large result set, return paged
   result_id = cache_result(solutions)
   return {
       "result_id": result_id,
       "total_count": len(solutions),
       "page_1": solutions[0:10],
       "next_page": "mathhook_get_page(result_id, page=2)"
   }
   ```

3. **Compression + Base64** (for structured data):
   ```python
   import gzip, base64

   def compress_large_result(data: dict) -> dict:
       """Compress large JSON responses."""
       json_bytes = json.dumps(data).encode('utf-8')
       if len(json_bytes) < MAX_RESPONSE_SIZE:
           return data  # No compression needed

       compressed = gzip.compress(json_bytes)
       return {
           "compressed": True,
           "format": "gzip+base64",
           "data": base64.b64encode(compressed).decode('ascii'),
           "original_size": len(json_bytes),
           "compressed_size": len(compressed)
       }
   ```

4. **Lazy Evaluation** (for expensive operations):
   ```python
   @mcp.tool()
   async def expand_expression_lazy(expression: str, preview_only: bool = True) -> str:
       """Expand expression with optional full evaluation."""
       expr = Expression.parse(expression)

       if preview_only:
           # Quick estimate without full expansion
           return {
               "preview": "Expansion would produce ~10000 terms",
               "estimated_size": "2.5 MB",
               "warning": "Large result - use preview_only=False to compute",
               "recommendation": "Consider factoring instead"
           }

       # User explicitly requested full expansion
       result = expand(expr)
       return format_with_size_check(result)
   ```

**Deliverables**:
- Helpful error messages
- Enhanced educational tools
- Special case handling
- Educational context for all operations
- **Response size optimization** (truncation, pagination, compression)
- **Large result handling** (warnings, previews, lazy evaluation)

---

### Wave 4: Staged Deployment & Distribution (4-6 hours)

**Goal**: Gradual rollout of MathHook MCP server with beta testing before public release

**Staged Rollout Strategy**:

**Phase 1: Internal Beta** (Week 1):
- Deploy to development team only
- Test all 30-40 tools with real Claude Desktop usage
- Fix critical bugs found during dogfooding
- Validation: All tools working in real workflows

**Phase 2: Private Beta** (Week 1):
- Invite 5-10 trusted users from MathHook community
- Provide beta installation instructions (pre-release PyPI)
- Collect feedback via GitHub Discussions
- Monitor for edge cases and unexpected usage patterns
- Validation: ≥80% positive feedback, <5 critical bugs

**Phase 3: Public Beta** (Week 2):
- Publish to PyPI with `0.1.0b1` version tag
- Announce in MCP community channels (beta label)
- Limit announcement scope (no major social media)
- Continue monitoring and rapid iteration
- Validation: 50+ beta users, stable error rate <1%

**Phase 4: General Availability** (Week 2):
- Publish stable `0.1.0` release to PyPI
- Submit to official MCP registry (lobehub.com/mcp)
- Full announcement (blog post, social media)
- Documentation complete and polished
- Validation: Smooth rollout, positive community reception

**Feature Flags for Gradual Rollout**:

```python
# server.py
import os

BETA_FEATURES = os.getenv("MATHHOOK_MCP_BETA", "false").lower() == "true"

@mcp.tool()
async def advanced_polynomial_solver(equation: str) -> str:
    """Advanced polynomial solving (BETA)."""
    if not BETA_FEATURES:
        return {
            "success": False,
            "error": "Feature not available",
            "message": "This is a beta feature. Set MATHHOOK_MCP_BETA=true to enable."
        }

    # Beta implementation
    return advanced_solve(equation)
```

**Rollback Strategy**:

```yaml
rollback_triggers:
  - Critical bug affecting >10% of users
  - Security vulnerability discovered
  - Data corruption or mathematical errors
  - MCP protocol incompatibility

rollback_process:
  1. Yank PyPI version (pip won't auto-install)
  2. Publish emergency patch as new version
  3. Notify users via GitHub release notes
  4. Post-mortem analysis and prevention plan
```

**Deployment Options**:

1. **Local (stdio transport)** - Primary:
   ```bash
   pip install mathhook-mcp
   # Configure in Claude Desktop config
   ```

2. **Self-Hosted (SSE transport)**:
   ```python
   # For team deployments
   mcp.run(transport="sse", host="0.0.0.0", port=8080)
   ```

3. **Docker**:
   ```dockerfile
   FROM python:3.12-slim
   RUN pip install mathhook-mcp
   CMD ["python", "-m", "mathhook_mcp"]
   ```

4. **FastMCP Cloud** (optional):
   ```bash
   fastmcp deploy server.py
   # Instant HTTPS endpoint
   ```

**Tasks**:

1. **PyPI Publication**:
   ```toml
   # pyproject.toml
   [project]
   name = "mathhook-mcp"
   version = "0.1.0"
   description = "MCP server for MathHook CAS"
   dependencies = ["fastmcp>=2.0", "mathhook>=0.1.0"]
   ```

2. **MCP Registry Listing**:
   - Submit to lobehub.com/mcp (official MCP registry)
   - Include: Description, installation, usage examples

3. **Documentation**:
   ```markdown
   # MathHook MCP Server

   ## Installation
   ```bash
   pip install mathhook-mcp
   ```

   ## Configuration
   Add to Claude Desktop config:
   ```json
   {
     "mcpServers": {
       "mathhook": {
         "command": "python",
         "args": ["-m", "mathhook_mcp"]
       }
     }
   }
   ```

   ## Available Tools
   - solve_equation: Solve algebraic equations
   - compute_derivative: Symbolic differentiation
   - ... (30+ tools)

   ## Examples
   Ask Claude:
   - "Solve x^2 - 5x + 6 = 0 using MathHook"
   - "Find the derivative of sin(x)*cos(x) with steps"
   ```

4. **Demo Video** (3 min):
   - Installation
   - Configuration
   - Using with Claude Desktop
   - Example queries

5. **Announcement**:
   - Blog post: "MathHook + Claude: Explainable Symbolic Math for AI"
   - Share on MCP community channels

**Deliverables**:
- PyPI package published
- MCP registry listing
- Documentation complete
- Docker image available
- Demo video

---

## Architecture Decision

**Phase 1: Python MCP Server** (This Plan)
- **Timeline**: 3-4 weeks
- **Technology**: FastMCP + PyO3 bindings
- **Benefits**: Fast to market, leverages existing Python integration
- **Trade-offs**: Python performance overhead (acceptable for most queries)

**Phase 2: Rust MCP Server** (Future, Optional)
- **Timeline**: 4-6 weeks (after Python proven)
- **Technology**: rmcp SDK (Rust)
- **When**: If performance becomes bottleneck OR want single-binary deployment
- **Benefits**: 10-100x faster, single binary, lower memory
- **Trade-offs**: More development time, less ecosystem maturity

**Hybrid Approach** (Long-Term):
```python
# FastMCP proxy bridging Python and Rust
python_server = FastMCP("mathhook-simple")
rust_server = FastMCP.as_proxy("mathhook-advanced", transport="stdio")

# Python for 90% of queries
python_server.mount_tools(simple_algebra_tools)

# Rust for complex polynomial solving, large matrix ops
python_server.mount(rust_server, prefix="advanced_")
```

---

## Success Criteria

**Wave Completion**:
- [ ] Wave 1: 5-10 core tools working, Claude Desktop integration
- [ ] Wave 2: 30-40 comprehensive tools across all categories
- [ ] Wave 3: Helpful errors, educational enhancements
- [ ] Wave 4: PyPI published, MCP registry listed

**Quality Metrics**:
- All tools tested with in-memory transport
- Error messages help AI make next decision
- Educational tools demonstrate MathHook's unique value
- Documentation complete and clear

**Adoption Metrics**:
- Listed on official MCP registry
- 100+ pip installs in first month
- Positive user feedback
- Used in real Claude Desktop workflows

---

## Competitive Positioning

**SymPy MCP**: Exists, 31+ tools, good reference
**MathHook MCP Advantages**:
1. **10-100x faster** (Rust core vs Python)
2. **Educational mode** (unique differentiator)
3. **Better error handling** (helps AI decide next steps)
4. **Non-commutative algebra** (matrices, operators, quaternions)

**Use Cases Python Can't Handle Well**:
- Large polynomial computations (Rust 100x faster)
- Matrix operations (SIMD optimization)
- Long-running symbolic computations (async Rust)

---

## Future Enhancements

**After Wave 4**:
- [ ] Rust MCP server for performance-critical use cases
- [ ] Resources (access to mathematical knowledge base)
- [ ] Prompts (templates for common mathematical tasks)
- [ ] Streaming responses for long computations
- [ ] OAuth authentication for hosted deployments

**Timeline**: 3-4 weeks for complete Python MCP server

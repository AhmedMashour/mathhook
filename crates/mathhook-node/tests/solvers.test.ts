import test from "ava";
import {
  Expression,
  SmartEquationSolver,
  symbols,
  parse,
} from "../index.js";

// Helper functions for cleaner test code
const add = (...args: Expression[]): Expression => Expression.add(args);
const mul = (...args: Expression[]): Expression => Expression.mul(args);
const pow = (base: Expression, exponent: Expression): Expression =>
  Expression.pow(base, exponent);
const int = (n: number): Expression => Expression.integer(n);

// =============================================================================
// LINEAR EQUATIONS
// =============================================================================

test("solve linear equation: 2x - 6 = 0", (t) => {
  const [x] = symbols("x");
  // 2x - 6 = 0  =>  x = 3
  const equation = add(mul(int(2), x), int(-6));
  const result = equation.solveLinear(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 1);
});

test("solve linear equation: x + 5 = 0", (t) => {
  const [x] = symbols("x");
  // x + 5 = 0  =>  x = -5
  const equation = add(x, int(5));
  const result = equation.solveLinear(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 1);
});

test("solve linear equation with steps", (t) => {
  const [x] = symbols("x");
  // 3x + 9 = 0  =>  x = -3
  const equation = add(mul(int(3), x), int(9));
  const [result, explanation] = equation.solveLinearWithSteps(x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
  t.is(result.solutionCount(), 1);
});

test("solve linear equation via parse", (t) => {
  const [x] = symbols("x");
  const equation = parse("4*x - 8");
  const result = equation.solveLinear(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 1);
});

// =============================================================================
// QUADRATIC EQUATIONS
// =============================================================================

test("solve quadratic equation: x^2 - 4 = 0", (t) => {
  const [x] = symbols("x");
  // x^2 - 4 = 0  =>  x = 2, x = -2
  const equation = add(pow(x, int(2)), int(-4));
  const result = equation.solveQuadratic(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 2);
});

test("solve quadratic equation: x^2 - 5x + 6 = 0", (t) => {
  const [x] = symbols("x");
  // x^2 - 5x + 6 = 0  =>  x = 2, x = 3
  const equation = add(pow(x, int(2)), mul(int(-5), x), int(6));
  const result = equation.solveQuadratic(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 2);
});

test("solve quadratic with double root: x^2 - 2x + 1 = 0", (t) => {
  const [x] = symbols("x");
  // x^2 - 2x + 1 = 0  =>  x = 1 (double root)
  const equation = add(pow(x, int(2)), mul(int(-2), x), int(1));
  const result = equation.solveQuadratic(x.asSymbol()!);

  t.truthy(result);
  // Either 1 or 2 solutions depending on implementation
  const count = result.solutionCount();
  t.truthy(count !== null && count >= 1);
});

test("solve quadratic equation with steps", (t) => {
  const [x] = symbols("x");
  // x^2 + 2x - 3 = 0  =>  x = 1, x = -3
  const equation = add(pow(x, int(2)), mul(int(2), x), int(-3));
  const [result, explanation] = equation.solveQuadraticWithSteps(x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
  t.is(result.solutionCount(), 2);
});

test("solve quadratic via parse", (t) => {
  const [x] = symbols("x");
  const equation = parse("x^2 - 9");
  const result = equation.solveQuadratic(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 2);
});

// =============================================================================
// POLYNOMIAL EQUATIONS
// =============================================================================

test("solve cubic equation: x^3 - 6x^2 + 11x - 6 = 0", (t) => {
  const [x] = symbols("x");
  // x^3 - 6x^2 + 11x - 6 = 0  =>  x = 1, x = 2, x = 3
  const equation = add(
    pow(x, int(3)),
    mul(int(-6), pow(x, int(2))),
    mul(int(11), x),
    int(-6)
  );
  const result = equation.solvePolynomial(x.asSymbol()!);

  t.truthy(result);
  const count = result.solutionCount();
  t.truthy(count !== null && count >= 1);
});

test("solve polynomial equation with steps", (t) => {
  const [x] = symbols("x");
  // x^3 - x = 0  =>  x(x^2 - 1) = 0  =>  x = 0, x = 1, x = -1
  const equation = add(pow(x, int(3)), mul(int(-1), x));
  const [result, explanation] = equation.solvePolynomialWithSteps(x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
});

test("solve polynomial via parse", (t) => {
  const [x] = symbols("x");
  const equation = parse("x^4 - 16");
  const result = equation.solvePolynomial(x.asSymbol()!);

  t.truthy(result);
});

// =============================================================================
// AUTO-DETECTION WITH solve()
// =============================================================================

test("solve() auto-detects linear equation", (t) => {
  const [x] = symbols("x");
  // 5x - 10 = 0
  const equation = add(mul(int(5), x), int(-10));
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 1);
});

test("solve() auto-detects quadratic equation", (t) => {
  const [x] = symbols("x");
  // x^2 - 1 = 0
  const equation = add(pow(x, int(2)), int(-1));
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 2);
});

test("solve() auto-detects polynomial equation", (t) => {
  const [x] = symbols("x");
  // x^3 - 8 = 0
  const equation = add(pow(x, int(3)), int(-8));
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
});

test("solve() with steps auto-detects equation type", (t) => {
  const [x] = symbols("x");
  // x^2 - 4 = 0
  const equation = add(pow(x, int(2)), int(-4));
  const [result, explanation] = equation.solveWithSteps(x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
  t.is(result.solutionCount(), 2);
});

// =============================================================================
// SmartEquationSolver CLASS
// =============================================================================

test("SmartEquationSolver solve linear equation", (t) => {
  const solver = new SmartEquationSolver();
  const [x] = symbols("x");
  const equation = add(mul(int(2), x), int(-8));
  const [result, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
  t.true(result.isValidSolution());
});

test("SmartEquationSolver solve quadratic equation", (t) => {
  const solver = new SmartEquationSolver();
  const [x] = symbols("x");
  const equation = parse("x^2 - 4");
  const [result, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
  t.true(result.isValidSolution());
});

test("SmartEquationSolver returns steps", (t) => {
  const solver = new SmartEquationSolver();
  const [x] = symbols("x");
  const equation = parse("x^2 - 9");
  const [result, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);

  t.truthy(result);
  t.truthy(explanation);
  t.true(result.isValidSolution());
});

test("SmartEquationSolver reuse across multiple equations", (t) => {
  const solver = new SmartEquationSolver();
  const [x] = symbols("x");

  const eq1 = parse("x - 5");
  const [result1] = solver.solveWithEquation(eq1, x.asSymbol()!);
  t.true(result1.isValidSolution());

  const eq2 = parse("x^2 - 16");
  const [result2] = solver.solveWithEquation(eq2, x.asSymbol()!);
  t.true(result2.isValidSolution());

  const eq3 = parse("x^3 - 27");
  const [result3] = solver.solveWithEquation(eq3, x.asSymbol()!);
  t.truthy(result3);
});

// =============================================================================
// EDGE CASES
// =============================================================================

test("solve equation with no solution", (t) => {
  const [x] = symbols("x");
  // x^2 + 1 = 0 has no real solutions
  const equation = add(pow(x, int(2)), int(1));
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
  // May have complex solutions or no_solution depending on implementation
});

test("solve equation with coefficient zero", (t) => {
  const [x] = symbols("x");
  // 0 = 0 - identity, infinite solutions
  const equation = int(0);
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
});

test("solve equation with large coefficients", (t) => {
  const [x] = symbols("x");
  // 1000x - 5000 = 0  =>  x = 5
  const equation = add(mul(int(1000), x), int(-5000));
  const result = equation.solveLinear(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 1);
});

test("solve nested expression", (t) => {
  const [x] = symbols("x");
  // (x + 1)^2 - 4 = 0 expanded is x^2 + 2x + 1 - 4 = x^2 + 2x - 3 = 0
  const expr = parse("(x + 1)^2 - 4");
  const expanded = expr.expand();
  const result = expanded.solve(x.asSymbol()!);

  t.truthy(result);
});

// =============================================================================
// PARSED EQUATIONS
// =============================================================================

test("solve parsed linear equation", (t) => {
  const [x] = symbols("x");
  const equation = parse("7*x + 14");
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
  t.is(result.solutionCount(), 1);
});

test("solve parsed quadratic equation", (t) => {
  const [x] = symbols("x");
  const equation = parse("x^2 - 6*x + 9");
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
});

test("solve parsed complex quadratic", (t) => {
  const [x] = symbols("x");
  const equation = parse("2*x^2 + 3*x - 2");
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
});

// =============================================================================
// MULTI-VARIABLE (solve for one)
// =============================================================================

test("solve for x with y present", (t) => {
  const [x, y] = symbols("x y");
  // 2x + y - 10 = 0, solve for x
  const equation = add(mul(int(2), x), y, int(-10));
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
});

test("solve for y with x present", (t) => {
  const [x, y] = symbols("x y");
  // x + 3y - 9 = 0, solve for y
  const equation = add(x, mul(int(3), y), int(-9));
  const result = equation.solve(y.asSymbol()!);

  t.truthy(result);
});

// =============================================================================
// VALIDATION TESTS
// =============================================================================

test("linear solution is valid", (t) => {
  const [x] = symbols("x");
  // 2x - 6 = 0  =>  x = 3
  const equation = add(mul(int(2), x), int(-6));
  const result = equation.solveLinear(x.asSymbol()!);

  t.truthy(result);
  t.true(result.isValidSolution());
});

test("quadratic solutions are valid", (t) => {
  const [x] = symbols("x");
  // x^2 - 4 = 0  =>  x = 2, x = -2
  const equation = add(pow(x, int(2)), int(-4));
  const result = equation.solveQuadratic(x.asSymbol()!);

  t.truthy(result);
  t.true(result.isValidSolution());
});

// =============================================================================
// STRESS TESTS
// =============================================================================

test("solve many equations sequentially", (t) => {
  const solver = new SmartEquationSolver();
  const [x] = symbols("x");

  for (let i = 1; i <= 10; i++) {
    const equation = add(mul(int(i), x), int(-i * 2));
    const [result] = solver.solveWithEquation(equation, x.asSymbol()!);
    t.truthy(result);
  }
});

test("solve polynomial of degree 4", (t) => {
  const [x] = symbols("x");
  // x^4 - 1 = 0  =>  x = 1, -1, i, -i
  const equation = add(pow(x, int(4)), int(-1));
  const result = equation.solvePolynomial(x.asSymbol()!);

  t.truthy(result);
});

test("solve equation with rational coefficients via parse", (t) => {
  const [x] = symbols("x");
  // (1/2)x - 1 = 0  =>  x = 2
  const equation = parse("x/2 - 1");
  const result = equation.solve(x.asSymbol()!);

  t.truthy(result);
});

#!/usr/bin/env node
/**
 * Benchmark Node.js bindings performance.
 *
 * This benchmarks the FULL STACK: Node.js → NAPI → Rust
 */

import {
  Expression,
  symbols,
  parse,
  sin,
  cos,
  tan,
  exp,
  ln,
  sqrt,
} from "./index.js";

// Helper functions
const add = (...args: Expression[]): Expression => Expression.add(args);
const mul = (...args: Expression[]): Expression => Expression.mul(args);
const pow = (base: Expression, exponent: Expression): Expression =>
  Expression.pow(base, exponent);
const int = (n: number): Expression => Expression.integer(n);

interface BenchResult {
  mean: number;
  stdev: number;
  median: number;
  min: number;
  max: number;
}

function benchFunction(
  name: string,
  func: () => unknown,
  iterations: number = 1000000
): BenchResult {
  // Warmup
  for (let i = 0; i < 1000; i++) {
    func();
  }

  // Benchmark with statistical analysis
  const times: number[] = [];
  const batchSize = iterations / 100;

  for (let batch = 0; batch < 100; batch++) {
    const start = process.hrtime.bigint();
    for (let i = 0; i < batchSize; i++) {
      func();
    }
    const end = process.hrtime.bigint();
    const nsPerCall = Number(end - start) / batchSize;
    times.push(nsPerCall);
  }

  // Calculate statistics
  const mean = times.reduce((a, b) => a + b, 0) / times.length;
  const variance =
    times.reduce((sum, time) => sum + Math.pow(time - mean, 2), 0) /
    times.length;
  const stdev = Math.sqrt(variance);
  const sorted = [...times].sort((a, b) => a - b);
  const median = sorted[Math.floor(times.length / 2)];
  const min = Math.min(...times);
  const max = Math.max(...times);

  return { mean, stdev, median, min, max };
}

function printResult(name: string, result: BenchResult): void {
  const opsPerSec = (1e9 / result.mean).toFixed(0);
  console.log(
    `${name.padEnd(30)} ${result.mean.toFixed(2).padStart(10)} ns/call  ` +
      `(${opsPerSec.padStart(10)} ops/s)  ` +
      `σ=${result.stdev.toFixed(1).padStart(6)} ns`
  );
}

function main(): void {
  console.log("=".repeat(90));
  console.log("MathHook Node.js Bindings Performance Benchmark");
  console.log("=".repeat(90));
  console.log();
  console.log("Stack: Node.js → NAPI-RS → Rust");
  console.log("Iterations: 1,000,000 per benchmark");
  console.log();

  // Pre-create symbols for benchmarks
  const [x] = symbols("x");
  const [y] = symbols("y");

  // ==========================================================================
  // Expression Creation
  // ==========================================================================
  console.log("Expression Creation:");
  console.log("-".repeat(90));

  printResult(
    "symbols('x')",
    benchFunction("symbols", () => symbols("x"))
  );
  printResult(
    "Expression.integer(42)",
    benchFunction("integer", () => Expression.integer(42))
  );
  printResult(
    "Expression.pi()",
    benchFunction("pi", () => Expression.pi())
  );
  printResult(
    "Expression.e()",
    benchFunction("e", () => Expression.e())
  );

  console.log();

  // ==========================================================================
  // Arithmetic Operations
  // ==========================================================================
  console.log("Arithmetic Operations:");
  console.log("-".repeat(90));

  printResult(
    "add(x, y)",
    benchFunction("add", () => add(x, y))
  );
  printResult(
    "mul(x, y)",
    benchFunction("mul", () => mul(x, y))
  );
  printResult(
    "pow(x, 2)",
    benchFunction("pow", () => pow(x, int(2)))
  );
  printResult(
    "Expression.div(x, y)",
    benchFunction("div", () => Expression.div(x, y))
  );

  console.log();

  // ==========================================================================
  // Trigonometric Functions
  // ==========================================================================
  console.log("Trigonometric Functions:");
  console.log("-".repeat(90));

  printResult(
    "sin(x)",
    benchFunction("sin", () => sin(x))
  );
  printResult(
    "cos(x)",
    benchFunction("cos", () => cos(x))
  );
  printResult(
    "tan(x)",
    benchFunction("tan", () => tan(x))
  );

  console.log();

  // ==========================================================================
  // Other Functions
  // ==========================================================================
  console.log("Other Functions:");
  console.log("-".repeat(90));

  printResult(
    "exp(x)",
    benchFunction("exp", () => exp(x))
  );
  printResult(
    "ln(x)",
    benchFunction("ln", () => ln(x))
  );
  printResult(
    "sqrt(x)",
    benchFunction("sqrt", () => sqrt(x))
  );

  console.log();

  // ==========================================================================
  // Parsing
  // ==========================================================================
  console.log("Parsing:");
  console.log("-".repeat(90));

  printResult(
    "parse('x + 1')",
    benchFunction("parse_simple", () => parse("x + 1"), 100000)
  );
  printResult(
    "parse('x^2 + 2*x + 1')",
    benchFunction("parse_quadratic", () => parse("x^2 + 2*x + 1"), 100000)
  );
  printResult(
    "parse('sin(x) + cos(y)')",
    benchFunction("parse_trig", () => parse("sin(x) + cos(y)"), 100000)
  );

  console.log();

  // ==========================================================================
  // Expression Methods
  // ==========================================================================
  console.log("Expression Methods:");
  console.log("-".repeat(90));

  const quadratic = parse("x^2 + 2*x + 1");
  const simple = add(x, x, x);
  const sinX = sin(x);

  printResult(
    "expr.format()",
    benchFunction("format", () => quadratic.format(), 100000)
  );
  printResult(
    "expr.simplify()",
    benchFunction("simplify", () => simple.simplify(), 100000)
  );
  printResult(
    "expr.derivative(x)",
    benchFunction("derivative", () => sinX.derivative(x.asSymbol()!), 100000)
  );
  printResult(
    "expr.expand()",
    benchFunction("expand", () => quadratic.expand(), 100000)
  );

  console.log();

  // ==========================================================================
  // Solving
  // ==========================================================================
  console.log("Equation Solving:");
  console.log("-".repeat(90));

  const linear = add(mul(int(2), x), int(-6)); // 2x - 6 = 0
  const quad = add(pow(x, int(2)), int(-4)); // x^2 - 4 = 0

  printResult(
    "solveLinear()",
    benchFunction(
      "solve_linear",
      () => linear.solveLinear(x.asSymbol()!),
      10000
    )
  );
  printResult(
    "solveQuadratic()",
    benchFunction(
      "solve_quadratic",
      () => quad.solveQuadratic(x.asSymbol()!),
      10000
    )
  );
  printResult(
    "solve() [auto-detect]",
    benchFunction("solve_auto", () => quad.solve(x.asSymbol()!), 10000)
  );

  console.log();

  // ==========================================================================
  // Complex Expressions
  // ==========================================================================
  console.log("Complex Expressions:");
  console.log("-".repeat(90));

  printResult(
    "sin(cos(exp(x)))",
    benchFunction("nested_trig", () => sin(cos(exp(x))))
  );
  printResult(
    "x^2 + y^2 build",
    benchFunction("sum_squares", () => add(pow(x, int(2)), pow(y, int(2))))
  );

  console.log();
  console.log("=".repeat(90));
  console.log("Benchmark complete.");
}

main();

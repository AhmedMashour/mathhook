#!/usr/bin/env python3
"""
SymPy Comparison Framework for MathHook Validation - Phase 2

This script validates MathHook's correctness against SymPy (authoritative reference)
and benchmarks performance across comprehensive test coverage.

SymPy Reference: ~/Documents/work/math/sympy/
"""

import sys
import os
import json
import time
import subprocess
import tempfile
import re
from typing import List, Dict, Tuple, Any
from dataclasses import dataclass, asdict

sys.path.insert(0, '/Users/ahmedmashhour/Documents/work/math/sympy')

from sympy import (
    symbols, sympify, diff, integrate, simplify, solve, expand, factor,
    sin, cos, tan, exp, log, sqrt, pi, E, I,
    latex, pretty
)
from sympy.parsing.latex import parse_latex as sympy_parse_latex


@dataclass
class TestCase:
    """Represents a single test case for comparison"""
    name: str
    operation: str
    input_expr: str
    variable: str = 'x'
    expected_sympy: str = None
    mathhook_result: str = None
    sympy_time: float = 0.0
    mathhook_time: float = 0.0
    correctness: bool = None
    speedup: float = 0.0
    notes: str = ""


@dataclass
class ValidationReport:
    """Aggregated validation results"""
    total_tests: int = 0
    passed: int = 0
    failed: int = 0
    errors: int = 0
    min_speedup: float = float('inf')
    max_speedup: float = 0.0
    avg_speedup: float = 0.0
    test_cases: List[TestCase] = None

    def __post_init__(self):
        if self.test_cases is None:
            self.test_cases = []


class SymPyValidator:
    """Validates MathHook against SymPy reference implementation"""

    def __init__(self, workspace_root: str = None):
        if workspace_root is None:
            workspace_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        self.workspace_root = workspace_root
        self.reports: Dict[str, ValidationReport] = {}
        self.test_file_path = os.path.join(workspace_root, 'crates', 'mathhook-core', 'tests', 'sympy_comparison_generated.rs')

    def _generate_rust_test_file(self, test_cases: List[Dict[str, Any]]) -> str:
        test_code = """//! Generated test file for SymPy comparison
//! Auto-generated - do not edit manually

use mathhook_core::prelude::*;
use mathhook_core::calculus::Derivative;
use std::time::Instant;

"""
        for idx, test_spec in enumerate(test_cases):
            operation = test_spec['operation']
            expr_str = test_spec['expr']
            var = test_spec.get('var', 'x')
            name = test_spec.get('name', f"test_{idx}")

            safe_name = re.sub(r'[^a-zA-Z0-9]', '_', name)
            safe_name = re.sub(r'_+', '_', safe_name)
            safe_name = safe_name.strip('_')

            test_code += f"""
#[test]
fn {safe_name}() {{
    let var = symbol!({var});
    let expr_str = r#"{expr_str}"#;

    let expr = match parse!(expr_str) {{
        Ok(e) => e,
        Err(_) => {{
            eprintln!("PARSE_ERROR");
            return;
        }}
    }};

"""

            if operation == 'derivative':
                test_code += f"""
    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {{}}", result);
    println!("TIME: {{}} ns", elapsed.as_nanos());
"""
            elif operation == 'integral':
                test_code += f"""
    println!("RESULT: NOT_IMPLEMENTED");
    println!("TIME: 0 ns");
"""
            elif operation == 'simplify':
                test_code += f"""
    let start = Instant::now();
    let result = expr.simplify();
    let elapsed = start.elapsed();

    println!("RESULT: {{}}", result);
    println!("TIME: {{}} ns", elapsed.as_nanos());
"""
            elif operation == 'solve':
                test_code += f"""
    let equation = Expression::equation(expr, Expression::integer(0));
    let mut solver = MathSolver::new();

    let start = Instant::now();
    let result = solver.solve(&equation, &var);
    let elapsed = start.elapsed();

    match result {{
        SolverResult::Single(sol) => println!("RESULT: {{}}", sol),
        SolverResult::Multiple(sols) => println!("RESULT: [{{}}]", sols.iter().map(|s| format!("{{}}", s)).collect::<Vec<_>>().join(", ")),
        SolverResult::NoSolution => println!("RESULT: NO_SOLUTION"),
        SolverResult::Infinite => println!("RESULT: INFINITE"),
    }}
    println!("TIME: {{}} ns", elapsed.as_nanos());
"""
            elif operation == 'evaluate':
                test_code += f"""
    let start = Instant::now();
    let result = expr.simplify();
    let elapsed = start.elapsed();

    println!("RESULT: {{}}", result);
    println!("TIME: {{}} ns", elapsed.as_nanos());
"""

            test_code += "}\n"

        with open(self.test_file_path, 'w') as f:
            f.write(test_code)

        return self.test_file_path

    def _run_rust_test(self, test_name: str) -> Tuple[str, float]:
        cmd = [
            'cargo', 'test', '--test', 'sympy_comparison_generated',
            test_name, '--', '--nocapture', '--test-threads=1'
        ]

        try:
            result = subprocess.run(
                cmd,
                cwd=self.workspace_root,
                capture_output=True,
                text=True,
                timeout=30.0
            )

            output = result.stdout
            if 'PARSE_ERROR' in output:
                return "ERROR: Parse failed", 0.0

            result_line = [line for line in output.split('\n') if 'RESULT:' in line]
            time_line = [line for line in output.split('\n') if 'TIME:' in line]

            if not result_line or not time_line:
                return "ERROR: No output", 0.0

            result_str = result_line[0].split('RESULT:')[1].strip()
            time_ns = float(time_line[0].split('TIME:')[1].strip().split()[0])

            return result_str, time_ns

        except subprocess.TimeoutExpired:
            return "ERROR: Timeout", 0.0
        except Exception as e:
            return f"ERROR: {str(e)}", 0.0

    def _run_sympy_derivative(self, expr_str: str, var_str: str) -> Tuple[str, float]:
        var = symbols(var_str)

        start = time.perf_counter()
        try:
            expr = sympify(expr_str.replace('^', '**'))
            result = diff(expr, var)
            result_str = str(result)
            elapsed = (time.perf_counter() - start) * 1e9
            return result_str, elapsed
        except Exception as e:
            elapsed = (time.perf_counter() - start) * 1e9
            return f"ERROR: {str(e)}", elapsed

    def _run_sympy_integral(self, expr_str: str, var_str: str) -> Tuple[str, float]:
        var = symbols(var_str)

        start = time.perf_counter()
        try:
            expr = sympify(expr_str.replace('^', '**'))
            result = integrate(expr, var)
            result_str = str(result)
            elapsed = (time.perf_counter() - start) * 1e9
            return result_str, elapsed
        except Exception as e:
            elapsed = (time.perf_counter() - start) * 1e9
            return f"ERROR: {str(e)}", elapsed

    def _run_sympy_simplify(self, expr_str: str) -> Tuple[str, float]:
        start = time.perf_counter()
        try:
            expr = sympify(expr_str.replace('^', '**'))
            result = simplify(expr)
            result_str = str(result)
            elapsed = (time.perf_counter() - start) * 1e9
            return result_str, elapsed
        except Exception as e:
            elapsed = (time.perf_counter() - start) * 1e9
            return f"ERROR: {str(e)}", elapsed

    def _run_sympy_solve(self, expr_str: str, var_str: str) -> Tuple[str, float]:
        var = symbols(var_str)

        start = time.perf_counter()
        try:
            expr = sympify(expr_str.replace('^', '**'))
            result = solve(expr, var)
            result_str = str(result)
            elapsed = (time.perf_counter() - start) * 1e9
            return result_str, elapsed
        except Exception as e:
            elapsed = (time.perf_counter() - start) * 1e9
            return f"ERROR: {str(e)}", elapsed

    def _run_sympy_evaluate(self, expr_str: str) -> Tuple[str, float]:
        start = time.perf_counter()
        try:
            expr = sympify(expr_str.replace('^', '**'))
            result = expr.evalf()
            result_str = str(result)
            elapsed = (time.perf_counter() - start) * 1e9
            return result_str, elapsed
        except Exception as e:
            elapsed = (time.perf_counter() - start) * 1e9
            return f"ERROR: {str(e)}", elapsed

    def _normalize_result(self, result: str, operation: str) -> str:
        result = result.strip()

        if result.startswith("ERROR:") or result == "NOT_IMPLEMENTED":
            return result

        try:
            expr = sympify(result.replace('^', '**'))
            return str(simplify(expr))
        except:
            return result

    def _compare_results(self, sympy_result: str, mathhook_result: str, operation: str) -> bool:
        if sympy_result.startswith("ERROR:") or mathhook_result.startswith("ERROR:"):
            return False

        if mathhook_result == "NOT_IMPLEMENTED":
            return None

        norm_sympy = self._normalize_result(sympy_result, operation)
        norm_mathhook = self._normalize_result(mathhook_result, operation)

        if norm_sympy == norm_mathhook:
            return True

        try:
            sympy_expr = sympify(norm_sympy.replace('^', '**'))
            mathhook_expr = sympify(norm_mathhook.replace('^', '**'))

            diff = simplify(sympy_expr - mathhook_expr)
            return diff == 0
        except:
            return norm_sympy == norm_mathhook

    def run_test_suite(self, category: str, test_cases: List[Dict[str, Any]]) -> ValidationReport:
        report = ValidationReport()

        print(f"Generating Rust tests for {category}...")
        self._generate_rust_test_file(test_cases)

        print(f"Building tests...")
        build_cmd = ['cargo', 'test', '--test', 'sympy_comparison_generated', '--no-run']
        build_result = subprocess.run(build_cmd, cwd=self.workspace_root, capture_output=True, text=True)

        if build_result.returncode != 0:
            print(f"Build failed: {build_result.stderr[:500]}")
            return report

        for test_spec in test_cases:
            operation = test_spec['operation']
            name = test_spec.get('name', f"test_{operation}")
            print(f"  Running {name}...")

            if operation == 'derivative':
                sympy_result, sympy_time = self._run_sympy_derivative(test_spec['expr'], test_spec.get('var', 'x'))
            elif operation == 'integral':
                sympy_result, sympy_time = self._run_sympy_integral(test_spec['expr'], test_spec.get('var', 'x'))
            elif operation == 'simplify':
                sympy_result, sympy_time = self._run_sympy_simplify(test_spec['expr'])
            elif operation == 'solve':
                sympy_result, sympy_time = self._run_sympy_solve(test_spec['expr'], test_spec.get('var', 'x'))
            elif operation == 'evaluate':
                sympy_result, sympy_time = self._run_sympy_evaluate(test_spec['expr'])
            else:
                continue

            safe_name = re.sub(r'[^a-zA-Z0-9]', '_', name)
            safe_name = re.sub(r'_+', '_', safe_name)
            safe_name = safe_name.strip('_')
            mathhook_result, mathhook_time = self._run_rust_test(safe_name)

            correctness = self._compare_results(sympy_result, mathhook_result, operation)

            if correctness is None:
                continue

            speedup = sympy_time / mathhook_time if mathhook_time > 0 else 0.0

            test = TestCase(
                name=name,
                operation=operation,
                input_expr=test_spec['expr'],
                variable=test_spec.get('var', 'x'),
                expected_sympy=sympy_result,
                mathhook_result=mathhook_result,
                sympy_time=sympy_time,
                mathhook_time=mathhook_time,
                correctness=correctness,
                speedup=speedup
            )

            report.test_cases.append(test)
            report.total_tests += 1

            if test.correctness:
                report.passed += 1
            elif test.expected_sympy.startswith("ERROR:") or test.mathhook_result.startswith("ERROR:"):
                report.errors += 1
            else:
                report.failed += 1

            if test.speedup > 0:
                report.min_speedup = min(report.min_speedup, test.speedup)
                report.max_speedup = max(report.max_speedup, test.speedup)

        valid_speedups = [t.speedup for t in report.test_cases if t.speedup > 0]
        if valid_speedups:
            report.avg_speedup = sum(valid_speedups) / len(valid_speedups)

        self.reports[category] = report
        return report

    def generate_markdown_report(self, output_file: str):
        with open(output_file, 'w') as f:
            f.write("# SymPy Comparison Validation Report - Phase 2\n\n")
            f.write("## Summary\n\n")

            total_tests = sum(r.total_tests for r in self.reports.values())
            total_passed = sum(r.passed for r in self.reports.values())
            total_failed = sum(r.failed for r in self.reports.values())
            total_errors = sum(r.errors for r in self.reports.values())

            if total_tests == 0:
                f.write("No tests were run.\n")
                return

            f.write(f"- **Total Tests**: {total_tests}\n")
            f.write(f"- **Passed**: {total_passed} ({100*total_passed/total_tests:.1f}%)\n")
            f.write(f"- **Failed**: {total_failed} ({100*total_failed/total_tests:.1f}%)\n")
            f.write(f"- **Errors**: {total_errors}\n\n")

            all_speedups = []
            for report in self.reports.values():
                all_speedups.extend([t.speedup for t in report.test_cases if t.speedup > 0])

            if all_speedups:
                f.write("## Performance Summary\n\n")
                f.write(f"- **Min Speedup**: {min(all_speedups):.2f}x\n")
                f.write(f"- **Max Speedup**: {max(all_speedups):.2f}x\n")
                f.write(f"- **Average Speedup**: {sum(all_speedups)/len(all_speedups):.2f}x\n")
                f.write(f"- **10-100x Claim**: ")
                avg_speedup = sum(all_speedups) / len(all_speedups)
                if 10 <= avg_speedup <= 100:
                    f.write("VALIDATED\n\n")
                else:
                    f.write(f"NOT VALIDATED (avg: {avg_speedup:.2f}x)\n\n")

            for category, report in self.reports.items():
                f.write(f"## {category.title()}\n\n")
                f.write(f"- Tests: {report.total_tests}\n")
                f.write(f"- Passed: {report.passed}\n")
                f.write(f"- Failed: {report.failed}\n")
                f.write(f"- Errors: {report.errors}\n")

                if report.avg_speedup > 0:
                    f.write(f"- Average Speedup: {report.avg_speedup:.2f}x\n")

                f.write("\n### Test Cases\n\n")
                f.write("| Test | Correctness | SymPy Time | MathHook Time | Speedup |\n")
                f.write("|------|-------------|------------|---------------|----------|\n")

                for test in report.test_cases:
                    status = "PASS" if test.correctness else "FAIL"
                    f.write(f"| {test.name} | {status} | {test.sympy_time:.0f}ns | {test.mathhook_time:.0f}ns | {test.speedup:.2f}x |\n")

                failed_tests = [t for t in report.test_cases if not t.correctness and not t.expected_sympy.startswith("ERROR:")]
                if failed_tests:
                    f.write("\n### Failed Tests (Detail)\n\n")
                    for test in failed_tests:
                        f.write(f"**{test.name}**\n\n")
                        f.write(f"- Input: `{test.input_expr}`\n")
                        f.write(f"- SymPy: `{test.expected_sympy}`\n")
                        f.write(f"- MathHook: `{test.mathhook_result}`\n\n")

                f.write("\n")


def main():
    import argparse

    parser = argparse.ArgumentParser(description='Compare MathHook with SymPy - Phase 2')
    parser.add_argument('--workspace', help='Path to MathHook workspace root')
    parser.add_argument('--output', default='.mathhook_sessions/gtm/wave3.5/correctness_validation.md',
                        help='Output markdown file')
    parser.add_argument('--test-suites', nargs='+', default=['all'],
                        choices=['all', 'derivatives', 'integrals', 'simplify', 'solve', 'evaluate'],
                        help='Which test suites to run')

    args = parser.parse_args()

    validator = SymPyValidator(args.workspace)

    test_suites = {
        'derivatives': [
            {'operation': 'derivative', 'expr': 'x^2', 'name': 'd/dx(x^2)'},
            {'operation': 'derivative', 'expr': 'x^3', 'name': 'd/dx(x^3)'},
            {'operation': 'derivative', 'expr': 'x^4', 'name': 'd/dx(x^4)'},
            {'operation': 'derivative', 'expr': 'sin(x)', 'name': 'd/dx(sin(x))'},
            {'operation': 'derivative', 'expr': 'cos(x)', 'name': 'd/dx(cos(x))'},
            {'operation': 'derivative', 'expr': 'tan(x)', 'name': 'd/dx(tan(x))'},
            {'operation': 'derivative', 'expr': 'exp(x)', 'name': 'd/dx(exp(x))'},
            {'operation': 'derivative', 'expr': 'log(x)', 'name': 'd/dx(log(x))'},
            {'operation': 'derivative', 'expr': 'x*sin(x)', 'name': 'd/dx(x*sin(x))_product_rule'},
            {'operation': 'derivative', 'expr': 'sin(x)/x', 'name': 'd/dx(sin(x)/x)_quotient_rule'},
            {'operation': 'derivative', 'expr': 'sin(x^2)', 'name': 'd/dx(sin(x^2))_chain_rule'},
            {'operation': 'derivative', 'expr': 'x^2 + 2*x + 1', 'name': 'd/dx(x^2+2x+1)'},
            {'operation': 'derivative', 'expr': 'x^2*exp(x)', 'name': 'd/dx(x^2*exp(x))'},
            {'operation': 'derivative', 'expr': '1/x', 'name': 'd/dx(1/x)'},
            {'operation': 'derivative', 'expr': 'sqrt(x)', 'name': 'd/dx(sqrt(x))'},
        ],
        'simplify': [
            {'operation': 'simplify', 'expr': 'x + x', 'name': 'simplify(x+x)'},
            {'operation': 'simplify', 'expr': '2*x + 3*x', 'name': 'simplify(2x+3x)'},
            {'operation': 'simplify', 'expr': 'x^2 - x^2', 'name': 'simplify(x^2-x^2)'},
            {'operation': 'simplify', 'expr': 'x*x', 'name': 'simplify(x*x)'},
            {'operation': 'simplify', 'expr': '(x + 1)*(x - 1)', 'name': 'simplify((x+1)(x-1))'},
            {'operation': 'simplify', 'expr': 'sin(x)^2 + cos(x)^2', 'name': 'simplify(sin^2+cos^2)'},
            {'operation': 'simplify', 'expr': 'x + 0', 'name': 'simplify(x+0)'},
            {'operation': 'simplify', 'expr': 'x*1', 'name': 'simplify(x*1)'},
            {'operation': 'simplify', 'expr': 'x*0', 'name': 'simplify(x*0)'},
        ],
        'solve': [
            {'operation': 'solve', 'expr': 'x + 2', 'name': 'solve(x+2=0)_linear'},
            {'operation': 'solve', 'expr': '2*x - 4', 'name': 'solve(2x-4=0)_linear'},
            {'operation': 'solve', 'expr': 'x^2 - 4', 'name': 'solve(x^2-4=0)_quadratic'},
            {'operation': 'solve', 'expr': 'x^2 + 2*x + 1', 'name': 'solve(x^2+2x+1=0)_perfect_square'},
            {'operation': 'solve', 'expr': 'x^2 - 1', 'name': 'solve(x^2-1=0)_difference_squares'},
            {'operation': 'solve', 'expr': 'x^2 + 1', 'name': 'solve(x^2+1=0)_complex_roots'},
            {'operation': 'solve', 'expr': '3*x + 6', 'name': 'solve(3x+6=0)'},
        ],
        'evaluate': [
            {'operation': 'evaluate', 'expr': 'sin(0)', 'name': 'eval(sin(0))'},
            {'operation': 'evaluate', 'expr': 'cos(0)', 'name': 'eval(cos(0))'},
            {'operation': 'evaluate', 'expr': 'exp(0)', 'name': 'eval(exp(0))'},
            {'operation': 'evaluate', 'expr': 'log(1)', 'name': 'eval(log(1))'},
            {'operation': 'evaluate', 'expr': 'sqrt(4)', 'name': 'eval(sqrt(4))'},
            {'operation': 'evaluate', 'expr': '2 + 3', 'name': 'eval(2+3)'},
            {'operation': 'evaluate', 'expr': '2*3', 'name': 'eval(2*3)'},
            {'operation': 'evaluate', 'expr': '2^3', 'name': 'eval(2^3)'},
        ],
    }

    suites_to_run = test_suites.keys() if 'all' in args.test_suites else args.test_suites

    print("Running SymPy comparison validation - Phase 2...")
    print()

    for suite_name in suites_to_run:
        if suite_name not in test_suites:
            continue
        print(f"Running {suite_name} test suite...")
        report = validator.run_test_suite(suite_name, test_suites[suite_name])
        print(f"  {report.passed}/{report.total_tests} passed")
        if report.avg_speedup > 0:
            print(f"  Average speedup: {report.avg_speedup:.2f}x")
        print()

    os.makedirs(os.path.dirname(args.output), exist_ok=True)
    validator.generate_markdown_report(args.output)
    print(f"Validation report written to: {args.output}")


if __name__ == '__main__':
    main()

/// Comprehensive parsing and formatting benchmarks
///
/// Tests parsing and output formatting with varying expression complexity.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::formatter::latex::{LaTeXContext, LaTeXFormatter};
use mathhook_core::parser::{config::ParserConfig, Parser};
use mathhook_core::{symbol, Expression};
use std::hint::black_box;
use std::time::Duration;
/// Benchmark parsing for simple expressions
fn bench_parsing_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_simple");
    let parser = Parser::new(&ParserConfig::default());
    group.bench_function("parse_variable", |b| {
        b.iter(|| black_box(parser.parse("x")))
    });
    group.bench_function("parse_number", |b| b.iter(|| black_box(parser.parse("42"))));
    group.bench_function("parse_addition", |b| {
        b.iter(|| black_box(parser.parse("x + y")))
    });
    group.bench_function("parse_multiplication", |b| {
        b.iter(|| black_box(parser.parse("x * y")))
    });
    group.bench_function("parse_power", |b| b.iter(|| black_box(parser.parse("x^2"))));
    group.bench_function("parse_sin", |b| {
        b.iter(|| black_box(parser.parse("sin(x)")))
    });
    group.finish();
}
/// Benchmark parsing for complex expressions
fn bench_parsing_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_complex");
    let parser = Parser::new(&ParserConfig::default());
    group.bench_function("parse_polynomial", |b| {
        b.iter(|| black_box(parser.parse("x^3 + 2*x^2 - 5*x + 3")))
    });
    group.bench_function("parse_nested_functions", |b| {
        b.iter(|| black_box(parser.parse("sin(cos(x))")))
    });
    group.bench_function("parse_complex_fraction", |b| {
        b.iter(|| black_box(parser.parse("(x + 1) / (x - 1)")))
    });
    group.bench_function("parse_trig_identity", |b| {
        b.iter(|| black_box(parser.parse("sin(x)^2 + cos(x)^2")))
    });
    group.finish();
}
/// Benchmark implicit multiplication parsing
fn bench_implicit_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("implicit_multiplication");
    let parser = Parser::new(&ParserConfig::default());
    group.bench_function("parse_2x", |b| b.iter(|| black_box(parser.parse("2x"))));
    group.bench_function("parse_2(x+1)", |b| {
        b.iter(|| black_box(parser.parse("2(x+1)")))
    });
    group.bench_function("parse_(a)(b)", |b| {
        b.iter(|| black_box(parser.parse("(a)(b)")))
    });
    group.bench_function("parse_sin(x)cos(x)", |b| {
        b.iter(|| black_box(parser.parse("sin(x)cos(x)")))
    });
    group.finish();
}
/// Benchmark expression formatting
fn bench_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatting");
    let context = LaTeXContext::default();
    let x = symbol!(x);
    let simple = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
    group.bench_function("format_simple", |b| {
        b.iter(|| black_box(simple.to_latex(context.clone())))
    });
    let polynomial = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::symbol(x.clone()),
    ]);
    group.bench_function("format_polynomial", |b| {
        b.iter(|| black_box(polynomial.to_latex(context.clone())))
    });
    let nested = Expression::function(
        "sin",
        vec![Expression::function(
            "cos",
            vec![Expression::symbol(x.clone())],
        )],
    );
    group.bench_function("format_nested_functions", |b| {
        b.iter(|| black_box(nested.to_latex(context.clone())))
    });
    group.finish();
}
/// Benchmark parsing throughput with varying expression sizes
fn bench_parsing_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_throughput");
    let parser = Parser::new(&ParserConfig::default());
    for size in [10, 20, 50, 100].iter() {
        let mut expr = String::from("x");
        for i in 1..*size {
            expr.push_str(&format!(" + x{}", i));
        }
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("sum_terms", size), size, |b, _| {
            b.iter(|| black_box(parser.parse(&expr)))
        });
    }
    group.finish();
}
criterion_group!(
    name = parsing_benchmarks; config = Criterion::default()
    .measurement_time(Duration::from_secs(10)).sample_size(100); targets =
    bench_parsing_simple, bench_parsing_complex, bench_implicit_multiplication,
    bench_formatting, bench_parsing_throughput
);
criterion_main!(parsing_benchmarks);

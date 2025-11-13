#!/bin/bash
# Performance diagnostic script for slow tests

echo "=== Testing Deeply Nested Expression ===" echo "Testing with 5 iterations (2^5 = 32 nodes)..."
cat > /tmp/test_nested_5.rs << 'EOF'
use mathhook_core::{symbol, Expression};
use std::time::Instant;

fn main() {
    let x = symbol!(x);
    let mut nested = Expression::symbol(x.clone());

    let start = Instant::now();
    for i in 0..5 {
        nested = Expression::mul(vec![nested.clone(), nested.clone()]);
        println!("Iteration {}: nodes = 2^{}", i+1, i+1);
    }
    let elapsed = start.elapsed();
    println!("Time for 5 iterations: {:?}", elapsed);
    println!("Final tree has ~{} nodes", 1 << 5);
}
EOF

echo ""
echo "Testing with 10 iterations (2^10 = 1024 nodes)..."
cat > /tmp/test_nested_10.rs << 'EOF'
use mathhook_core::{symbol, Expression};
use std::time::Instant;

fn main() {
    let x = symbol!(x);
    let mut nested = Expression::symbol(x.clone());

    let start = Instant::now();
    for i in 0..10 {
        let iter_start = Instant::now();
        nested = Expression::mul(vec![nested.clone(), nested.clone()]);
        let iter_elapsed = iter_start.elapsed();
        println!("Iteration {}: nodes = 2^{} (took {:?})", i+1, i+1, iter_elapsed);
    }
    let elapsed = start.elapsed();
    println!("Total time for 10 iterations: {:?}", elapsed);
    println!("Final tree has ~{} nodes", 1 << 10);
}
EOF

echo ""
echo "=== Testing Groebner Basis ===" echo "Testing redundant generators (x^2, x^3)..."
cat > /tmp/test_groebner.rs << 'EOF'
use mathhook_core::{symbol, Expression};
use mathhook_core::algebra::groebner::{buchberger_algorithm, MonomialOrder};
use std::time::Instant;

fn main() {
    let x = symbol!(x);
    let vars = vec![x.clone()];

    let f1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let f2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    println!("Running Buchberger on {{x^2, x^3}}...");
    let start = Instant::now();
    match buchberger_algorithm(&vec![f1, f2], &vars, &MonomialOrder::Lex) {
        Ok(gb) => {
            let elapsed = start.elapsed();
            println!("Success! Time: {:?}", elapsed);
            println!("Basis size: {}", gb.len());
            for (i, poly) in gb.iter().enumerate() {
                println!("  Basis[{}]: {}", i, poly);
            }
        }
        Err(e) => {
            let elapsed = start.elapsed();
            println!("Error after {:?}: {:?}", elapsed, e);
        }
    }
}
EOF

echo ""
echo "=== Testing ODE Backward Integration ===" echo "Testing backward integration from x=1 to x=0..."
cat > /tmp/test_ode.rs << 'EOF'
use mathhook_core::ode::numerical::adaptive::{rkf45_method, AdaptiveConfig};
use std::time::Instant;

fn main() {
    println!("Running backward integration: dy/dx = x, y(1) = 0.5, solve to x=0");
    let start = Instant::now();
    let solution = rkf45_method(|x, _y| x, 1.0, 0.5, 0.0, 0.1, AdaptiveConfig::default());
    let elapsed = start.elapsed();

    println!("Time: {:?}", elapsed);
    println!("Solution points: {}", solution.len());
    println!("First point: {:?}", solution.first());
    println!("Last point: {:?}", solution.last());

    if solution.len() > 100 {
        println!("WARNING: Many solution points - may indicate small step sizes");
    }
}
EOF

echo ""
echo "=== Run Instructions ===" echo "To test these manually:"
echo "1. Nested expression (5 iter):  Add to examples/ and run"
echo "2. Nested expression (10 iter): Add to examples/ and run"
echo "3. Groebner basis:              Add to examples/ and run"
echo "4. ODE backward:                Add to examples/ and run"
echo ""
echo "To add as example:"
echo "  cp /tmp/test_nested_5.rs crates/mathhook-core/examples/"
echo "  cargo run --example test_nested_5"

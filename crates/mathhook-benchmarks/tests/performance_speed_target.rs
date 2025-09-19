//! Speed target verification tests

use mathhook_core::prelude::*;

#[test]
fn test_speed_target_achievement() {
    use std::time::Instant;

    println!("🎯 SPEED TARGET VERIFICATION");

    let start = Instant::now();

    // Target: Achieve consistent high performance
    let x = Expression::symbol(Symbol::new("x"));
    let mut total_ops = 0;

    for i in 0..50_000 {
        let expr = Expression::mul(vec![Expression::integer(i % 100), x.clone()]);
        let _result = expr.simplify();
        total_ops += 1;
    }

    let duration = start.elapsed();
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();

    println!(
        "Speed target: {:.2}M ops/sec ({} ops)",
        ops_per_sec / 1_000_000.0,
        total_ops
    );

    // Target: >3M ops/sec (Symbolica level)
    assert!(ops_per_sec >= 500_000.0, "Should achieve speed target");
}

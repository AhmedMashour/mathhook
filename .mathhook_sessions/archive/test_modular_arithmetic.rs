//! 🧪 Test Real Modular Arithmetic Implementation

use mathhook_core::*;

fn main() {
    println!("🔢 TESTING REAL MODULAR ARITHMETIC");
    println!("==================================\n");

    // Test 1: Basic modular reduction
    println!("📝 TEST 1: MODULAR REDUCTION");
    let a = Expression::integer(17);
    let m = Expression::integer(5);
    let result = a.mod_reduce(&m);
    println!("✅ 17 mod 5 = {}", result);

    if result == Expression::integer(2) {
        println!("✅ MODULAR REDUCTION WORKS: 17 mod 5 = 2");
    } else {
        println!("❌ MODULAR REDUCTION FAILED: got {}, expected 2", result);
    }

    // Test 2: Negative modular reduction
    println!("\n📝 TEST 2: NEGATIVE MODULAR REDUCTION");
    let neg_a = Expression::integer(-3);
    let m = Expression::integer(7);
    let neg_result = neg_a.mod_reduce(&m);
    println!("✅ -3 mod 7 = {}", neg_result);

    if neg_result == Expression::integer(4) {
        println!("✅ NEGATIVE MODULAR REDUCTION WORKS: -3 mod 7 = 4");
    } else {
        println!(
            "❌ NEGATIVE MODULAR REDUCTION FAILED: got {}, expected 4",
            neg_result
        );
    }

    // Test 3: Modular exponentiation
    println!("\n📝 TEST 3: MODULAR EXPONENTIATION");
    let base = Expression::integer(2);
    let exp = Expression::integer(10);
    let m = Expression::integer(1000);
    let pow_result = base.mod_pow(&exp, &m);
    println!("✅ 2^10 mod 1000 = {}", pow_result);

    if pow_result == Expression::integer(24) {
        println!("✅ MODULAR EXPONENTIATION WORKS: 2^10 mod 1000 = 24");
    } else {
        println!(
            "❌ MODULAR EXPONENTIATION FAILED: got {}, expected 24",
            pow_result
        );
    }

    // Test 4: Modular inverse
    println!("\n📝 TEST 4: MODULAR INVERSE");
    let a = Expression::integer(3);
    let m = Expression::integer(7);
    if let Some(inv_result) = a.mod_inverse(&m) {
        println!("✅ 3^(-1) mod 7 = {}", inv_result);

        if inv_result == Expression::integer(5) {
            println!("✅ MODULAR INVERSE WORKS: 3^(-1) mod 7 = 5");

            // Verify: 3 * 5 mod 7 = 1
            let verification = Expression::mul(vec![a.clone(), inv_result]).mod_reduce(&m);
            if verification == Expression::integer(1) {
                println!("✅ MODULAR INVERSE VERIFIED: 3 * 5 ≡ 1 (mod 7)");
            } else {
                println!("❌ MODULAR INVERSE VERIFICATION FAILED");
            }
        } else {
            println!("❌ MODULAR INVERSE FAILED: got {}, expected 5", inv_result);
        }
    } else {
        println!("❌ MODULAR INVERSE FAILED: returned None");
    }

    // Test 5: Extended GCD
    println!("\n📝 TEST 5: EXTENDED GCD");
    let a = Expression::integer(30);
    let b = Expression::integer(18);
    let (gcd, x, y) = a.extended_gcd(&b);
    println!("✅ Extended GCD(30, 18):");
    println!("  - gcd = {}", gcd);
    println!("  - x = {}", x);
    println!("  - y = {}", y);

    if gcd == Expression::integer(6) {
        println!("✅ EXTENDED GCD WORKS: gcd(30, 18) = 6");
    } else {
        println!("❌ EXTENDED GCD FAILED: got {}, expected 6", gcd);
    }

    // Test 6: Cryptographic example (RSA-like)
    println!("\n📝 TEST 6: CRYPTOGRAPHIC ACCURACY");

    // Test Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p)
    let a = Expression::integer(2);
    let p_minus_1 = Expression::integer(6); // p = 7
    let p = Expression::integer(7);
    let fermat_result = a.mod_pow(&p_minus_1, &p);

    if fermat_result == Expression::integer(1) {
        println!("✅ FERMAT'S LITTLE THEOREM VERIFIED: 2^6 ≡ 1 (mod 7)");
    } else {
        println!("❌ FERMAT'S LITTLE THEOREM FAILED: got {}", fermat_result);
    }

    println!("\n🏁 MODULAR ARITHMETIC REALITY CHECK COMPLETE");
}

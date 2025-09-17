//! Educational Explanations for Zippel's Algorithm
//!
//! Functions providing step-by-step educational explanations for the
//! various components of Zippel's modular GCD algorithm.

/// Explain the content extraction step
pub fn explain_content_extraction() -> &'static str {
    "Content extraction separates the integer factor from a polynomial:\n\
     \n\
     For f(x) = 6x² + 12x + 18:\n\
     • content(f) = gcd(6, 12, 18) = 6\n\
     • primitive part pp(f) = x² + 2x + 3\n\
     • f = 6 · pp(f)\n\
     \n\
     This simplifies GCD computation:\n\
     gcd(f, g) = gcd(content(f), content(g)) · gcd(pp(f), pp(g))"
}

/// Explain the CRT reconstruction step
pub fn explain_crt_reconstruction() -> &'static str {
    "Chinese Remainder Theorem reconstructs integer coefficients:\n\
     \n\
     1. Compute GCD mod p₁ → h₁(x) with coefficients in Z_{p₁}\n\
     2. Compute GCD mod p₂ → h₂(x) with coefficients in Z_{p₂}\n\
     3. CRT: Find h(x) with coefficients in Z_{p₁·p₂}\n\
     4. Repeat until coefficients stabilize\n\
     \n\
     The magic: If h is the true GCD, its coefficients will converge\n\
     after enough primes, regardless of which primes we choose."
}

/// Explain trial division verification
pub fn explain_trial_division() -> &'static str {
    "Trial division verifies the GCD candidate is correct:\n\
     \n\
     For candidate h to be gcd(f, g):\n\
     1. h must divide f exactly (no remainder)\n\
     2. h must divide g exactly (no remainder)\n\
     \n\
     This catches errors from:\n\
     • Unlucky primes giving wrong degree\n\
     • CRT reconstruction errors\n\
     • Numerical instabilities\n\
     \n\
     If verification fails, we need more primes or a different approach."
}

/// Explain sparse polynomial optimization
pub fn explain_sparse_optimization() -> &'static str {
    "Sparse polynomials have many zero coefficients:\n\
     \n\
     Dense: 5x⁴ + 3x³ + 2x² + x + 1  (5 non-zero terms out of 5)\n\
     Sparse: 5x¹⁰⁰ + 3x⁵⁰ + 1       (3 non-zero terms out of 101)\n\
     \n\
     For sparse polynomials:\n\
     • Skip zero coefficients in multiplication\n\
     • Use evaluation-interpolation instead of dense algorithms\n\
     • Zippel's algorithm is particularly efficient for sparse GCDs"
}

/// Explain multivariate evaluation-interpolation
pub fn explain_multivariate_evaluation() -> &'static str {
    "Multivariate GCD uses evaluation-interpolation:\n\
     \n\
     For f(x,y), g(x,y) in Z[x,y]:\n\
     1. Evaluate at y = α₁, α₂, ... to get univariate polynomials\n\
     2. Compute univariate GCDs: h(x, αᵢ) = gcd(f(x, αᵢ), g(x, αᵢ))\n\
     3. Interpolate to recover h(x, y)\n\
     \n\
     Zippel's insight: For sparse GCDs, many interpolation points\n\
     give zero, so we can detect sparsity and skip those terms."
}

/// Get complete algorithm overview
pub fn algorithm_overview() -> &'static str {
    "Zippel's Modular GCD Algorithm:\n\
     \n\
     INPUT: f(x), g(x) ∈ Z[x]\n\
     OUTPUT: gcd(f, g)\n\
     \n\
     1. CONTENT EXTRACTION\n\
        c_f = content(f), c_g = content(g)\n\
        f̃ = f/c_f (primitive part)\n\
        g̃ = g/c_g (primitive part)\n\
     \n\
     2. MODULAR GCD COMPUTATION\n\
        For primes p₁, p₂, ...:\n\
          hᵢ = gcd(f̃ mod pᵢ, g̃ mod pᵢ) in Z_{pᵢ}[x]\n\
     \n\
     3. CRT RECONSTRUCTION\n\
        Use Chinese Remainder Theorem to lift\n\
        coefficients to Z from Z_{p₁·p₂·...}\n\
     \n\
     4. TRIAL DIVISION VERIFICATION\n\
        Check: h | f and h | g exactly?\n\
        If yes → done\n\
        If no → get more primes\n\
     \n\
     OUTPUT: gcd(c_f, c_g) · h"
}

/// Explain algorithm selection criteria for polynomial GCD
///
/// Provides educational explanation of when to use each GCD algorithm
/// based on polynomial characteristics.
pub fn explain_algorithm_selection() -> &'static str {
    "Algorithm Selection for Polynomial GCD:\n\
     \n\
     DECISION TREE:\n\
     \n\
     1. Are both inputs integers?\n\
        YES -> Use Integer Euclidean Algorithm\n\
               Complexity: O(log(min(a,b)))\n\
     \n\
     2. Is either input zero or one?\n\
        ZERO: gcd(0, b) = |b|, gcd(a, 0) = |a|\n\
        ONE:  gcd(1, b) = 1, gcd(a, 1) = 1\n\
     \n\
     3. Is polynomial univariate with small degree (< 10)?\n\
        YES -> Use Classical Euclidean Algorithm\n\
               Complexity: O(n^2) where n = max(deg f, deg g)\n\
               Best for: Dense polynomials, small coefficients\n\
     \n\
     4. Is polynomial sparse (density < 0.3)?\n\
        YES -> Use Zippel Modular GCD\n\
               Complexity: O(n log n * log(coeff_bound))\n\
               Best for: Many zero coefficients, large degrees\n\
     \n\
     5. Does polynomial have large integer coefficients?\n\
        YES -> Use Zippel Modular GCD\n\
               Avoids coefficient explosion in Euclidean\n\
     \n\
     6. Is polynomial multivariate?\n\
        YES -> Use Zippel Evaluation-Interpolation\n\
               Evaluates at points, computes univariate GCDs,\n\
               then interpolates the result\n\
     \n\
     ALGORITHM COMPARISON:\n\
     \n\
     | Algorithm          | Best For              | Complexity    |\n\
     |--------------------|----------------------|---------------|\n\
     | Euclidean          | Small, dense polys   | O(n^2)        |\n\
     | Zippel Modular     | Large, sparse polys  | O(n log n)    |\n\
     | Multivariate Zippel| Multiple variables   | O(n^v log n)  |\n\
     \n\
     where n = degree, v = number of variables"
}

/// Generate step-by-step description for a specific GCD iteration
///
/// # Arguments
/// * `iteration` - Current iteration number (1-based)
/// * `prime` - The prime used in this iteration
/// * `gcd_degree` - Degree of GCD computed mod p
/// * `converged` - Whether coefficients have stabilized
pub fn explain_iteration_step(
    iteration: usize,
    prime: u64,
    gcd_degree: usize,
    converged: bool,
) -> String {
    format!(
        "Iteration {}:\n\
         - Prime p = {}\n\
         - Computed gcd(f mod p, g mod p) in Z_p[x]\n\
         - GCD degree mod p: {}\n\
         - CRT reconstruction: combining with previous primes\n\
         - Coefficients {}",
        iteration,
        prime,
        gcd_degree,
        if converged {
            "CONVERGED - ready for trial division"
        } else {
            "not yet stabilized - need more primes"
        }
    )
}

/// Explain why a particular algorithm was chosen
///
/// # Arguments
/// * `is_univariate` - Whether polynomial is univariate
/// * `max_degree` - Maximum degree of input polynomials
/// * `is_sparse` - Whether polynomial is sparse (density < 0.3)
/// * `large_coeffs` - Whether coefficients exceed 64-bit range
pub fn explain_selection_rationale(
    is_univariate: bool,
    max_degree: i64,
    is_sparse: bool,
    large_coeffs: bool,
) -> String {
    let algorithm = if !is_univariate {
        "Zippel Multivariate (evaluation-interpolation)"
    } else if is_sparse || large_coeffs || max_degree >= 10 {
        "Zippel Modular GCD"
    } else {
        "Classical Euclidean Algorithm"
    };

    let reason = if !is_univariate {
        "multivariate polynomial requires evaluation-interpolation approach"
    } else if is_sparse {
        "sparse polynomial benefits from modular approach (skips zero terms)"
    } else if large_coeffs {
        "large coefficients would cause explosion in classical Euclidean"
    } else if max_degree >= 10 {
        "high degree makes modular approach more efficient"
    } else {
        "small, dense polynomial is efficient with classical approach"
    };

    format!(
        "Algorithm Selection:\n\
         \n\
         Selected: {}\n\
         \n\
         Rationale:\n\
         - Univariate: {}\n\
         - Max degree: {}\n\
         - Sparse: {}\n\
         - Large coefficients: {}\n\
         \n\
         Decision: {}",
        algorithm,
        if is_univariate { "yes" } else { "no" },
        max_degree,
        if is_sparse { "yes" } else { "no" },
        if large_coeffs { "yes" } else { "no" },
        reason
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_educational_content() {
        assert!(!explain_content_extraction().is_empty());
        assert!(!explain_crt_reconstruction().is_empty());
        assert!(!explain_trial_division().is_empty());
        assert!(!explain_sparse_optimization().is_empty());
        assert!(!explain_multivariate_evaluation().is_empty());
        assert!(!algorithm_overview().is_empty());
    }

    #[test]
    fn test_algorithm_selection_explanation() {
        let explanation = explain_algorithm_selection();
        assert!(explanation.contains("DECISION TREE"));
        assert!(explanation.contains("Euclidean"));
        assert!(explanation.contains("Zippel"));
    }

    #[test]
    fn test_iteration_step_explanation() {
        let step = explain_iteration_step(1, 2147483647, 3, false);
        assert!(step.contains("Iteration 1"));
        assert!(step.contains("2147483647"));
        assert!(step.contains("not yet stabilized"));

        let converged = explain_iteration_step(5, 2147483629, 2, true);
        assert!(converged.contains("CONVERGED"));
    }

    #[test]
    fn test_selection_rationale() {
        let univariate_sparse = explain_selection_rationale(true, 50, true, false);
        assert!(univariate_sparse.contains("Zippel Modular GCD"));
        assert!(univariate_sparse.contains("sparse"));

        let multivariate = explain_selection_rationale(false, 10, false, false);
        assert!(multivariate.contains("Multivariate"));

        let small_dense = explain_selection_rationale(true, 5, false, false);
        assert!(small_dense.contains("Classical Euclidean"));
    }
}

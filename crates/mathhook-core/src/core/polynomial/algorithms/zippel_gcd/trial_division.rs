//! Trial Division Verification
//!
//! Functions for verifying GCD candidates through exact polynomial division.

/// Verification result for trial division
#[derive(Debug, Clone, PartialEq)]
pub enum TrialDivisionResult {
    /// Division successful, exact quotient obtained
    Success { quotient: Vec<i64> },
    /// Division failed, non-zero remainder
    Failure { remainder_degree: usize },
}

/// Perform trial division to verify GCD candidate
///
/// Given polynomials f, g and candidate GCD h, verifies that h | f and h | g
/// by performing exact polynomial division.
///
/// # Mathematical Background
///
/// For h to be the true GCD:
/// 1. h must divide f exactly (remainder = 0)
/// 2. h must divide g exactly (remainder = 0)
///
/// This catches errors from:
/// - Unlucky primes that give wrong degree
/// - CRT reconstruction errors
/// - Numerical instabilities
///
/// # Arguments
///
/// * `dividend` - Polynomial coefficients being divided
/// * `divisor` - Proposed GCD coefficients
///
/// # Returns
///
/// `TrialDivisionResult` indicating success or failure
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
///     trial_divide, TrialDivisionResult
/// };
///
/// // (x² - 1) / (x - 1) = (x + 1), exact
/// let dividend = vec![-1, 0, 1];  // -1 + 0x + x²
/// let divisor = vec![-1, 1];      // -1 + x
/// match trial_divide(&dividend, &divisor) {
///     TrialDivisionResult::Success { quotient } => {
///         assert_eq!(quotient, vec![1, 1]);  // 1 + x
///     }
///     _ => panic!("Expected success"),
/// }
/// ```
pub fn trial_divide(dividend: &[i64], divisor: &[i64]) -> TrialDivisionResult {
    if divisor.is_empty() || divisor.iter().all(|&c| c == 0) {
        return TrialDivisionResult::Failure {
            remainder_degree: 0,
        };
    }

    let divisor_deg = divisor.len() - 1;
    let dividend_deg = dividend.len() - 1;

    if divisor_deg > dividend_deg {
        return TrialDivisionResult::Failure {
            remainder_degree: dividend_deg,
        };
    }

    let lc_divisor = divisor[divisor_deg];
    if lc_divisor == 0 {
        return TrialDivisionResult::Failure {
            remainder_degree: 0,
        };
    }

    let mut remainder = dividend.to_vec();
    let mut quotient = vec![0i64; dividend_deg - divisor_deg + 1];

    for i in (0..=dividend_deg - divisor_deg).rev() {
        let idx = i + divisor_deg;
        if idx >= remainder.len() {
            continue;
        }

        let rem_coeff = remainder[idx];
        if rem_coeff == 0 {
            continue;
        }

        if rem_coeff % lc_divisor != 0 {
            return TrialDivisionResult::Failure {
                remainder_degree: idx,
            };
        }

        let q = rem_coeff / lc_divisor;
        quotient[i] = q;

        for (j, &d) in divisor.iter().enumerate() {
            remainder[i + j] -= q * d;
        }
    }

    if remainder.iter().all(|&c| c == 0) {
        while quotient.len() > 1 && quotient.last() == Some(&0) {
            quotient.pop();
        }
        TrialDivisionResult::Success { quotient }
    } else {
        let rem_deg = remainder.iter().rposition(|&c| c != 0).unwrap_or(0);
        TrialDivisionResult::Failure {
            remainder_degree: rem_deg,
        }
    }
}

/// Verify that a GCD candidate divides both polynomials exactly
///
/// # Arguments
///
/// * `f_coeffs` - First polynomial coefficients
/// * `g_coeffs` - Second polynomial coefficients
/// * `h_coeffs` - Proposed GCD coefficients
///
/// # Returns
///
/// True if h divides both f and g exactly
pub fn verify_gcd_candidate(f_coeffs: &[i64], g_coeffs: &[i64], h_coeffs: &[i64]) -> bool {
    matches!(
        trial_divide(f_coeffs, h_coeffs),
        TrialDivisionResult::Success { .. }
    ) && matches!(
        trial_divide(g_coeffs, h_coeffs),
        TrialDivisionResult::Success { .. }
    )
}

/// Compute cofactor after GCD division
pub fn compute_cofactor(dividend: &[i64], divisor: &[i64]) -> Option<Vec<i64>> {
    match trial_divide(dividend, divisor) {
        TrialDivisionResult::Success { quotient } => Some(quotient),
        TrialDivisionResult::Failure { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_divide_exact() {
        let dividend = vec![-1, 0, 1];
        let divisor = vec![-1, 1];
        match trial_divide(&dividend, &divisor) {
            TrialDivisionResult::Success { quotient } => {
                assert_eq!(quotient, vec![1, 1]);
            }
            _ => panic!("Expected exact division"),
        }
    }

    #[test]
    fn test_trial_divide_not_exact() {
        let dividend = vec![1, 2, 3];
        let divisor = vec![1, 1];
        match trial_divide(&dividend, &divisor) {
            TrialDivisionResult::Failure { .. } => {}
            _ => panic!("Expected failure"),
        }
    }

    #[test]
    fn test_trial_divide_by_constant() {
        let dividend = vec![6, 12, 18];
        let divisor = vec![3];
        match trial_divide(&dividend, &divisor) {
            TrialDivisionResult::Success { quotient } => {
                assert_eq!(quotient, vec![2, 4, 6]);
            }
            _ => panic!("Expected exact division by constant"),
        }
    }

    #[test]
    fn test_verify_gcd_candidate() {
        let f = vec![-1, 0, 1];
        let g = vec![-1, 1];
        let h = vec![-1, 1];
        assert!(verify_gcd_candidate(&f, &g, &h));

        let h_wrong = vec![1, 1];
        assert!(!verify_gcd_candidate(&f, &g, &h_wrong));
    }
}

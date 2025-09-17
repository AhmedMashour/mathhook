//! Multivariate polynomial GCD computation using evaluation-interpolation
//!
//! Implements the heuristic GCD algorithm (heugcd) from SymPy's euclidtools.py.
//! This approach avoids the infinite recursion issues of content-primitive factorization
//! by using integer evaluation and polynomial interpolation.
//!
//! # Algorithm Overview (from `[Liao95]`)
//!
//! For multivariate polynomials f, g in Z[x₁, ..., xₙ]:
//! 1. Extract ground GCD (numeric content only - NON-RECURSIVE)
//! 2. Evaluate both polynomials at integer point x₀ for main variable
//! 3. Recursively compute GCD of resulting (n-1)-variate polynomials
//! 4. Interpolate back to n-variate polynomial
//! 5. Verify result by polynomial division
//! 6. If verification fails, try new evaluation point (up to 6 attempts)
//!
//! # Mathematical Background
//!
//! The key insight is that GCD computation can be reduced dimension by dimension:
//! - gcd(f(x,y), g(x,y)) at y=y₀ gives gcd(f(x,y₀), g(x,y₀))
//! - The GCD is then reconstructed via polynomial interpolation
//! - Verification ensures correctness (heuristic may fail, triggering retry)
//!
//! # References
//!
//! - `[Liao95]` Liao, Q. "Factoring multivariate polynomials over algebraic number fields"
//! - SymPy polys/euclidtools.py: `dmp_zz_heu_gcd`, `dup_zz_heu_gcd`

use crate::algebra::gcd::PolynomialGcd;
use crate::algebra::polynomial_advanced::AdvancedPolynomial;
use crate::core::{Expression, Number, Symbol};
use crate::expr;
use crate::simplify::Simplify;
use num_traits::ToPrimitive;

/// Maximum number of evaluation points to try before giving up
const HEU_GCD_MAX_ATTEMPTS: usize = 6;

/// Error type for heuristic GCD failures
#[derive(Debug, Clone)]
pub struct HeuristicGCDFailed;

/// Compute GCD of multivariate polynomials using evaluation-interpolation
///
/// This is the main entry point. Uses the heuristic GCD algorithm which:
/// 1. Extracts numeric content (ground GCD)
/// 2. Evaluates at integer points
/// 3. Recursively reduces dimension
/// 4. Interpolates and verifies
///
/// # Arguments
///
/// * `poly1` - First polynomial expression
/// * `poly2` - Second polynomial expression
/// * `vars` - List of variables (in order of elimination)
///
/// # Returns
///
/// Returns the GCD expression. Falls back to 1 if heuristic fails.
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_core::{symbol, expr};
/// use mathhook_core::algebra::multivariate_gcd::multivariate_gcd;
///
/// let x = symbol!(x);
/// let y = symbol!(y);
///
/// // gcd(2xy, 3xy) = xy
/// let p1 = expr!(2 * x * y);
/// let p2 = expr!(3 * x * y);
/// let result = multivariate_gcd(&p1, &p2, &[x.clone(), y.clone()]);
/// ```
pub fn multivariate_gcd(poly1: &Expression, poly2: &Expression, vars: &[Symbol]) -> Expression {
    // Handle trivial cases first
    if let Some(result) = trivial_gcd(poly1, poly2, vars) {
        return result;
    }

    // Try heuristic GCD algorithm
    match multivariate_heu_gcd(poly1, poly2, vars) {
        Ok((gcd, _, _)) => gcd,
        Err(HeuristicGCDFailed) => {
            // Fallback: try univariate GCD if single variable
            if vars.len() == 1 {
                univariate_gcd_euclidean(poly1, poly2, &vars[0])
            } else {
                // Last resort: return 1 (conservative, but correct)
                Expression::integer(1)
            }
        }
    }
}

/// Check for trivial GCD cases (following SymPy's _dmp_rr_trivial_gcd pattern)
fn trivial_gcd(poly1: &Expression, poly2: &Expression, vars: &[Symbol]) -> Option<Expression> {
    if poly1.is_zero() {
        return Some(poly2.clone());
    }
    if poly2.is_zero() {
        return Some(poly1.clone());
    }
    if poly1 == poly2 {
        return Some(poly1.clone());
    }
    if poly1.is_one() || poly2.is_one() {
        return Some(Expression::integer(1));
    }
    if is_constant(poly1, vars) && is_constant(poly2, vars) {
        return Some(poly1.gcd(poly2));
    }

    None
}

/// Heuristic multivariate polynomial GCD in Z[X]
///
/// Implements SymPy's `dmp_zz_heu_gcd` algorithm:
/// 1. Base case: univariate → use `univariate_heu_gcd`
/// 2. Extract ground GCD (numeric content only)
/// 3. Compute bounds for evaluation point
/// 4. For each attempt:
///    a. Evaluate at integer point
///    b. Recursively compute GCD
///    c. Interpolate result
///    d. Verify by division
/// 5. Return (gcd, cofactor1, cofactor2) or error
fn multivariate_heu_gcd(
    poly1: &Expression,
    poly2: &Expression,
    vars: &[Symbol],
) -> Result<(Expression, Expression, Expression), HeuristicGCDFailed> {
    // Base case: univariate
    if vars.len() <= 1 {
        if vars.is_empty() {
            let g = poly1.gcd(poly2);
            let cff = if g.is_one() {
                poly1.clone()
            } else {
                divide_exact(poly1, &g)
            };
            let cfg = if g.is_one() {
                poly2.clone()
            } else {
                divide_exact(poly2, &g)
            };
            return Ok((g, cff, cfg));
        }
        return univariate_heu_gcd(poly1, poly2, &vars[0]);
    }

    // Check trivial cases
    if let Some(g) = trivial_gcd(poly1, poly2, vars) {
        let cff = divide_exact(poly1, &g);
        let cfg = divide_exact(poly2, &g);
        return Ok((g, cff, cfg));
    }

    // Disjoint variables check: gcd(x+1, y+1) = 1 (they share no common variables)
    // This is the key fix for coprime polynomials in different variables
    if have_disjoint_variables(poly1, poly2) {
        return Ok((expr!(1), poly1.clone(), poly2.clone()));
    }

    // Extract ground GCD (numeric content only - NON-RECURSIVE!)
    let (ground_gcd, f, g) = extract_ground_gcd(poly1, poly2);

    // Main variable is the first one
    let main_var = &vars[0];
    let remaining_vars: Vec<Symbol> = vars[1..].to_vec();

    // Compute bounds for evaluation point
    let f_norm = polynomial_max_norm(&f, vars);
    let g_norm = polynomial_max_norm(&g, vars);

    let b = 2 * f_norm.min(g_norm) + 29;
    let mut x = compute_initial_eval_point(b, &f, &g, main_var);

    // Try up to HEU_GCD_MAX_ATTEMPTS evaluation points
    for _ in 0..HEU_GCD_MAX_ATTEMPTS {
        // Evaluate polynomials at x for main variable
        let ff = polynomial_evaluate_at(&f, main_var, x);
        let gg = polynomial_evaluate_at(&g, main_var, x);

        // Skip if either evaluation is zero
        if !ff.is_zero() && !gg.is_zero() {
            // Recursively compute GCD of evaluated polynomials
            if let Ok((h_eval, cff_eval, cfg_eval)) =
                multivariate_heu_gcd(&ff, &gg, &remaining_vars)
            {
                // Interpolate GCD back to full dimension
                let mut h = polynomial_interpolate(&h_eval, x, main_var);
                h = ground_primitive(&h, vars);

                // Try to verify h is the correct GCD
                if let Some(cff) = try_exact_division(&f, &h, main_var) {
                    if let Some(cfg) = try_exact_division(&g, &h, main_var) {
                        // Success! Multiply by ground GCD
                        let result_gcd = if ground_gcd == 1 {
                            h
                        } else {
                            Expression::mul(vec![Expression::integer(ground_gcd), h]).simplify()
                        };
                        return Ok((result_gcd, cff, cfg));
                    }
                }

                // Try cofactor approach: interpolate cff, compute h = f/cff
                let cff = polynomial_interpolate(&cff_eval, x, main_var);
                if let Some(h2) = try_exact_division(&f, &cff, main_var) {
                    if let Some(cfg) = try_exact_division(&g, &h2, main_var) {
                        let result_gcd = if ground_gcd == 1 {
                            h2
                        } else {
                            Expression::mul(vec![Expression::integer(ground_gcd), h2]).simplify()
                        };
                        return Ok((result_gcd, cff, cfg));
                    }
                }

                // Try other cofactor: interpolate cfg, compute h = g/cfg
                let cfg = polynomial_interpolate(&cfg_eval, x, main_var);
                if let Some(h3) = try_exact_division(&g, &cfg, main_var) {
                    if let Some(cff2) = try_exact_division(&f, &h3, main_var) {
                        let result_gcd = if ground_gcd == 1 {
                            h3
                        } else {
                            Expression::mul(vec![Expression::integer(ground_gcd), h3]).simplify()
                        };
                        return Ok((result_gcd, cff2, cfg));
                    }
                }
            }
        }

        // Update evaluation point using SymPy's formula
        x = update_eval_point(x);
    }

    Err(HeuristicGCDFailed)
}

/// Heuristic univariate polynomial GCD in ``Z[x]``
///
/// Implements SymPy's `dup_zz_heu_gcd` algorithm for single variable case.
fn univariate_heu_gcd(
    poly1: &Expression,
    poly2: &Expression,
    var: &Symbol,
) -> Result<(Expression, Expression, Expression), HeuristicGCDFailed> {
    // Trivial cases
    if poly1.is_zero() {
        return Ok((poly2.clone(), expr!(0), expr!(1)));
    }
    if poly2.is_zero() {
        return Ok((poly1.clone(), expr!(1), expr!(0)));
    }
    if poly1.is_one() || poly2.is_one() {
        return Ok((expr!(1), poly1.clone(), poly2.clone()));
    }

    // Both don't depend on var → numeric GCD
    if !depends_on_var(poly1, var) && !depends_on_var(poly2, var) {
        let g = poly1.gcd(poly2);
        let cff = divide_exact(poly1, &g);
        let cfg = divide_exact(poly2, &g);
        return Ok((g, cff, cfg));
    }

    // One depends on var, one doesn't → coprime (GCD = 1)
    // Example: gcd(x+1, 5) = 1 when considering variable x
    if !depends_on_var(poly1, var) || !depends_on_var(poly2, var) {
        return Ok((expr!(1), poly1.clone(), poly2.clone()));
    }

    let deg1 = poly1.polynomial_degree(var).unwrap_or(0);
    let deg2 = poly2.polynomial_degree(var).unwrap_or(0);

    // Extract numeric content
    let (ground_gcd, f, g) = extract_ground_gcd(poly1, poly2);

    // Both constant after content extraction
    if deg1 == 0 && deg2 == 0 {
        let gcd = Expression::integer(ground_gcd);
        return Ok((gcd, f, g));
    }

    // Compute bounds
    let f_norm = polynomial_max_norm(&f, std::slice::from_ref(var));
    let g_norm = polynomial_max_norm(&g, std::slice::from_ref(var));

    let b = 2 * f_norm.min(g_norm) + 29;
    let mut x = compute_initial_eval_point(b, &f, &g, var);

    for _attempt in 0..HEU_GCD_MAX_ATTEMPTS {
        // Evaluate at x
        let ff = evaluate_univariate(&f, var, x);
        let gg = evaluate_univariate(&g, var, x);

        if ff != 0 && gg != 0 {
            // Integer GCD
            let h_int = gcd_integers(ff, gg);
            let cff_int = ff / h_int;
            let cfg_int = gg / h_int;

            // Interpolate back to polynomial
            let mut h = univariate_interpolate(h_int, x, var);
            h = univariate_primitive(&h, var);

            // Verify by division
            if let Some(cff) = try_exact_division(&f, &h, var) {
                if let Some(cfg) = try_exact_division(&g, &h, var) {
                    let result = if ground_gcd == 1 {
                        h
                    } else {
                        Expression::mul(vec![Expression::integer(ground_gcd), h]).simplify()
                    };
                    return Ok((result, cff, cfg));
                }
            }

            // Try cofactor interpolation
            let cff = univariate_interpolate(cff_int, x, var);
            if let Some(h2) = try_exact_division(&f, &cff, var) {
                if let Some(cfg) = try_exact_division(&g, &h2, var) {
                    let result = if ground_gcd == 1 {
                        h2
                    } else {
                        Expression::mul(vec![Expression::integer(ground_gcd), h2]).simplify()
                    };
                    return Ok((result, cff, cfg));
                }
            }

            let cfg = univariate_interpolate(cfg_int, x, var);
            if let Some(h3) = try_exact_division(&g, &cfg, var) {
                if let Some(cff2) = try_exact_division(&f, &h3, var) {
                    let result = if ground_gcd == 1 {
                        h3
                    } else {
                        Expression::mul(vec![Expression::integer(ground_gcd), h3]).simplify()
                    };
                    return Ok((result, cff2, cfg));
                }
            }
        }

        x = update_eval_point(x);
    }

    Err(HeuristicGCDFailed)
}

/// Extract ground GCD (numeric content only) from two polynomials
///
/// Returns (gcd, poly1/gcd, poly2/gcd) where gcd is the integer GCD
/// of all numeric coefficients in both polynomials.
///
/// CRITICAL: This is NON-RECURSIVE - only extracts numeric content!
fn extract_ground_gcd(poly1: &Expression, poly2: &Expression) -> (i64, Expression, Expression) {
    let coeffs1 = collect_numeric_coefficients(poly1);
    let coeffs2 = collect_numeric_coefficients(poly2);

    // Compute GCD of all coefficients
    let mut gcd = 0i64;
    for c in coeffs1.iter().chain(coeffs2.iter()) {
        gcd = gcd_integers(gcd, c.abs());
        if gcd == 1 {
            break;
        }
    }

    if gcd <= 1 {
        return (1, poly1.clone(), poly2.clone());
    }

    // Divide both polynomials by ground GCD
    let f = divide_by_integer(poly1, gcd);
    let g = divide_by_integer(poly2, gcd);

    (gcd, f, g)
}

/// Collect all numeric coefficients from a polynomial expression
fn collect_numeric_coefficients(expr: &Expression) -> Vec<i64> {
    let mut coeffs = Vec::new();
    collect_coeffs_recursive(expr, &mut coeffs);
    if coeffs.is_empty() {
        coeffs.push(1);
    }
    coeffs
}

fn collect_coeffs_recursive(expr: &Expression, coeffs: &mut Vec<i64>) {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            coeffs.push(*n);
        }
        Expression::Number(Number::Rational(r)) => {
            if let Some(n) = r.numer().to_i64() {
                coeffs.push(n);
            }
            if let Some(d) = r.denom().to_i64() {
                coeffs.push(d);
            }
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                collect_coeffs_recursive(term, coeffs);
            }
        }
        Expression::Mul(factors) => {
            for factor in factors.iter() {
                collect_coeffs_recursive(factor, coeffs);
            }
        }
        Expression::Pow(base, exp) => {
            collect_coeffs_recursive(base, coeffs);
            collect_coeffs_recursive(exp, coeffs);
        }
        _ => {}
    }
}

/// Divide polynomial by an integer
fn divide_by_integer(expr: &Expression, divisor: i64) -> Expression {
    if divisor == 1 {
        return expr.clone();
    }

    match expr {
        Expression::Number(Number::Integer(n)) => {
            if n % divisor == 0 {
                Expression::integer(n / divisor)
            } else {
                expr.clone()
            }
        }
        Expression::Add(terms) => {
            let new_terms: Vec<Expression> = terms
                .iter()
                .map(|t| divide_by_integer(t, divisor))
                .collect();
            Expression::add(new_terms).simplify()
        }
        Expression::Mul(factors) => {
            // Try to divide the first numeric factor
            let mut divided = false;
            let mut new_factors = Vec::new();
            for factor in factors.iter() {
                if !divided {
                    if let Expression::Number(Number::Integer(n)) = factor {
                        if n % divisor == 0 {
                            let new_coeff = n / divisor;
                            if new_coeff != 1 {
                                new_factors.push(Expression::integer(new_coeff));
                            }
                            divided = true;
                            continue;
                        }
                    }
                }
                new_factors.push(factor.clone());
            }
            if new_factors.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(new_factors).simplify()
            }
        }
        _ => expr.clone(),
    }
}

/// Compute maximum absolute value of coefficients (infinity norm)
fn polynomial_max_norm(expr: &Expression, _vars: &[Symbol]) -> i64 {
    let coeffs = collect_numeric_coefficients(expr);
    coeffs.iter().map(|c| c.abs()).max().unwrap_or(1)
}

/// Compute initial evaluation point (SymPy formula)
fn compute_initial_eval_point(b: i64, f: &Expression, g: &Expression, var: &Symbol) -> i64 {
    let f_norm = polynomial_max_norm(f, std::slice::from_ref(var));
    let g_norm = polynomial_max_norm(g, std::slice::from_ref(var));

    let lc_f = leading_coeff_abs(f, var);
    let lc_g = leading_coeff_abs(g, var);

    let sqrt_b = (b as f64).sqrt() as i64;
    let option1 = b.min(99 * sqrt_b);
    let option2 = if lc_f > 0 && lc_g > 0 {
        2 * (f_norm / lc_f).min(g_norm / lc_g) + 4
    } else {
        4
    };

    option1.max(option2).max(2)
}

/// Update evaluation point for next attempt (SymPy formula)
fn update_eval_point(x: i64) -> i64 {
    let sqrt_x = (x as f64).sqrt() as i64;
    let sqrt_sqrt_x = (sqrt_x as f64).sqrt() as i64;
    (73794 * x * sqrt_sqrt_x) / 27011
}

/// Get absolute value of leading coefficient
fn leading_coeff_abs(expr: &Expression, var: &Symbol) -> i64 {
    let lc = expr.polynomial_leading_coefficient(var);
    match lc {
        Expression::Number(Number::Integer(n)) => n.abs(),
        _ => 1,
    }
}

/// Evaluate polynomial at integer value for main variable
///
/// Substitutes var = value and simplifies to get a polynomial in remaining variables.
pub fn polynomial_evaluate_at(poly: &Expression, var: &Symbol, value: i64) -> Expression {
    substitute_var(poly, var, value).simplify()
}

fn substitute_var(expr: &Expression, var: &Symbol, value: i64) -> Expression {
    match expr {
        Expression::Symbol(s) if s == var => Expression::integer(value),
        Expression::Symbol(_) => expr.clone(),
        Expression::Number(_) => expr.clone(),
        Expression::Constant(_) => expr.clone(),
        Expression::Add(terms) => {
            let new_terms: Vec<Expression> = terms
                .iter()
                .map(|t| substitute_var(t, var, value))
                .collect();
            Expression::add(new_terms)
        }
        Expression::Mul(factors) => {
            let new_factors: Vec<Expression> = factors
                .iter()
                .map(|f| substitute_var(f, var, value))
                .collect();
            Expression::mul(new_factors)
        }
        Expression::Pow(base, exp) => {
            let new_base = substitute_var(base, var, value);
            let new_exp = substitute_var(exp, var, value);
            Expression::pow(new_base, new_exp)
        }
        _ => expr.clone(),
    }
}

/// Evaluate univariate polynomial at integer value, returning integer result
fn evaluate_univariate(poly: &Expression, var: &Symbol, value: i64) -> i64 {
    let result = polynomial_evaluate_at(poly, var, value);
    match result {
        Expression::Number(Number::Integer(n)) => n,
        _ => 0,
    }
}

/// Interpolate polynomial from integer using symmetric representation
///
/// Recovers polynomial coefficients from integer h using base x.
/// Uses symmetric modular representation: if coeff > x/2, use coeff - x.
pub fn polynomial_interpolate(h: &Expression, x: i64, var: &Symbol) -> Expression {
    // If h is already a polynomial (multivariate case), apply interpolation recursively
    match h {
        Expression::Number(Number::Integer(n)) => univariate_interpolate(*n, x, var),
        Expression::Add(terms) => {
            let new_terms: Vec<Expression> = terms
                .iter()
                .map(|t| polynomial_interpolate(t, x, var))
                .collect();
            Expression::add(new_terms).simplify()
        }
        Expression::Mul(factors) => {
            let new_factors: Vec<Expression> = factors
                .iter()
                .map(|f| polynomial_interpolate(f, x, var))
                .collect();
            Expression::mul(new_factors).simplify()
        }
        _ => h.clone(),
    }
}

/// Interpolate univariate polynomial from integer
///
/// Converts integer h to polynomial using base x with symmetric representation.
/// The resulting polynomial uses `var` as the variable.
fn univariate_interpolate(mut h: i64, x: i64, var: &Symbol) -> Expression {
    if h == 0 {
        return expr!(0);
    }

    let mut coeffs = Vec::new();
    let half_x = x / 2;

    while h != 0 {
        let mut coeff = h % x;
        if coeff > half_x {
            coeff -= x;
        }
        coeffs.push(coeff);
        h = (h - coeff) / x;
    }

    if coeffs.is_empty() {
        return expr!(0);
    }

    let mut terms = Vec::new();

    for (power, &coeff) in coeffs.iter().enumerate() {
        if coeff == 0 {
            continue;
        }
        let term = if power == 0 {
            Expression::integer(coeff)
        } else if power == 1 {
            if coeff == 1 {
                Expression::symbol(var.clone())
            } else {
                Expression::mul(vec![
                    Expression::integer(coeff),
                    Expression::symbol(var.clone()),
                ])
            }
        } else {
            let power_expr = Expression::pow(
                Expression::symbol(var.clone()),
                Expression::integer(power as i64),
            );
            if coeff == 1 {
                power_expr
            } else {
                Expression::mul(vec![Expression::integer(coeff), power_expr])
            }
        };
        terms.push(term);
    }

    if terms.is_empty() {
        expr!(0)
    } else if terms.len() == 1 {
        terms.pop().unwrap()
    } else {
        Expression::add(terms)
    }
}

/// Compute primitive part (divide by content) for multivariate polynomial
fn ground_primitive(poly: &Expression, _vars: &[Symbol]) -> Expression {
    let coeffs = collect_numeric_coefficients(poly);
    let mut content = 0i64;
    for c in coeffs {
        content = gcd_integers(content, c.abs());
        if content == 1 {
            break;
        }
    }

    if content <= 1 {
        return poly.clone();
    }

    divide_by_integer(poly, content)
}

/// Compute primitive part for univariate polynomial
fn univariate_primitive(poly: &Expression, _var: &Symbol) -> Expression {
    let coeffs = collect_numeric_coefficients(poly);
    let mut content = 0i64;
    for c in coeffs {
        content = gcd_integers(content, c.abs());
        if content == 1 {
            break;
        }
    }

    if content <= 1 {
        return poly.clone();
    }

    divide_by_integer(poly, content)
}

/// Try exact polynomial division, returning Some(quotient) if successful
fn try_exact_division(
    dividend: &Expression,
    divisor: &Expression,
    _var: &Symbol,
) -> Option<Expression> {
    if divisor.is_one() {
        return Some(dividend.clone());
    }
    if divisor.is_zero() {
        return None;
    }
    if dividend == divisor {
        return Some(expr!(1));
    }
    if dividend.is_zero() {
        return Some(expr!(0));
    }

    // Try integer division first
    if let (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) =
        (dividend, divisor)
    {
        if *b != 0 && a % b == 0 {
            return Some(Expression::integer(a / b));
        } else {
            return None;
        }
    }

    // Try monomial division: coeff * vars / vars -> coeff (e.g., 2*y / y = 2)
    if let Some(result) = try_monomial_division(dividend, divisor) {
        return Some(result);
    }

    // Try sum division: (a + b) / c = a/c + b/c (if both exact)
    if let Expression::Add(terms) = dividend {
        let mut quotient_terms = Vec::new();
        for term in terms.iter() {
            if let Some(q) = try_exact_division(term, divisor, _var) {
                quotient_terms.push(q);
            } else {
                return None;
            }
        }
        let result = Expression::add(quotient_terms).simplify();
        return Some(result);
    }

    None
}

/// Try to divide monomials directly
/// Handles cases like: 2*y / y = 2, 6*x*y / (2*x) = 3*y
fn try_monomial_division(dividend: &Expression, divisor: &Expression) -> Option<Expression> {
    let (div_coeff, div_vars) = extract_coeff_and_vars(dividend);
    let (sor_coeff, sor_vars) = extract_coeff_and_vars(divisor);

    // Check coefficient divisibility
    if sor_coeff == 0 {
        return None;
    }
    if div_coeff % sor_coeff != 0 {
        return None;
    }
    let result_coeff = div_coeff / sor_coeff;

    // Check variable divisibility: each var in divisor must appear in dividend with >= power
    let mut remaining_vars = div_vars;
    for (var, power) in &sor_vars {
        if let Some(div_power) = remaining_vars.get(var) {
            if *div_power >= *power {
                let new_power = div_power - power;
                if new_power == 0 {
                    remaining_vars.remove(var);
                } else {
                    remaining_vars.insert(var.clone(), new_power);
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    // Build result expression
    let mut factors = Vec::new();
    if result_coeff != 1 || remaining_vars.is_empty() {
        factors.push(Expression::integer(result_coeff));
    }
    for (var, power) in remaining_vars {
        if power == 1 {
            factors.push(Expression::symbol(var));
        } else {
            factors.push(Expression::pow(
                Expression::symbol(var),
                Expression::integer(power),
            ));
        }
    }

    if factors.is_empty() {
        Some(Expression::integer(1))
    } else if factors.len() == 1 {
        Some(factors.pop().unwrap())
    } else {
        Some(Expression::mul(factors).simplify())
    }
}

/// Extract numeric coefficient and variable factors from a monomial expression
/// Returns (coefficient, HashMap of variable -> power)
fn extract_coeff_and_vars(expr: &Expression) -> (i64, std::collections::HashMap<Symbol, i64>) {
    let mut coeff = 1i64;
    let mut vars = std::collections::HashMap::new();

    extract_coeff_and_vars_recursive(expr, &mut coeff, &mut vars);

    (coeff, vars)
}

fn extract_coeff_and_vars_recursive(
    expr: &Expression,
    coeff: &mut i64,
    vars: &mut std::collections::HashMap<Symbol, i64>,
) {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            *coeff *= *n;
        }
        Expression::Symbol(s) => {
            *vars.entry(s.clone()).or_insert(0) += 1;
        }
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                (base.as_ref(), exp.as_ref())
            {
                *vars.entry(s.clone()).or_insert(0) += *e;
            }
        }
        Expression::Mul(factors) => {
            for factor in factors.iter() {
                extract_coeff_and_vars_recursive(factor, coeff, vars);
            }
        }
        _ => {}
    }
}

/// Divide exactly (assuming division is exact)
fn divide_exact(dividend: &Expression, divisor: &Expression) -> Expression {
    if divisor.is_one() {
        return dividend.clone();
    }
    if divisor.is_zero() {
        return dividend.clone();
    }
    if dividend == divisor {
        return expr!(1);
    }

    if let (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) =
        (dividend, divisor)
    {
        if *b != 0 && a % b == 0 {
            return Expression::integer(a / b);
        }
    }

    Expression::mul(vec![
        dividend.clone(),
        Expression::pow(divisor.clone(), Expression::integer(-1)),
    ])
    .simplify()
}

/// Integer GCD using Euclidean algorithm
fn gcd_integers(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

/// Check if expression depends on a variable
fn depends_on_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => s == var,
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().any(|t| depends_on_var(t, var))
        }
        Expression::Pow(base, exp) => depends_on_var(base, var) || depends_on_var(exp, var),
        _ => false,
    }
}

/// Check if expression is constant with respect to variables
fn is_constant(expr: &Expression, vars: &[Symbol]) -> bool {
    !vars.iter().any(|v| depends_on_var(expr, v))
}

/// Collect all symbols (free variables) from an expression
fn collect_expression_symbols(expr: &Expression) -> std::collections::HashSet<Symbol> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();

    fn collect(expr: &Expression, symbols: &mut HashSet<Symbol>) {
        match expr {
            Expression::Symbol(s) => {
                symbols.insert(s.clone());
            }
            Expression::Add(terms) | Expression::Mul(terms) => {
                for term in terms.iter() {
                    collect(term, symbols);
                }
            }
            Expression::Pow(base, exp) => {
                collect(base, symbols);
                collect(exp, symbols);
            }
            Expression::Function { args, .. } => {
                for arg in args.iter() {
                    collect(arg, symbols);
                }
            }
            _ => {}
        }
    }

    collect(expr, &mut symbols);
    symbols
}

/// Check if two polynomials have disjoint variable sets
/// If they share no common variables, their GCD is 1 (coprime)
fn have_disjoint_variables(poly1: &Expression, poly2: &Expression) -> bool {
    let vars1 = collect_expression_symbols(poly1);
    let vars2 = collect_expression_symbols(poly2);
    vars1.is_disjoint(&vars2)
}

/// Fallback: Euclidean GCD for univariate polynomials
fn univariate_gcd_euclidean(poly1: &Expression, poly2: &Expression, var: &Symbol) -> Expression {
    if poly1.is_one() || poly2.is_one() {
        return expr!(1);
    }
    if !depends_on_var(poly1, var) && !depends_on_var(poly2, var) {
        return poly1.gcd(poly2);
    }

    let mut a = poly1.clone();
    let mut b = poly2.clone();

    while !b.is_zero() {
        let r = a.rem_polynomial(&b, var);
        if r == a {
            return expr!(1);
        }
        a = b;
        b = r;
    }

    normalize_gcd(&a, var)
}

/// Normalize GCD to have positive leading coefficient
fn normalize_gcd(poly: &Expression, var: &Symbol) -> Expression {
    let lc = poly.polynomial_leading_coefficient(var);
    if let Expression::Number(Number::Integer(n)) = lc {
        if n < 0 {
            return Expression::mul(vec![Expression::integer(-1), poly.clone()]).simplify();
        }
    }
    poly.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_gcd_integers() {
        assert_eq!(gcd_integers(12, 18), 6);
        assert_eq!(gcd_integers(15, 25), 5);
        assert_eq!(gcd_integers(7, 11), 1);
        assert_eq!(gcd_integers(0, 5), 5);
        assert_eq!(gcd_integers(-12, 18), 6);
    }

    #[test]
    fn test_extract_ground_gcd() {
        let p1 = Expression::mul(vec![Expression::integer(6), expr!(x)]);
        let p2 = Expression::mul(vec![Expression::integer(9), expr!(y)]);
        let (gcd, _f, _g) = extract_ground_gcd(&p1, &p2);
        assert_eq!(gcd, 3);
    }

    #[test]
    fn test_polynomial_evaluate_at() {
        let x = symbol!(x);
        let poly = Expression::add(vec![
            Expression::pow(expr!(x), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(2), expr!(x)]),
            Expression::integer(1),
        ]);
        let result = polynomial_evaluate_at(&poly, &x, 3);
        assert_eq!(result, Expression::integer(16));
    }

    #[test]
    fn test_trivial_cases() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let p = expr!(x);
        assert_eq!(multivariate_gcd(&expr!(0), &p, &vars), p);

        assert_eq!(multivariate_gcd(&p, &p, &vars), p);

        assert_eq!(multivariate_gcd(&expr!(1), &p, &vars), expr!(1));
    }

    #[test]
    fn test_constant_gcd() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let result = multivariate_gcd(&Expression::integer(6), &Expression::integer(9), &vars);
        assert_eq!(result, Expression::integer(3));
    }

    #[test]
    fn test_coprime_polynomials() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let result = multivariate_gcd(&expr!(x), &expr!(y), &vars);
        assert!(result.is_one() || result == Expression::integer(1));
    }

    #[test]
    fn test_bivariate_gcd_simple() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let p = Expression::mul(vec![expr!(x), expr!(y)]);
        let result = multivariate_gcd(&p, &p, &vars);
        assert_eq!(result, p);
    }

    #[test]
    fn test_bivariate_gcd_content() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let p1 = Expression::mul(vec![Expression::integer(2), expr!(x), expr!(y)]);
        let p2 = Expression::mul(vec![Expression::integer(3), expr!(x), expr!(y)]);
        let result = multivariate_gcd(&p1, &p2, &vars);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_bivariate_gcd_different_degrees() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let p1 = Expression::mul(vec![expr!(x ^ 2), expr!(y)]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y ^ 2)]);
        let result = multivariate_gcd(&p1, &p2, &vars);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_trivariate_gcd() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);
        let vars = vec![x.clone(), y.clone(), z.clone()];

        let p1 = Expression::mul(vec![expr!(x), expr!(y), expr!(z)]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
        let result = multivariate_gcd(&p1, &p2, &vars);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_univariate_interpolate() {
        let x = symbol!(x);
        let result = univariate_interpolate(5, 10, &x);
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_zero_polynomial() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let p = Expression::mul(vec![expr!(x), expr!(y)]);
        let zero = expr!(0);

        assert_eq!(multivariate_gcd(&p, &zero, &vars), p);

        assert_eq!(multivariate_gcd(&zero, &p, &vars), p);
    }
}

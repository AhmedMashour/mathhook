use super::IntPoly;
use crate::core::{Expression, Number, Symbol};

impl IntPoly {
    /// Try to convert an Expression to IntPoly
    ///
    /// Returns Some(IntPoly) if the expression is a univariate polynomial
    /// with integer coefficients. Returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::core::polynomial::poly::IntPoly;
    ///
    /// let x = symbol!(x);
    /// let poly = expr!((x^2) + (2*x) + 1);
    /// if let Some(int_poly) = IntPoly::try_from_expression(&poly, &x) {
    ///     let deriv = int_poly.derivative();
    /// }
    /// ```
    pub fn try_from_expression(expr: &Expression, var: &Symbol) -> Option<Self> {
        let mut coeffs = std::collections::HashMap::new();

        if !extract_int_coefficients(expr, var, &mut coeffs) {
            return None;
        }

        if coeffs.is_empty() {
            return Some(Self::zero());
        }

        let max_deg = *coeffs.keys().max()?;
        if max_deg > 1000 {
            return None;
        }

        let mut coeff_vec = vec![0i64; max_deg as usize + 1];
        for (deg, coeff) in coeffs {
            if deg >= 0 {
                coeff_vec[deg as usize] = coeff;
            }
        }

        Some(Self::from_coeffs(coeff_vec))
    }

    /// Convert IntPoly back to Expression
    ///
    /// # Example
    /// ```rust
    /// use mathhook_core::symbol;
    /// use mathhook_core::core::polynomial::poly::IntPoly;
    ///
    /// let x = symbol!(x);
    /// let p = IntPoly::from_coeffs(vec![1, 2, 3]);
    /// let expr = p.to_expression(&x);
    /// ```
    pub fn to_expression(&self, var: &Symbol) -> Expression {
        if self.is_zero() {
            return Expression::integer(0);
        }

        let mut terms = Vec::new();

        for (i, &c) in self.coeffs.iter().enumerate() {
            if c == 0 {
                continue;
            }

            let term = match i {
                0 => Expression::integer(c),
                1 if c == 1 => Expression::symbol(var.clone()),
                1 if c == -1 => Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::symbol(var.clone()),
                ]),
                1 => Expression::mul(vec![
                    Expression::integer(c),
                    Expression::symbol(var.clone()),
                ]),
                _ if c == 1 => Expression::pow(
                    Expression::symbol(var.clone()),
                    Expression::integer(i as i64),
                ),
                _ if c == -1 => Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::pow(
                        Expression::symbol(var.clone()),
                        Expression::integer(i as i64),
                    ),
                ]),
                _ => Expression::mul(vec![
                    Expression::integer(c),
                    Expression::pow(
                        Expression::symbol(var.clone()),
                        Expression::integer(i as i64),
                    ),
                ]),
            };

            terms.push(term);
        }

        if terms.is_empty() {
            Expression::integer(0)
        } else if terms.len() == 1 {
            terms.pop().unwrap()
        } else {
            Expression::add(terms)
        }
    }

    /// Check if an Expression can be converted to IntPoly
    ///
    /// This is a fast check that doesn't allocate.
    #[inline]
    pub fn can_convert(expr: &Expression, var: &Symbol) -> bool {
        is_int_polynomial(expr, var)
    }
}

/// Extract integer coefficients from Expression
///
/// Returns false if any non-integer coefficient is found
fn extract_int_coefficients(
    expr: &Expression,
    var: &Symbol,
    coeffs: &mut std::collections::HashMap<i64, i64>,
) -> bool {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            *coeffs.entry(0).or_insert(0) += n;
            true
        }
        Expression::Number(Number::BigInteger(_)) => false,
        Expression::Symbol(s) if s == var => {
            *coeffs.entry(1).or_insert(0) += 1;
            true
        }
        Expression::Symbol(_) => false,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var && *n >= 0 {
                    *coeffs.entry(*n).or_insert(0) += 1;
                    return true;
                }
            }
            false
        }
        Expression::Mul(factors) => {
            let mut coeff = 1i64;
            let mut degree = 0i64;

            for factor in factors.iter() {
                match factor {
                    Expression::Number(Number::Integer(n)) => match coeff.checked_mul(*n) {
                        Some(c) => coeff = c,
                        None => return false,
                    },
                    Expression::Symbol(s) if s == var => {
                        degree += 1;
                    }
                    Expression::Pow(base, exp) => {
                        if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                            (base.as_ref(), exp.as_ref())
                        {
                            if s == var && *n >= 0 {
                                degree += *n;
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            *coeffs.entry(degree).or_insert(0) += coeff;
            true
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                if !extract_int_coefficients(term, var, coeffs) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

/// Check if Expression is a polynomial with integer coefficients
fn is_int_polynomial(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Number(Number::Integer(_)) => true,
        Expression::Number(_) => false,
        Expression::Symbol(s) => s == var,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                s == var && *n >= 0
            } else {
                false
            }
        }
        Expression::Mul(factors) => {
            let mut has_valid_var_term = true;
            for factor in factors.iter() {
                match factor {
                    Expression::Number(Number::Integer(_)) => {}
                    Expression::Symbol(s) if s == var => {}
                    Expression::Pow(base, exp) => {
                        if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                            (base.as_ref(), exp.as_ref())
                        {
                            if s != var || *n < 0 {
                                has_valid_var_term = false;
                            }
                        } else {
                            has_valid_var_term = false;
                        }
                    }
                    _ => {
                        has_valid_var_term = false;
                    }
                }
            }
            has_valid_var_term
        }
        Expression::Add(terms) => terms.iter().all(|t| is_int_polynomial(t, var)),
        _ => false,
    }
}

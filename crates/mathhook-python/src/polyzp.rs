//! Direct Python bindings for PolyZp (polynomial over finite field)
//!
//! This module provides Python access to the fast polynomial operations
//! in MathHook's finite field module, bypassing the Expression tree overhead.
//!
//! Performance comparison:
//! - PyExpression path: ~800µs overhead (tree walking + conversion)
//! - PyPolyZp path: ~1-20µs (direct coefficient array operations)
use mathhook_core::algebra::{PolyZp, Zp};
use pyo3::prelude::*;
/// High-performance polynomial over finite field Z_p[x]
///
/// This class provides direct access to MathHook's optimized polynomial
/// algorithms including NTT-based multiplication and Euclidean GCD.
///
/// Use this class for maximum performance when working with polynomials
/// over finite fields. For symbolic manipulation, use PyExpression instead.
#[pyclass(name = "PolyZp")]
#[derive(Clone)]
pub struct PyPolyZp {
    inner: PolyZp,
}
#[pymethods]
impl PyPolyZp {
    /// Create a polynomial from coefficients and modulus
    ///
    /// Args:
    ///     coeffs: List of integer coefficients [c0, c1, c2, ...] for c0 + c1*x + c2*x^2 + ...
    ///     modulus: Prime modulus p for Z_p[x]
    ///
    /// Example:
    ///     >>> p = PolyZp([1, 2, 1], 7)  # 1 + 2x + x^2 mod 7
    #[new]
    pub fn new(coeffs: Vec<i64>, modulus: u64) -> Self {
        let unsigned_coeffs: Vec<u64> = coeffs
            .into_iter()
            .map(|c| {
                if c < 0 {
                    let c_mod = ((-c) as u64) % modulus;
                    if c_mod == 0 {
                        0
                    } else {
                        modulus - c_mod
                    }
                } else {
                    (c as u64) % modulus
                }
            })
            .collect();
        PyPolyZp {
            inner: PolyZp::from_coeffs(unsigned_coeffs, modulus),
        }
    }
    /// Create zero polynomial
    ///
    /// Args:
    ///     modulus: Prime modulus p
    #[staticmethod]
    pub fn zero(modulus: u64) -> Self {
        PyPolyZp {
            inner: PolyZp::zero(modulus),
        }
    }
    /// Create constant polynomial
    ///
    /// Args:
    ///     c: Constant value
    ///     modulus: Prime modulus p
    #[staticmethod]
    pub fn constant(c: u64, modulus: u64) -> Self {
        PyPolyZp {
            inner: PolyZp::constant(c, modulus),
        }
    }
    /// Create polynomial x
    ///
    /// Args:
    ///     modulus: Prime modulus p
    #[staticmethod]
    pub fn x(modulus: u64) -> Self {
        PyPolyZp {
            inner: PolyZp::x(modulus),
        }
    }
    /// Get the degree of the polynomial (None for zero polynomial)
    #[getter]
    pub fn degree(&self) -> Option<usize> {
        self.inner.degree()
    }
    /// Get the modulus
    #[getter]
    pub fn modulus(&self) -> u64 {
        self.inner.modulus()
    }
    /// Check if polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
    /// Check if polynomial is a constant
    pub fn is_constant(&self) -> bool {
        self.inner.is_constant()
    }
    /// Get coefficients as list
    pub fn coefficients(&self) -> Vec<u64> {
        self.inner.coefficients().to_vec()
    }
    /// Get coefficient at index i (returns 0 if out of bounds)
    pub fn coeff(&self, i: usize) -> u64 {
        self.inner.coeff(i).value()
    }
    /// Add two polynomials
    ///
    /// Returns: self + other
    pub fn add(&self, other: &PyPolyZp) -> Self {
        PyPolyZp {
            inner: self.inner.add(&other.inner),
        }
    }
    /// Subtract two polynomials
    ///
    /// Returns: self - other
    pub fn sub(&self, other: &PyPolyZp) -> Self {
        PyPolyZp {
            inner: self.inner.sub(&other.inner),
        }
    }
    /// Multiply two polynomials (naive O(n^2) algorithm)
    ///
    /// Returns: self * other
    pub fn mul(&self, other: &PyPolyZp) -> Self {
        PyPolyZp {
            inner: self.inner.mul(&other.inner),
        }
    }
    /// Fast multiply using NTT when beneficial (auto-selects algorithm)
    ///
    /// Uses NTT (Number Theoretic Transform) for large polynomials (degree > 64),
    /// falls back to naive multiplication for small polynomials.
    ///
    /// Returns: self * other
    pub fn mul_fast(&self, other: &PyPolyZp) -> Self {
        PyPolyZp {
            inner: self.inner.mul_fast(&other.inner),
        }
    }
    /// Divide with remainder
    ///
    /// Returns: (quotient, remainder) such that self = quotient * divisor + remainder
    pub fn div_rem(&self, divisor: &PyPolyZp) -> PyResult<(PyPolyZp, PyPolyZp)> {
        match self.inner.div_rem(&divisor.inner) {
            Ok((q, r)) => Ok((PyPolyZp { inner: q }, PyPolyZp { inner: r })),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }
    /// Euclidean division quotient
    ///
    /// Returns: self // divisor
    pub fn __floordiv__(&self, divisor: &PyPolyZp) -> PyResult<PyPolyZp> {
        let (q, _) = self.div_rem(divisor)?;
        Ok(q)
    }
    /// Euclidean division remainder
    ///
    /// Returns: self % divisor
    pub fn __mod__(&self, divisor: &PyPolyZp) -> PyResult<PyPolyZp> {
        let (_, r) = self.div_rem(divisor)?;
        Ok(r)
    }
    /// Compute GCD using Euclidean algorithm
    ///
    /// Returns: monic GCD of self and other
    pub fn gcd(&self, other: &PyPolyZp) -> PyResult<PyPolyZp> {
        match self.inner.gcd(&other.inner) {
            Ok(g) => Ok(PyPolyZp { inner: g }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }
    /// Extended Euclidean algorithm
    ///
    /// Returns: (gcd, s, t) such that gcd = s*self + t*other
    pub fn extended_gcd(&self, other: &PyPolyZp) -> PyResult<(PyPolyZp, PyPolyZp, PyPolyZp)> {
        match self.inner.extended_gcd(&other.inner) {
            Ok((g, s, t)) => Ok((
                PyPolyZp { inner: g },
                PyPolyZp { inner: s },
                PyPolyZp { inner: t },
            )),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }
    /// Make polynomial monic (leading coefficient = 1)
    pub fn make_monic(&self) -> PyResult<PyPolyZp> {
        match self.inner.make_monic() {
            Ok(m) => Ok(PyPolyZp { inner: m }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{}", e))),
        }
    }
    /// Evaluate polynomial at a point using Horner's method
    ///
    /// Args:
    ///     x: Point to evaluate at
    ///
    /// Returns: self(x) mod modulus
    pub fn evaluate(&self, x: u64) -> u64 {
        self.inner.evaluate(x).value()
    }
    /// Scale polynomial by a constant
    ///
    /// Args:
    ///     c: Scaling factor
    ///
    /// Returns: c * self
    pub fn scale(&self, c: u64) -> Self {
        PyPolyZp {
            inner: self.inner.scale(Zp::new(c, self.inner.modulus())),
        }
    }
    /// Negate polynomial
    ///
    /// Returns: -self
    pub fn neg(&self) -> Self {
        let m = self.inner.modulus();
        PyPolyZp {
            inner: self.inner.scale(Zp::new(m - 1, m)),
        }
    }
    /// Shift polynomial by n (multiply by x^n)
    ///
    /// Args:
    ///     n: Shift amount
    ///
    /// Returns: self * x^n
    pub fn shift(&self, n: usize) -> Self {
        PyPolyZp {
            inner: self.inner.shift(n),
        }
    }
    /// Get leading coefficient
    pub fn leading_coeff(&self) -> Option<u64> {
        self.inner.leading_coeff().map(|c| c.value())
    }
    fn __add__(&self, other: &PyPolyZp) -> Self {
        self.add(other)
    }
    fn __sub__(&self, other: &PyPolyZp) -> Self {
        self.sub(other)
    }
    fn __mul__(&self, other: &PyPolyZp) -> Self {
        self.mul_fast(other)
    }
    fn __neg__(&self) -> Self {
        self.neg()
    }
    fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
    fn __eq__(&self, other: &PyPolyZp) -> bool {
        self.inner == other.inner
    }
    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.inner.coefficients().hash(&mut hasher);
        self.inner.modulus().hash(&mut hasher);
        hasher.finish()
    }
}
/// Convenience function to create polynomial from Python list
#[pyfunction]
pub fn poly_zp(coeffs: Vec<i64>, modulus: u64) -> PyPolyZp {
    PyPolyZp::new(coeffs, modulus)
}
/// Compute GCD of two polynomials over Z_p
#[pyfunction]
pub fn poly_gcd(a: &PyPolyZp, b: &PyPolyZp) -> PyResult<PyPolyZp> {
    a.gcd(b)
}
/// Fast polynomial multiplication using NTT
#[pyfunction]
pub fn poly_mul_fast(a: &PyPolyZp, b: &PyPolyZp) -> PyPolyZp {
    a.mul_fast(b)
}

//! Expression constructor methods

use super::{
    CalculusData, ComplexData, Expression, IntervalData, PiecewiseData, RelationData, RelationType,
};
use crate::core::{MathConstant, Number, Symbol};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;

impl Expression {
    /// Create a number expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::number(42);
    /// let expr = Expression::number(3.14);
    /// ```
    pub fn number<T: Into<Number>>(value: T) -> Self {
        Self::Number(value.into())
    }

    /// Create an integer expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::integer(42);
    /// ```
    pub fn integer(value: i64) -> Self {
        Self::Number(Number::integer(value))
    }

    /// Create an integer expression from BigInt
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use num_bigint::BigInt;
    ///
    /// let big_val = BigInt::from(42);
    /// let expr = Expression::big_integer(big_val);
    /// ```
    pub fn big_integer(value: BigInt) -> Self {
        if let Some(small_val) = value.to_i64() {
            Self::Number(Number::integer(small_val))
        } else {
            Self::Number(Number::BigInteger(Box::new(value)))
        }
    }

    /// Create a rational number expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::rational(3, 4); // 3/4
    /// let expr = Expression::rational(-1, 2); // -1/2
    /// ```
    pub fn rational(numerator: i64, denominator: i64) -> Self {
        let rational = BigRational::new(BigInt::from(numerator), BigInt::from(denominator));
        Self::Number(Number::rational(rational))
    }

    /// Create a float expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::float(3.14159);
    /// ```
    pub fn float(value: f64) -> Self {
        Self::Number(Number::float(value))
    }

    /// Create a symbol expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::symbol(Symbol::new("x"));
    /// let expr = Expression::symbol("x");
    /// ```
    pub fn symbol<T: Into<Symbol>>(symbol: T) -> Self {
        Self::Symbol(symbol.into())
    }

    /// Create an addition expression in canonical form
    ///
    /// This constructor automatically produces a canonical form expression by:
    /// - Flattening nested additions: `(a + b) + c` → `a + b + c`
    /// - Removing identity elements: `x + 0` → `x`
    /// - Combining like terms: `2x + 3x` → `5x`
    /// - Sorting terms in canonical order: `y + x` → `x + y`
    /// - Evaluating constant subexpressions: `2 + 3` → `5`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Constant folding
    /// let expr = Expression::add(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    /// ]);
    /// assert_eq!(expr, Expression::integer(3));
    ///
    /// // Identity element removal
    /// let x = Expression::symbol("x");
    /// let expr = Expression::add(vec![x.clone(), Expression::integer(0)]);
    /// assert_eq!(expr, x);
    ///
    /// // Commutativity (canonical ordering)
    /// let y = Expression::symbol("y");
    /// let expr1 = Expression::add(vec![x.clone(), y.clone()]);
    /// let expr2 = Expression::add(vec![y.clone(), x.clone()]);
    /// assert_eq!(expr1, expr2); // Both produce x + y in canonical order
    /// ```
    pub fn add(terms: Vec<Expression>) -> Self {
        crate::simplify::arithmetic::simplify_addition(&terms)
    }

    /// Create a multiplication expression in canonical form
    ///
    /// This constructor automatically produces a canonical form expression by:
    /// - Flattening nested multiplications: `(a * b) * c` → `a * b * c`
    /// - Removing identity elements: `x * 1` → `x`
    /// - Handling zero: `x * 0` → `0`
    /// - Sorting factors in canonical order: `y * x` → `x * y`
    /// - Evaluating constant subexpressions: `2 * 3` → `6`
    /// - Converting division to multiplication: `a / b` → `a * b^(-1)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Constant folding
    /// let expr = Expression::mul(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// ]);
    /// assert_eq!(expr, Expression::integer(6));
    ///
    /// // Identity element removal
    /// let x = Expression::symbol("x");
    /// let expr = Expression::mul(vec![x.clone(), Expression::integer(1)]);
    /// assert_eq!(expr, x);
    ///
    /// // Zero handling
    /// let expr = Expression::mul(vec![x.clone(), Expression::integer(0)]);
    /// assert_eq!(expr, Expression::integer(0));
    ///
    /// // Commutativity (canonical ordering)
    /// let y = Expression::symbol("y");
    /// let expr1 = Expression::mul(vec![x.clone(), y.clone()]);
    /// let expr2 = Expression::mul(vec![y.clone(), x.clone()]);
    /// assert_eq!(expr1, expr2); // Both produce x * y in canonical order
    /// ```
    pub fn mul(factors: Vec<Expression>) -> Self {
        crate::simplify::arithmetic::simplify_multiplication(&factors)
    }

    /// Create a power expression in canonical form
    ///
    /// This constructor automatically produces a canonical form expression by:
    /// - Applying power identities: `x^0` → `1`, `x^1` → `x`, `1^x` → `1`
    /// - Evaluating constant powers: `2^3` → `8`
    /// - Converting negative exponents to rationals: `x^(-1)` → `1/x`
    /// - Flattening nested powers: `(x^a)^b` → `x^(a*b)`
    /// - Handling special cases: `0^n` → `0` for positive n
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Power identities
    /// let x = Expression::symbol("x");
    /// let expr = Expression::pow(x.clone(), Expression::integer(1));
    /// assert_eq!(expr, x);
    ///
    /// let expr = Expression::pow(x.clone(), Expression::integer(0));
    /// assert_eq!(expr, Expression::integer(1));
    ///
    /// // Constant evaluation
    /// let expr = Expression::pow(
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// );
    /// assert_eq!(expr, Expression::integer(8));
    ///
    /// // Nested power flattening
    /// let expr = Expression::pow(
    ///     Expression::pow(x.clone(), Expression::integer(2)),
    ///     Expression::integer(3),
    /// );
    /// // Produces x^6 in canonical form
    /// ```
    pub fn pow(base: Expression, exponent: Expression) -> Self {
        crate::simplify::arithmetic::simplify_power(&base, &exponent)
    }

    /// Create a function expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::function("sin", vec![Expression::symbol("x")]);
    /// ```
    pub fn function<S: Into<String>>(name: S, args: Vec<Expression>) -> Self {
        Self::Function {
            name: name.into(),
            args: Box::new(args),
        }
    }

    /// Create a constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MathConstant};
    ///
    /// let expr = Expression::constant(MathConstant::Pi);
    /// ```
    pub fn constant(constant: MathConstant) -> Self {
        Self::Constant(constant)
    }

    /// Create a pi constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let pi = Expression::pi();
    /// ```
    pub fn pi() -> Self {
        Self::Constant(MathConstant::Pi)
    }

    /// Create an e constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let e = Expression::e();
    /// ```
    pub fn e() -> Self {
        Self::Constant(MathConstant::E)
    }

    /// Create an imaginary unit expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let i = Expression::i();
    /// ```
    pub fn i() -> Self {
        Self::Constant(MathConstant::I)
    }

    /// Create an infinity expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let inf = Expression::infinity();
    /// ```
    pub fn infinity() -> Self {
        Self::Constant(MathConstant::Infinity)
    }

    /// Create a negative infinity expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let neg_inf = Expression::negative_infinity();
    /// ```
    pub fn negative_infinity() -> Self {
        Self::Constant(MathConstant::NegativeInfinity)
    }

    /// Create a square root expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let sqrt_2 = Expression::sqrt(Expression::integer(2));
    /// ```
    pub fn sqrt(arg: Expression) -> Self {
        Self::function("sqrt", vec![arg])
    }

    /// Create an undefined expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let undef = Expression::undefined();
    /// ```
    pub fn undefined() -> Self {
        Self::Constant(MathConstant::Undefined)
    }

    /// Create a golden ratio (phi) expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let phi = Expression::golden_ratio();
    /// ```
    pub fn golden_ratio() -> Self {
        Self::Constant(MathConstant::GoldenRatio)
    }

    /// Create an Euler-Mascheroni constant (gamma) expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let gamma = Expression::euler_gamma();
    /// ```
    pub fn euler_gamma() -> Self {
        Self::Constant(MathConstant::EulerGamma)
    }

    /// Create a complex number expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::complex(
    ///     Expression::integer(3),
    ///     Expression::integer(4),
    /// );
    /// ```
    pub fn complex(real: Expression, imag: Expression) -> Self {
        Self::Complex(Box::new(ComplexData { real, imag }))
    }

    /// Create an equation (equality relation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::equation(
    ///     Expression::symbol("x"),
    ///     Expression::integer(5),
    /// );
    /// ```
    pub fn equation(left: Expression, right: Expression) -> Self {
        Self::Relation(Box::new(RelationData {
            left,
            right,
            relation_type: RelationType::Equal,
        }))
    }

    /// Create a derivative expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::derivative(
    ///     Expression::pow(Expression::symbol("x"), Expression::integer(2)),
    ///     Symbol::new("x"),
    ///     1,
    /// );
    /// ```
    pub fn derivative(expression: Expression, variable: Symbol, order: u32) -> Self {
        Self::Calculus(Box::new(CalculusData::Derivative {
            expression,
            variable,
            order,
        }))
    }

    /// Create an integral expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::integral(
    ///     Expression::symbol("x"),
    ///     Symbol::new("x"),
    /// );
    /// ```
    pub fn integral(integrand: Expression, variable: Symbol) -> Self {
        Self::Calculus(Box::new(CalculusData::Integral {
            integrand,
            variable,
            bounds: None,
        }))
    }

    /// Create a definite integral expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::definite_integral(
    ///     Expression::symbol("x"),
    ///     Symbol::new("x"),
    ///     Expression::integer(0),
    ///     Expression::integer(1),
    /// );
    /// ```
    pub fn definite_integral(
        integrand: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(CalculusData::Integral {
            integrand,
            variable,
            bounds: Some((start, end)),
        }))
    }

    /// Create a limit expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::limit(
    ///     Expression::symbol("x"),
    ///     Symbol::new("x"),
    ///     Expression::integer(0),
    /// );
    /// ```
    pub fn limit(expression: Expression, variable: Symbol, point: Expression) -> Self {
        Self::Calculus(Box::new(CalculusData::Limit {
            expression,
            variable,
            point,
            direction: super::LimitDirection::Both,
        }))
    }

    /// Create a sum expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::sum(
    ///     Expression::symbol("i"),
    ///     Symbol::new("i"),
    ///     Expression::integer(1),
    ///     Expression::integer(10),
    /// );
    /// ```
    pub fn sum(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(CalculusData::Sum {
            expression,
            variable,
            start,
            end,
        }))
    }

    /// Create a product expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let expr = Expression::product(
    ///     Expression::symbol("i"),
    ///     Symbol::new("i"),
    ///     Expression::integer(1),
    ///     Expression::integer(10),
    /// );
    /// ```
    pub fn product(
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    ) -> Self {
        Self::Calculus(Box::new(CalculusData::Product {
            expression,
            variable,
            start,
            end,
        }))
    }

    /// Create a set expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let set = Expression::set(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// ]);
    /// ```
    pub fn set(elements: Vec<Expression>) -> Self {
        Self::Set(Box::new(elements))
    }

    /// Create an interval expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let interval = Expression::interval(
    ///     Expression::integer(0),
    ///     Expression::integer(10),
    ///     true,
    ///     false,
    /// );
    /// ```
    pub fn interval(
        start: Expression,
        end: Expression,
        start_inclusive: bool,
        end_inclusive: bool,
    ) -> Self {
        Self::Interval(Box::new(IntervalData {
            start,
            end,
            start_inclusive,
            end_inclusive,
        }))
    }

    /// Create a piecewise function expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let piecewise = Expression::piecewise(
    ///     vec![(Expression::symbol("x"), Expression::integer(1))],
    ///     Some(Expression::integer(0)),
    /// );
    /// ```
    pub fn piecewise(pieces: Vec<(Expression, Expression)>, default: Option<Expression>) -> Self {
        Self::Piecewise(Box::new(PiecewiseData { pieces, default }))
    }

    /// Create a relation expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::core::expression::RelationType;
    ///
    /// let relation = Expression::relation(
    ///     Expression::symbol("x"),
    ///     Expression::integer(5),
    ///     RelationType::Greater,
    /// );
    /// ```
    pub fn relation(left: Expression, right: Expression, relation_type: RelationType) -> Self {
        Self::Relation(Box::new(RelationData {
            left,
            right,
            relation_type,
        }))
    }

    // ========== MATRIX CONSTRUCTORS ==========

    /// Create a matrix expression from rows
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// ```
    pub fn matrix(rows: Vec<Vec<Expression>>) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::dense(rows)))
    }

    /// Create an identity matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let identity = Expression::identity_matrix(3);
    /// assert!(identity.is_identity_matrix());
    /// ```
    pub fn identity_matrix(size: usize) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::identity(size)))
    }

    /// Create a method call expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::symbol("A");
    /// let det_call = Expression::method_call(matrix, "det", vec![]);
    /// let trace_call = Expression::method_call(
    ///     Expression::symbol("B"),
    ///     "trace",
    ///     vec![]
    /// );
    /// ```
    pub fn method_call(
        object: Expression,
        method_name: impl Into<String>,
        args: Vec<Expression>,
    ) -> Self {
        use crate::core::expression::MethodCallData;
        Self::MethodCall(Box::new(MethodCallData {
            object,
            method_name: method_name.into(),
            args,
        }))
    }

    /// Create a zero matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let zero = Expression::zero_matrix(2, 3);
    /// assert!(zero.is_zero_matrix());
    /// ```
    pub fn zero_matrix(rows: usize, cols: usize) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::zero(rows, cols)))
    }

    /// Create a diagonal matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let diag = Expression::diagonal_matrix(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// ```
    pub fn diagonal_matrix(diagonal_elements: Vec<Expression>) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::diagonal(diagonal_elements)))
    }

    /// Create a scalar matrix expression (c*I)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let scalar = Expression::scalar_matrix(3, Expression::integer(5));
    /// // Creates 5*I (5 times the 3x3 identity matrix)
    /// ```
    pub fn scalar_matrix(size: usize, scalar_value: Expression) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::scalar(size, scalar_value)))
    }

    /// Create matrix from nested arrays (convenience method)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix_from_arrays([
    ///     [1, 2, 3],
    ///     [4, 5, 6]
    /// ]);
    /// ```
    pub fn matrix_from_arrays<const R: usize, const C: usize>(arrays: [[i64; C]; R]) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::from_arrays(arrays)))
    }
}

#[cfg(test)]
mod canonical_form_tests {
    use super::*;

    // ========== COMMUTATIVITY TESTS ==========

    #[test]
    fn test_addition_commutativity() {
        // x + y should equal y + x (canonical ordering)
        let x = Expression::symbol("x");
        let y = Expression::symbol("y");

        let expr1 = Expression::add(vec![x.clone(), y.clone()]);
        let expr2 = Expression::add(vec![y.clone(), x.clone()]);

        assert_eq!(expr1, expr2, "Addition should be commutative in canonical form");
    }

    #[test]
    fn test_multiplication_commutativity() {
        // x * y should equal y * x (canonical ordering)
        let x = Expression::symbol("x");
        let y = Expression::symbol("y");

        let expr1 = Expression::mul(vec![x.clone(), y.clone()]);
        let expr2 = Expression::mul(vec![y.clone(), x.clone()]);

        assert_eq!(expr1, expr2, "Multiplication should be commutative in canonical form");
    }

    #[test]
    fn test_multi_term_commutativity() {
        // x + y + z should equal z + y + x (canonical ordering)
        let x = Expression::symbol("x");
        let y = Expression::symbol("y");
        let z = Expression::symbol("z");

        let expr1 = Expression::add(vec![x.clone(), y.clone(), z.clone()]);
        let expr2 = Expression::add(vec![z.clone(), y.clone(), x.clone()]);

        assert_eq!(expr1, expr2, "Multi-term addition should have canonical order");
    }

    // ========== IDENTITY ELEMENT TESTS ==========

    #[test]
    fn test_addition_identity() {
        // x + 0 should equal x
        let x = Expression::symbol("x");
        let expr = Expression::add(vec![x.clone(), Expression::integer(0)]);

        assert_eq!(expr, x, "Adding zero should return the original expression");
    }

    #[test]
    fn test_multiplication_identity() {
        // x * 1 should equal x
        let x = Expression::symbol("x");
        let expr = Expression::mul(vec![x.clone(), Expression::integer(1)]);

        assert_eq!(expr, x, "Multiplying by one should return the original expression");
    }

    #[test]
    fn test_power_identity_exponent_one() {
        // x^1 should equal x
        let x = Expression::symbol("x");
        let expr = Expression::pow(x.clone(), Expression::integer(1));

        assert_eq!(expr, x, "Raising to power 1 should return the original expression");
    }

    #[test]
    fn test_power_identity_exponent_zero() {
        // x^0 should equal 1
        let x = Expression::symbol("x");
        let expr = Expression::pow(x, Expression::integer(0));

        assert_eq!(expr, Expression::integer(1), "Any expression raised to power 0 should equal 1");
    }

    #[test]
    fn test_power_identity_base_one() {
        // 1^x should equal 1
        let x = Expression::symbol("x");
        let expr = Expression::pow(Expression::integer(1), x);

        assert_eq!(expr, Expression::integer(1), "One raised to any power should equal 1");
    }

    #[test]
    fn test_multiplication_zero() {
        // x * 0 should equal 0
        let x = Expression::symbol("x");
        let expr = Expression::mul(vec![x, Expression::integer(0)]);

        assert_eq!(expr, Expression::integer(0), "Multiplying by zero should return zero");
    }

    // ========== ASSOCIATIVITY FLATTENING TESTS ==========

    #[test]
    fn test_addition_associativity_flattening() {
        // (x + y) + z should be flattened to x + y + z
        let x = Expression::symbol("x");
        let y = Expression::symbol("y");
        let z = Expression::symbol("z");

        let inner = Expression::add(vec![x.clone(), y.clone()]);
        let expr = Expression::add(vec![inner, z.clone()]);

        // Should be flattened (no nested Add nodes)
        match &expr {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 3, "Addition should be flattened");
                // All terms should be non-Add expressions
                for term in terms.iter() {
                    assert!(!matches!(term, Expression::Add(_)), "No nested Add nodes");
                }
            }
            _ => panic!("Expected Add expression"),
        }
    }

    #[test]
    fn test_multiplication_associativity_flattening() {
        // (x * y) * z should be flattened to x * y * z
        let x = Expression::symbol("x");
        let y = Expression::symbol("y");
        let z = Expression::symbol("z");

        let inner = Expression::mul(vec![x.clone(), y.clone()]);
        let expr = Expression::mul(vec![inner, z.clone()]);

        // Should be flattened (no nested Mul nodes)
        match &expr {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3, "Multiplication should be flattened");
                // All factors should be non-Mul expressions
                for factor in factors.iter() {
                    assert!(!matches!(factor, Expression::Mul(_)), "No nested Mul nodes");
                }
            }
            _ => panic!("Expected Mul expression"),
        }
    }

    // ========== CONSTANT FOLDING TESTS ==========

    #[test]
    fn test_addition_constant_folding() {
        // 2 + 3 should evaluate to 5
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);

        assert_eq!(expr, Expression::integer(5), "Constant addition should be evaluated");
    }

    #[test]
    fn test_multiplication_constant_folding() {
        // 2 * 3 should evaluate to 6
        let expr = Expression::mul(vec![Expression::integer(2), Expression::integer(3)]);

        assert_eq!(expr, Expression::integer(6), "Constant multiplication should be evaluated");
    }

    #[test]
    fn test_power_constant_folding() {
        // 2^3 should evaluate to 8
        let expr = Expression::pow(Expression::integer(2), Expression::integer(3));

        assert_eq!(expr, Expression::integer(8), "Constant power should be evaluated");
    }

    #[test]
    fn test_mixed_constant_and_symbolic() {
        // 2 + 3 + x should evaluate to 5 + x
        let x = Expression::symbol("x");
        let expr = Expression::add(vec![
            Expression::integer(2),
            Expression::integer(3),
            x.clone(),
        ]);

        // Should combine constants but keep x separate
        match &expr {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2, "Constants should be combined");
                // One term should be 5, the other should be x
                assert!(terms.contains(&Expression::integer(5)));
                assert!(terms.contains(&x));
            }
            _ => panic!("Expected Add expression with 2 terms"),
        }
    }

    // ========== IDEMPOTENCY TEST ==========

    #[test]
    fn test_constructor_idempotency() {
        // Applying constructor twice should produce same result
        let x = Expression::symbol("x");
        let y = Expression::symbol("y");

        let expr1 = Expression::add(vec![x.clone(), y.clone()]);
        let expr2 = Expression::add(vec![expr1.clone()]);

        assert_eq!(expr1, expr2, "Constructor should be idempotent");
    }

    // ========== LIKE TERMS COMBINING TEST ==========

    #[test]
    fn test_combining_like_terms() {
        // 2x + 3x should evaluate to 5x
        let x = Expression::symbol("x");
        let term1 = Expression::mul(vec![Expression::integer(2), x.clone()]);
        let term2 = Expression::mul(vec![Expression::integer(3), x.clone()]);
        let expr = Expression::add(vec![term1, term2]);

        // Should combine to 5x (which is Mul([5, x]))
        match &expr {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2, "Should combine like terms");
                assert_eq!(factors[0], Expression::integer(5));
                assert_eq!(factors[1], x);
            }
            _ => panic!("Expected Mul expression for 5x"),
        }
    }
}

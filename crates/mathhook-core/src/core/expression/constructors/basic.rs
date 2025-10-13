//! Core expression constructors for basic operations

use crate::core::expression::Expression;
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
    /// use mathhook_core::{Expression, Number};
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
    /// use mathhook_core::{symbol, Expression};
    ///
    /// let expr = Expression::symbol(symbol!(x));
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
    /// use mathhook_core::{Expression, expr};
    ///
    /// // Constant folding
    /// let expression = Expression::add(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    /// ]);
    /// assert_eq!(expression, Expression::integer(3));
    ///
    /// // Identity element removal
    /// let x = expr!(x);
    /// let expression = Expression::add(vec![x.clone(), Expression::integer(0)]);
    /// assert_eq!(expression, x);
    ///
    /// // Commutativity (canonical ordering)
    /// let y = expr!(y);
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
    /// use mathhook_core::{Expression, expr};
    ///
    /// // Constant folding
    /// let expression = Expression::mul(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// ]);
    /// assert_eq!(expression, Expression::integer(6));
    ///
    /// // Identity element removal
    /// let x = expr!(x);
    /// let expr = Expression::mul(vec![x.clone(), Expression::integer(1)]);
    /// assert_eq!(expr, x);
    ///
    /// // Zero handling
    /// let expression = Expression::mul(vec![x.clone(), Expression::integer(0)]);
    /// assert_eq!(expression, Expression::integer(0));
    ///
    /// // Commutativity (canonical ordering)
    /// let y = expr!(y);
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
    /// use mathhook_core::{Expression, expr};
    ///
    /// // Power identities
    /// let x = expr!(x);
    /// let expression = Expression::pow(x.clone(), Expression::integer(1));
    /// assert_eq!(expression, x);
    ///
    /// let expression = Expression::pow(x.clone(), Expression::integer(0));
    /// assert_eq!(expression, Expression::integer(1));
    ///
    /// // Constant evaluation
    /// let expression = expr!(2 ^ 3);
    /// assert_eq!(expression, Expression::integer(8));
    ///
    /// // Nested power flattening
    /// let expression = Expression::pow(
    ///     Expression::pow(x.clone(), Expression::integer(2)),
    ///     Expression::integer(3),
    /// );
    /// // Produces x^6 in canonical form
    /// ```
    pub fn pow(base: Expression, exponent: Expression) -> Self {
        crate::simplify::arithmetic::simplify_power(&base, &exponent)
    }

    /// Create a constant expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, core::MathConstant};
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

    /// Create an equation (equality relation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let expr = Expression::equation(
    ///     expr!(x),
    ///     expr!(5),
    /// );
    /// ```
    pub fn equation(left: Expression, right: Expression) -> Self {
        use crate::core::expression::{RelationData, RelationType};
        Self::Relation(Box::new(RelationData {
            left,
            right,
            relation_type: RelationType::Equal,
        }))
    }

    /// Create a relation expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::core::expression::RelationType;
    ///
    /// let relation = Expression::relation(
    ///     Expression::symbol(symbol!(x)),
    ///     Expression::integer(5),
    ///     RelationType::Greater,
    /// );
    /// ```
    pub fn relation(
        left: Expression,
        right: Expression,
        relation_type: crate::core::expression::RelationType,
    ) -> Self {
        use crate::core::expression::RelationData;
        Self::Relation(Box::new(RelationData {
            left,
            right,
            relation_type,
        }))
    }
}
